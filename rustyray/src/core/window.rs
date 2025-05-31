use std::{ffi::CString, fmt::Debug};

use rustyray_sys::ffi;
use thiserror::Error;

use super::{
    consts::{ConfigFlag, KeyboardKey, MouseButton},
    math::{Vector2, Vector2i},
};

#[derive(Debug)]
pub struct Window;

#[derive(Debug, Error)]
pub enum WindowError {
    #[error("failed to initialize window")]
    WindowNotReady,
    #[error("you can't initialize two windows at the same time")]
    DoubleWindowInit,
    #[error("audio device is already initialized")]
    AudioDeviceAlreadyInitialized,
    #[error("failed to initialize audio device")]
    AudioDeviceFailedInitialize,
}

#[derive(Debug)]
pub struct WindowBuilder {
    width: i32,
    height: i32,
    title: String,
    flags: ConfigFlag,
    fps: Option<i32>,
    audio: bool,
}

impl WindowBuilder {
    fn new(width: i32, height: i32, title: String) -> Self {
        WindowBuilder {
            width,
            height,
            title,
            flags: ConfigFlag::none(),
            fps: None,
            audio: false,
        }
    }

    pub fn set_fps(mut self, fps: i32) -> Self {
        self.fps = Some(fps);
        self
    }

    pub fn set_config_flags(mut self, flags: ConfigFlag) -> Self {
        self.flags = flags;
        unsafe {
            ffi::set_config_flags(flags.into());
        }
        self
    }

    pub fn init_audio(mut self) -> Self {
        self.audio = true;
        self
    }

    pub fn build(&self) -> Result<Window, WindowError> {
        unsafe {
            if ffi::is_window_ready() {
                return Err(WindowError::DoubleWindowInit);
            }

            ffi::init_window(
                self.width,
                self.height,
                CString::new(self.title.as_str()).unwrap().as_ptr(),
            );

            if !ffi::is_window_ready() {
                return Err(WindowError::WindowNotReady);
            }
        }

        let window = Window {};

        if let Some(fps) = self.fps {
            window.set_fps(fps);
        }
        if self.audio {
            window.init_audio()?;
        }

        Ok(window)
    }
}

// Window-related functions
impl Window {
    pub fn new(width: i32, height: i32, title: String) -> WindowBuilder {
        WindowBuilder::new(width, height, title)
    }

    pub fn init_audio(&self) -> Result<(), WindowError> {
        if self.is_audio_device_ready() {
            return Err(WindowError::AudioDeviceAlreadyInitialized);
        }

        unsafe {
            ffi::init_audio_device();
        }

        if !self.is_audio_device_ready() {
            return Err(WindowError::AudioDeviceFailedInitialize);
        }

        Ok(())
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

    // Configuration-related functions
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

    pub fn fps(self, fps: i32) -> Self {
        self.set_fps(fps);
        self
    }

    pub fn set_fps(&self, fps: i32) {
        unsafe {
            ffi::set_target_fps(fps);
        }
    }

    pub fn change_size(width: i32, height: i32) {
        unsafe {
            ffi::set_window_size(width, height);
        }
    }

    // Input-related functions
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
        unsafe {
            if self.is_audio_device_ready() {
                ffi::close_audio_device();
            }
            ffi::close_window();
        }
    }
}
