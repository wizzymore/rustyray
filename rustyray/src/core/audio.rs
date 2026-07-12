use std::ffi::CString;
use std::path::Path;
use thiserror::Error;

use rustyray_sys::{
    audio::Sound as RaySound,
    ffi::{
        self, is_audio_device_ready, is_music_valid, is_sound_valid, is_wave_valid,
        load_music_stream_from_memory, load_sound_alias, load_sound_from_wave,
        load_wave_from_memory, unload_wave,
    },
};

use super::assets::{Asset, AssetLoader, AssetManager, Handle};

fn file_type_from_path(path: &str) -> Option<String> {
    let ext = Path::new(path).extension()?.to_str()?;
    let ext = ext.trim_start_matches('.').to_ascii_lowercase();
    Some(format!(".{ext}"))
}

#[derive(Debug, Error)]
pub enum SoundLoadError {
    #[error("file not found: {0}")]
    FileNotFound(String),
    #[error("audio device is not ready")]
    AudioDeviceNotReady,
    #[error("failed to decode sound")]
    DecodeFailed,
}

#[derive(Debug, Error)]
pub enum MusicLoadError {
    #[error("file not found: {0}")]
    FileNotFound(String),
    #[error("audio device is not ready")]
    AudioDeviceNotReady,
    #[error("failed to decode music")]
    DecodeFailed,
}

#[derive(Debug)]
pub struct Sound {
    inner: RaySound,
    is_alias: bool,
}

#[derive(Debug)]
pub struct Music {
    inner: rustyray_sys::audio::Music,
    paused: bool,
    _buffer: Vec<u8>,
}

impl Sound {
    pub(crate) fn from_wave(wave: rustyray_sys::audio::Wave) -> Result<Self, SoundLoadError> {
        let inner = unsafe { load_sound_from_wave(wave.clone()) };
        unsafe { unload_wave(wave) };
        if !unsafe { is_sound_valid(inner.clone()) } {
            return Err(SoundLoadError::DecodeFailed);
        }
        Ok(Self {
            inner,
            is_alias: false,
        })
    }

    pub(crate) fn from_alias(source: &Sound) -> Self {
        Self {
            inner: unsafe { load_sound_alias(source.inner.clone()) },
            is_alias: true,
        }
    }

    pub fn is_alias(&self) -> bool {
        self.is_alias
    }

    pub fn play(&self) {
        unsafe {
            ffi::play_sound(self.inner.clone());
        }
    }

    pub fn alias(manager: &mut AssetManager, source: &Handle<Sound>) -> Option<Handle<Sound>> {
        let source = manager.get(source)?;
        if source.is_alias() {
            return None;
        }
        Some(manager.insert(Self::from_alias(source)))
    }
}

impl Music {
    pub fn play(&mut self) {
        self.paused = false;
        unsafe {
            ffi::play_music_stream(self.inner.clone());
        }
    }

    #[inline]
    pub fn is_playing(&self) -> bool {
        unsafe { ffi::is_music_stream_playing(self.inner.clone()) }
    }

    #[inline]
    pub fn is_paused(&self) -> bool {
        self.paused
    }

    #[inline]
    pub fn toggle(&mut self) {
        if self.is_paused() {
            self.resume();
        } else {
            self.pause();
        }
    }

    pub fn pause(&mut self) {
        self.paused = true;
        unsafe {
            ffi::pause_music_stream(self.inner.clone());
        }
    }

    pub fn stop(&mut self) {
        self.paused = false;
        unsafe {
            ffi::stop_music_stream(self.inner.clone());
        }
    }

    pub fn resume(&mut self) {
        self.paused = false;
        unsafe {
            ffi::resume_music_stream(self.inner.clone());
        }
    }

    #[inline]
    pub fn restart(&mut self) {
        self.stop();
        self.play();
    }

    #[inline]
    pub fn played(&self) -> f32 {
        unsafe { ffi::get_music_time_played(self.inner.clone()) }
    }

    #[inline]
    pub fn length(&self) -> f32 {
        unsafe { ffi::get_music_time_length(self.inner.clone()) }
    }

    #[inline]
    pub fn update(&self) {
        unsafe {
            ffi::update_music_stream(self.inner.clone());
        }
    }

    #[inline]
    pub fn pitch(&self, pitch: f32) {
        unsafe {
            ffi::set_music_pitch(self.inner.clone(), pitch);
        }
    }

    #[inline]
    pub fn is_looping(&self) -> bool {
        self.inner.looping
    }

    pub fn looping(&mut self, looping: bool) {
        self.inner.looping = looping;
    }
}

impl Drop for Sound {
    fn drop(&mut self) {
        if self.is_alias {
            self.inner.clone().unload_alias();
        } else {
            self.inner.clone().unload();
        }
    }
}

impl Drop for Music {
    fn drop(&mut self) {
        self.inner.clone().unload();
    }
}

impl Asset for Sound {}

impl AssetLoader for Sound {
    type Key = String;
    type Error = SoundLoadError;

    async fn load(path: Self::Key) -> Result<Self, Self::Error> {
        if !unsafe { is_audio_device_ready() } {
            return Err(SoundLoadError::AudioDeviceNotReady);
        }

        let file_type =
            file_type_from_path(&path).ok_or(SoundLoadError::FileNotFound(path.clone()))?;

        let bytes = async_fs::read(&path)
            .await
            .map_err(|_| SoundLoadError::FileNotFound(path.clone()))?;

        let file_type = CString::new(file_type).map_err(|_| SoundLoadError::DecodeFailed)?;
        let wave = unsafe {
            load_wave_from_memory(file_type.as_ptr(), bytes.as_ptr(), bytes.len() as i32)
        };

        if !unsafe { is_wave_valid(wave.clone()) } {
            return Err(SoundLoadError::DecodeFailed);
        }

        Self::from_wave(wave)
    }
}

impl Asset for Music {}

impl AssetLoader for Music {
    type Key = String;
    type Error = MusicLoadError;

    async fn load(path: Self::Key) -> Result<Self, Self::Error> {
        if !unsafe { is_audio_device_ready() } {
            return Err(MusicLoadError::AudioDeviceNotReady);
        }

        let file_type =
            file_type_from_path(&path).ok_or(MusicLoadError::FileNotFound(path.clone()))?;

        let bytes = async_fs::read(&path)
            .await
            .map_err(|_| MusicLoadError::FileNotFound(path.clone()))?;

        let file_type = CString::new(file_type).map_err(|_| MusicLoadError::DecodeFailed)?;
        let inner = unsafe {
            load_music_stream_from_memory(file_type.as_ptr(), bytes.as_ptr(), bytes.len() as i32)
        };

        if !unsafe { is_music_valid(inner.clone()) } {
            return Err(MusicLoadError::DecodeFailed);
        }

        Ok(Self {
            inner,
            paused: false,
            _buffer: bytes,
        })
    }
}
