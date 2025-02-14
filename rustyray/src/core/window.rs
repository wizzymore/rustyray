use std::{
    error::Error,
    fmt::{Debug, Display},
};

use rustyray_sys::ConfigFlags;

use super::draw::{DrawHandler, RenderTexture};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct Window {
    width: i32,
    height: i32,
    title: String,
}

pub struct DrawAlreadyStarted;

impl Display for DrawAlreadyStarted {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("Window's begin_draw function called twice in the same scope.")
    }
}

impl Debug for DrawAlreadyStarted {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("Window's begin_draw function called twice in the same scope.")
    }
}

impl Error for DrawAlreadyStarted {}

impl Window {
    pub fn new(width: i32, height: i32, title: String) -> Window {
        let mut window = Window {
            width: width,
            height: height,
            title: title,
        };

        window.init_window();

        window
    }

    fn init_window(&mut self) {
        unsafe {
            if self.is_window_ready() {
                panic!("You can't create two windows at the same time.");
            }

            rustyray_sys::InitWindow(self.width, self.height, self.title.as_ptr() as *const i8);
        }
    }

    fn is_window_ready(&self) -> bool {
        unsafe { rustyray_sys::IsWindowReady() }
    }

    pub fn draw(&mut self, callback: impl Fn(DrawHandler)) {
        unsafe {
            rustyray_sys::BeginDrawing();
        }

        callback(DrawHandler {});

        unsafe {
            rustyray_sys::EndDrawing();
        }
    }

    pub fn draw_render_texture(
        &mut self,
        render_texture: &RenderTexture,
        callback: impl Fn(DrawHandler),
    ) {
        unsafe {
            rustyray_sys::BeginTextureMode(*(&render_texture.0));
        }

        callback(DrawHandler {});

        unsafe {
            rustyray_sys::EndTextureMode();
        }
    }

    pub fn should_close(&self) -> bool {
        unsafe { rustyray_sys::WindowShouldClose() }
    }

    pub fn vsync(self, v: bool) -> Self {
        self.set_vsync(v);
        self
    }

    pub fn change_size(width: i32, height: i32) {
        unsafe {
            rustyray_sys::SetWindowSize(width, height);
        }
    }

    pub fn set_vsync(&self, v: bool) {
        unsafe {
            if v {
                rustyray_sys::SetWindowState(ConfigFlags::FLAG_VSYNC_HINT as u32);
            } else {
                rustyray_sys::ClearWindowState(ConfigFlags::FLAG_VSYNC_HINT as u32);
            }
        }
    }
}

impl Drop for Window {
    fn drop(&mut self) {
        unsafe {
            rustyray_sys::CloseWindow();
        }
    }
}
