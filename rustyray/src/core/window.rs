use std::{ffi::CString, fmt::Debug};

use rustyray_sys::ffi;
use thiserror::Error;

use super::{
    consts::{ConfigFlag, KeyboardKey, MouseButton},
    math::{Vector2, Vector2i},
};

#[derive(Debug)]
pub struct Window {
    width: i32,
    height: i32,
    title: String,
}

#[derive(Error)]
pub enum WindowError {
    #[error("failed to initialize window")]
    WindowNotReady,
    #[error("audio device is already initialized")]
    AudioDeviceAlreadyInitialized,
    #[error("failed to initialize audio device")]
    AudioDeviceFailedInitialize,
}

impl Debug for WindowError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}

// Window-related functions
impl Window {
    pub fn new(width: i32, height: i32, title: String) -> Window {
        Window {
            width,
            height,
            title,
        }
        .init_window()
    }

    fn init_window(self) -> Self {
        unsafe {
            if self.is_window_ready() {
                panic!("You can't create two windows at the same time.");
            }

            ffi::init_window(
                self.width,
                self.height,
                CString::new(self.title.clone()).unwrap().as_ptr(),
            );
        }

        self
    }

    pub fn init_audio(self) -> Result<Self, WindowError> {
        if self.is_audio_device_ready() {
            return Err(WindowError::AudioDeviceAlreadyInitialized);
        }

        unsafe {
            ffi::init_audio_device();
        }

        if !self.is_audio_device_ready() {
            return Err(WindowError::AudioDeviceFailedInitialize);
        }

        Ok(self)
    }

    pub fn is_window_ready(&self) -> bool {
        unsafe { ffi::is_window_ready() }
    }

    pub fn is_audio_device_ready(&self) -> bool {
        unsafe { ffi::is_audio_device_ready() }
    }

    pub fn should_close(&self) -> bool {
        unsafe { ffi::window_should_close() }
    }

    pub fn get_screen_size(&self) -> Vector2i {
        Vector2i {
            x: self.get_screen_width(),
            y: self.get_screen_height(),
        }
    }

    pub fn get_screen_width(&self) -> i32 {
        unsafe { ffi::get_screen_width() }
    }

    pub fn get_screen_height(&self) -> i32 {
        unsafe { ffi::get_screen_height() }
    }

    pub fn dt(&self) -> f32 {
        unsafe { ffi::get_frame_time() }
    }
}

// Configuration-related functions
impl Window {
    pub fn vsync(self, v: bool) -> Self {
        self.set_vsync(v);
        self
    }

    pub fn set_vsync(&self, v: bool) {
        unsafe {
            if v {
                ffi::set_window_state(ConfigFlag::VsyncHint.into());
            } else {
                ffi::clear_window_state(ConfigFlag::VsyncHint.into());
            }
        }
    }

    pub fn fps(self, v: i32) -> Self {
        self.set_fps(v);
        self
    }

    pub fn set_fps(&self, v: i32) {
        unsafe {
            ffi::set_target_fps(v);
        }
    }

    pub fn change_size(width: i32, height: i32) {
        unsafe {
            ffi::set_window_size(width, height);
        }
    }
}

// Input-related functions
impl Window {
    pub fn is_mouse_down(&self, button: MouseButton) -> bool {
        unsafe { ffi::is_mouse_button_down(button) }
    }

    pub fn is_key_down(&self, key: KeyboardKey) -> bool {
        unsafe { ffi::is_key_down(key) }
    }

    pub fn is_key_pressed(&self, key: KeyboardKey) -> bool {
        unsafe { ffi::is_key_pressed(key) }
    }

    pub fn get_mouse_pos(&self) -> Vector2 {
        unsafe { ffi::get_mouse_position().into() }
    }
}

impl Drop for Window {
    fn drop(&mut self) {
        if self.is_audio_device_ready() {
            unsafe {
                ffi::close_audio_device();
            }
        }
        unsafe {
            ffi::close_window();
        }
    }
}
