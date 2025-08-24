use std::{ffi::CString, fmt::Debug};

use libc::{c_int, c_uint, c_void, uintptr_t};

use crate::ffi;

/// Wave, audio wave data
#[repr(C)]
#[derive(Debug, Clone)]
pub struct Wave {
    pub frame_count: c_uint,
    pub sample_rate: c_uint,
    pub sample_size: c_uint,
    pub channels: c_uint,
    pub data: *const c_void,
}

pub type AudioCallback = extern "C" fn(buffer_data: *const c_void, frames: c_uint);

/// AudioStream, custom audio stream
#[repr(C)]
#[derive(Debug, Clone)]
pub struct AudioStream {
    /// Pointer to internal data used by the audio system
    pub buffer: uintptr_t,
    /// Pointer to internal data processor, useful for audio effects
    pub processor: uintptr_t,

    /// Frequency (samples per second)
    pub sample_rate: c_uint,
    /// Bit detph (bits per sample): 8, 16, 32 (24 not supported)
    pub sample_size: c_uint,
    /// Number of channels (1-mono, 2-stereo, ...)
    pub channels: c_uint,
}

/// Sound
#[repr(C)]
#[derive(Debug, Clone)]
pub struct Sound {
    /// Audio Stream
    pub stream: AudioStream,
    /// Total number of frames (considering channels)
    pub frame_count: c_uint,
}

impl Sound {
    pub fn new(path: String) -> Self {
        unsafe { ffi::load_sound(CString::new(path).unwrap().as_ptr()) }
    }

    pub fn unload(self) {
        unsafe {
            ffi::unload_sound(self);
        }
    }

    pub fn unload_alias(self) {
        unsafe {
            ffi::unload_sound_alias(self);
        }
    }
}

#[repr(C)]
#[derive(Debug, Clone)]
pub struct Music {
    pub stream: AudioStream,
    pub frame_count: c_uint,
    pub looping: bool,

    pub ctx_type: c_int,
    pub ctx_data: uintptr_t,
}

impl Music {
    pub fn new(path: String) -> Self {
        unsafe { ffi::load_music_stream(CString::new(path).unwrap().as_ptr()) }
    }

    pub fn unload(self) {
        unsafe {
            ffi::unload_music_stream(self);
        }
    }
}
