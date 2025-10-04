use std::{ffi::CString, fmt::Debug};

use rustyray_sys::ffi;
use thiserror::Error;

use crate::prelude::{DrawingExt, GamepadAxis, GamepadButton, TextureModeExt};

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
    pub fn new(width: i32, height: i32, title: impl Into<String>) -> Self {
        WindowBuilder {
            width,
            height,
            title: title.into(),
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
        let mut window = Window {};

        if window.is_ready() {
            return Err(WindowError::DoubleWindowInit);
        }

        unsafe {
            ffi::init_window(
                self.width,
                self.height,
                CString::new(self.title.as_str()).unwrap().as_ptr(),
            );
        }

        if !window.is_ready() {
            return Err(WindowError::WindowNotReady);
        }

        if let Some(fps) = self.fps {
            window.set_target_fps(fps);
        }
        if self.audio {
            window.init_audio()?;
        }

        Ok(window)
    }
}

// Window-related functions
impl Window {
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

    #[inline]
    pub fn is_audio_device_ready(&self) -> bool {
        unsafe { ffi::is_audio_device_ready() }
    }

    #[inline]
    pub fn is_ready(&self) -> bool {
        unsafe { ffi::is_window_ready() }
    }

    #[inline]
    pub fn is_fullscreen(&self) -> bool {
        unsafe { ffi::is_window_fullscreen() }
    }

    #[inline]
    pub fn is_hidden(&self) -> bool {
        unsafe { ffi::is_window_hidden() }
    }

    #[inline]
    pub fn is_minimized(&self) -> bool {
        unsafe { ffi::is_window_minimized() }
    }

    #[inline]
    pub fn is_maximized(&self) -> bool {
        unsafe { ffi::is_window_maximized() }
    }

    #[inline]
    pub fn is_focused(&self) -> bool {
        unsafe { ffi::is_window_focused() }
    }

    #[inline]
    pub fn is_resized(&self) -> bool {
        unsafe { ffi::is_window_resized() }
    }

    #[inline]
    pub fn should_close(&self) -> bool {
        unsafe { ffi::window_should_close() }
    }

    pub fn screen_size(&self) -> Vector2i {
        Vector2i {
            x: self.screen_width(),
            y: self.screen_height(),
        }
    }

    #[inline]
    pub fn screen_width(&self) -> i32 {
        unsafe { ffi::get_screen_width() }
    }

    #[inline]
    pub fn screen_height(&self) -> i32 {
        unsafe { ffi::get_screen_height() }
    }

    pub fn render_size(&self) -> Vector2i {
        Vector2i {
            x: self.render_width(),
            y: self.render_height(),
        }
    }

    #[inline]
    pub fn render_width(&self) -> i32 {
        unsafe { ffi::get_render_width() }
    }

    #[inline]
    pub fn render_height(&self) -> i32 {
        unsafe { ffi::get_render_height() }
    }

    #[inline]
    pub fn monitor_count(&self) -> i32 {
        unsafe { ffi::get_monitor_count() }
    }

    #[inline]
    pub fn current_monitor(&self) -> i32 {
        unsafe { ffi::get_current_monitor() }
    }

    #[inline]
    pub fn monitor_position(&self, monitor: i32) -> Vector2 {
        unsafe { ffi::get_monitor_position(monitor).into() }
    }

    #[inline]
    pub fn monitor_width(&self, monitor: i32) -> i32 {
        unsafe { ffi::get_monitor_width(monitor) }
    }

    #[inline]
    pub fn monitor_height(&self, monitor: i32) -> i32 {
        unsafe { ffi::get_monitor_height(monitor) }
    }

    #[inline]
    pub fn monitor_physical_width(&self, monitor: i32) -> i32 {
        unsafe { ffi::get_monitor_physical_width(monitor) }
    }

    #[inline]
    pub fn monitor_physical_height(&self, monitor: i32) -> i32 {
        unsafe { ffi::get_monitor_physical_height(monitor) }
    }

    #[inline]
    pub fn monitor_refresh_rate(&self, monitor: i32) -> i32 {
        unsafe { ffi::get_monitor_refresh_rate(monitor) }
    }

    #[inline]
    pub fn position(&self) -> Vector2 {
        unsafe { ffi::get_window_position().into() }
    }

    #[inline]
    pub fn scale_dpi(&self) -> Vector2 {
        unsafe { ffi::get_window_scale_dpi().into() }
    }

    #[inline]
    pub fn monitor_name(&self, monitor: i32) -> CString {
        unsafe { CString::from_raw(ffi::get_monitor_name(monitor).cast_mut()) }
    }

    // Configuration-related functions
    pub fn vsync(self) -> Self {
        self.set_vsync(true);
        self
    }

    pub fn set_vsync(&self, v: bool) {
        if v {
            self.set_state(ConfigFlag::VsyncHint);
        } else {
            self.clear_state(ConfigFlag::VsyncHint);
        }
    }

    #[inline]
    pub fn toggle_fullscreen(&self) {
        unsafe {
            ffi::toggle_fullscreen();
        }
    }

    #[inline]
    pub fn toggle_borderless_windowed(&self) {
        unsafe {
            ffi::toggle_borderless_windowed();
        }
    }

    #[inline]
    pub fn maximize_window(&self) {
        unsafe {
            ffi::maximize_window();
        }
    }

    #[inline]
    pub fn minimize_window(&self) {
        unsafe {
            ffi::minimize_window();
        }
    }

    #[inline]
    pub fn restore_window(&self) {
        unsafe {
            ffi::restore_window();
        }
    }

