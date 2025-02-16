use libc::{c_int, c_uint, c_void};

/// Wave, audio wave data
#[repr(C)]
#[derive(Debug, Clone)]
pub struct Wave {
    frame_count: c_uint,
    sample_rate: c_uint,
    sample_size: c_uint,
    channels: c_uint,
    data: *const c_void,
}

enum RAudioBuffer {}
enum RAudioProcessor {}

pub type AudioCallback = extern "C" fn(buffer_data: *const c_void, frames: c_uint);

/// AudioStream, custom audio stream
#[repr(C)]
#[derive(Debug, Clone)]
pub struct AudioStream {
    /// Pointer to internal data used by the audio system
    buffer: *const RAudioBuffer,
    /// Pointer to internal data processor, useful for audio effects
    processor: *const RAudioProcessor,

    /// Frequency (samples per second)
    sample_rate: c_uint,
    /// Bit detph (bits per sample): 8, 16, 32 (24 not supported)
    sample_size: c_uint,
    /// Number of channels (1-mono, 2-stereo, ...)
    channels: c_uint,
}

/// Sound
#[repr(C)]
#[derive(Debug, Clone)]
pub struct Sound {
    /// Audio Stream
    stream: AudioStream,
    /// Total number of frames (considering channels)
    frame_count: c_uint,
}

#[repr(C)]
#[derive(Debug, Clone)]
pub struct Music {
    stream: AudioStream,
    frame_count: c_uint,
    looping: bool,

    ctx_type: c_int,
    ctx_data: *const c_void,
}
