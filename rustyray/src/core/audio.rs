use rustyray_sys::{
    audio::{Music, Sound},
    ffi,
};

/// RAII Implementation of [Sound]
///
/// Second parameter of the tuple represents if it is an alias or not.
/// It will be set as true if you create a sound by calling [Self::alias], false otherwise
///
/// To create a sound alias you must first own an [OwnedSound]
#[derive(Debug)]
pub struct OwnedSound(Sound, bool);

type OwnedSoundAlias = OwnedSound;

impl OwnedSound {
    /// Load a RAII implementation sound from file
    pub fn new(path: String) -> Self {
        Self(Sound::new(path), false)
    }

    /// Play a sound
    pub fn play(&self) {
        unsafe {
            ffi::play_sound(self.0.clone());
        }
    }

    pub fn is_alias(&self) -> bool {
        self.1
    }

    /// Create a new sound that shares the same sample data as the source sound, does not own the sound data
    pub fn alias(&self) -> OwnedSoundAlias {
        unsafe { OwnedSound(ffi::load_sound_alias(self.0.clone()), true) }
    }
}

impl From<Sound> for OwnedSound {
    fn from(value: Sound) -> Self {
        OwnedSound(value, false)
    }
}

impl From<OwnedSound> for Sound {
    fn from(val: OwnedSound) -> Self {
        val.0.clone()
    }
}

impl AsRef<Sound> for OwnedSound {
    fn as_ref(&self) -> &Sound {
        &self.0
    }
}

impl Drop for OwnedSound {
    fn drop(&mut self) {
        match self.1 {
            true => self.0.to_owned().unload_alias(),
            false => self.0.to_owned().unload(),
        }
    }
}

/// RAII Implementation of [Music]
pub struct OwnedMusic {
    music: Music,
    paused: bool,
}

impl OwnedMusic {
    pub fn new(path: String) -> Self {
        Self {
            music: Music::new(path),
            paused: false,
        }
    }

    /// Start music playing
    pub fn play(&mut self) {
        self.paused = false;
        unsafe {
            ffi::play_music_stream(self.into());
        }
    }

    #[inline]
    pub fn is_playing(&self) -> bool {
        unsafe { ffi::is_music_stream_playing(self.music.to_owned()) }
    }

    #[inline]
    pub fn is_paused(&self) -> bool {
        self.paused
    }

    /// Toggle the pause state of the [Music]
    #[inline]
    pub fn toggle(&mut self) {
        if self.is_paused() {
            self.resume();
        } else {
            self.pause();
        }
    }

    /// Pause music playing
    pub fn pause(&mut self) {
        self.paused = true;
        unsafe {
            ffi::pause_music_stream(self.music.to_owned());
        }
    }

    /// Stop music playing
    pub fn stop(&mut self) {
        self.paused = false;
        unsafe {
            ffi::stop_music_stream(self.music.to_owned());
        }
    }

    /// Resume playing paused music
    pub fn resume(&mut self) {
        self.paused = false;
        unsafe {
            ffi::resume_music_stream(self.music.to_owned());
        }
    }

    #[inline]
    pub fn restart(&mut self) {
        self.stop();
        self.play();
    }

    /// Get current music time played (in seconds)
    #[inline]
    pub fn played(&self) -> f32 {
        unsafe { ffi::get_music_time_played(self.music.to_owned()) }
    }

    /// Get music time length (in seconds)
    #[inline]
    pub fn length(&self) -> f32 {
        unsafe { ffi::get_music_time_length(self.music.to_owned()) }
    }

    /// Updates buffers for music streaming
    #[inline]
    pub fn update(&self) {
        unsafe {
            ffi::update_music_stream(self.music.to_owned());
        }
    }

    /// Set pitch for music (1.0 is base level)
    #[inline]
    pub fn pitch(&self, pitch: f32) {
        unsafe {
            ffi::set_music_pitch(self.music.to_owned(), pitch);
        }
    }

    #[inline]
    pub fn is_looping(&self) -> bool {
        self.music.looping
    }

    #[inline]
    pub fn looping(&mut self, looping: bool) {
        self.music.looping = looping;
    }
}

impl From<Music> for OwnedMusic {
    fn from(music: Music) -> Self {
        OwnedMusic {
            music,
            paused: false,
        }
    }
}

impl From<OwnedMusic> for Music {
    fn from(val: OwnedMusic) -> Self {
        val.music.clone()
    }
}

impl From<&OwnedMusic> for Music {
    fn from(val: &OwnedMusic) -> Self {
        val.music.clone()
    }
}

impl From<&mut OwnedMusic> for Music {
    fn from(val: &mut OwnedMusic) -> Self {
        val.music.clone()
    }
}

impl AsRef<Music> for OwnedMusic {
    fn as_ref(&self) -> &Music {
        &self.music
    }
}

impl Drop for OwnedMusic {
    fn drop(&mut self) {
        self.music.to_owned().unload()
    }
}