    #[inline]
    pub fn is_state(&self, state: ConfigFlag) -> bool {
        unsafe { ffi::is_window_state(state.into()) }
    }

    #[inline]
    pub fn set_state(&self, state: ConfigFlag) {
        unsafe {
            ffi::set_window_state(state.into());
        }
    }

    #[inline]
    pub fn set_title(&self, title: CString) {
        unsafe {
            ffi::set_window_title(title.as_ptr());
        }
    }

    #[inline]
    pub fn set_position(&self, x: i32, y: i32) {
        unsafe {
            ffi::set_window_position(x, y);
        }
    }

    #[inline]
    pub fn set_monitor(&self, monitor: i32) {
        unsafe {
            ffi::set_window_monitor(monitor);
        }
    }

    #[inline]
    pub fn set_min_size(&self, width: i32, height: i32) {
        unsafe {
            ffi::set_window_min_size(width, height);
        }
    }

    #[inline]
    pub fn set_max_size(&self, width: i32, height: i32) {
        unsafe {
            ffi::set_window_max_size(width, height);
        }
    }

    #[inline]
    pub fn set_size(&self, width: i32, height: i32) {
        unsafe {
            ffi::set_window_size(width, height);
        }
    }

    #[inline]
    pub fn set_opacity(&self, opacity: f32) {
        unsafe {
            ffi::set_window_opacity(opacity);
        }
    }

    #[inline]
    pub fn set_focused(&self) {
        unsafe {
            ffi::set_window_focused();
        }
    }

    #[inline]
    pub fn clear_state(&self, state: ConfigFlag) {
        unsafe {
            ffi::clear_window_state(state.into());
        }
    }

    #[inline]
    pub fn set_target_fps(&mut self, fps: i32) {
        unsafe {
            ffi::set_target_fps(fps);
        }
    }

    #[inline]
    pub fn frame_time(&self) -> f32 {
        unsafe { ffi::get_frame_time() }
    }

    #[inline]
    pub fn time(&self) -> f64 {
        unsafe { ffi::get_time() }
    }

    #[inline]
    pub fn window_size(&mut self, width: i32, height: i32) {
        unsafe {
            ffi::set_window_size(width, height);
        }
    }

    // Input-related functions
    #[inline]
    pub fn is_mouse_down(&self, button: MouseButton) -> bool {
        unsafe { ffi::is_mouse_button_down(button) }
    }

    #[inline]
    /// Check if a key is being pressed
    pub fn is_key_down(&self, key: KeyboardKey) -> bool {
        unsafe { ffi::is_key_down(key) }
    }

    #[inline]
    /// Check if a key has been pressed once
    pub fn is_key_pressed(&self, key: KeyboardKey) -> bool {
        unsafe { ffi::is_key_pressed(key) }
    }

    #[inline]
    pub fn mouse_pos(&self) -> Vector2 {
        unsafe { ffi::get_mouse_position().into() }
    }

    #[inline]
    pub fn mouse_wheel_move(&self) -> f32 {
        unsafe { ffi::get_mouse_wheel_move() }
    }

    #[inline]
    pub fn show_cursor(&self) {
        unsafe {
            ffi::show_cursor();
        }
    }

    #[inline]
    pub fn hide_cursor(&self) {
        unsafe {
            ffi::hide_cursor();
        }
    }

    #[inline]
    pub fn is_cursor_hidden(&self) -> bool {
        unsafe { ffi::is_cursor_hidden() }
    }

    #[inline]
    pub fn enable_cursor(&self) {
        unsafe {
            ffi::enable_cursor();
        }
    }

    #[inline]
    pub fn disable_cursor(&self) {
        unsafe {
            ffi::disable_cursor();
        }
    }

    #[inline]
    pub fn is_cursor_on_screen(&self) -> bool {
        unsafe { ffi::is_cursor_on_screen() }
    }

    #[inline]
    pub fn set_exit_key(&self, key: KeyboardKey) {
        unsafe { ffi::set_exit_key(key) }
    }

    #[inline]
    pub fn is_gamepad_button_pressed(&self, gamepad: i32, button: GamepadButton) -> bool {
        unsafe { ffi::is_gamepad_button_pressed(gamepad, button) }
    }

    #[inline]
    pub fn is_gamepad_button_down(&self, gamepad: i32, button: GamepadButton) -> bool {
        unsafe { ffi::is_gamepad_button_down(gamepad, button) }
    }

    #[inline]
    pub fn is_gamepad_button_released(&self, gamepad: i32, button: GamepadButton) -> bool {
        unsafe { ffi::is_gamepad_button_released(gamepad, button) }
    }

    #[inline]
    pub fn is_gamepad_button_up(&self, gamepad: i32, button: GamepadButton) -> bool {
        unsafe { ffi::is_gamepad_button_up(gamepad, button) }
    }

    #[inline]
    pub fn gamepad_button_pressed(&self, gamepad: i32) -> GamepadButton {
        unsafe { ffi::get_gamepad_button_pressed(gamepad) }
    }

    #[inline]
    pub fn gamepad_axis_count(&self, gamepad: i32) -> i32 {
        unsafe { ffi::get_gamepad_axis_count(gamepad) }
    }

    #[inline]
    pub fn gamepad_axis_movement(&self, gamepad: i32, axis: GamepadAxis) -> f32 {
        unsafe { ffi::get_gamepad_axis_movement(gamepad, axis) }
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

impl DrawingExt for Window {}
impl TextureModeExt for Window {}
