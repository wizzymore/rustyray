use std::{ffi::CString, fmt::Debug};

use super::{
    draw::{DrawHandler, RenderTexture},
    math::{Vector2, Vector2i},
    ConfigFlags, MouseButton,
};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct Window {
    width: i32,
    height: i32,
    title: String,
}

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

            rustyray_ffi::InitWindow(
                self.width,
                self.height,
                CString::new(self.title.clone()).unwrap().as_ptr(),
            );
        }

        self
    }

    fn is_window_ready(&self) -> bool {
        unsafe { rustyray_ffi::IsWindowReady() }
    }

    pub fn draw(&mut self, callback: impl Fn(DrawHandler)) {
        unsafe {
            rustyray_ffi::BeginDrawing();
        }

        callback(DrawHandler {});

        unsafe {
            rustyray_ffi::EndDrawing();
        }
    }

    pub fn draw_render_texture(
        &mut self,
        render_texture: &RenderTexture,
        callback: impl Fn(DrawHandler),
    ) {
        unsafe {
            rustyray_ffi::BeginTextureMode(render_texture.into());
        }

        callback(DrawHandler {});

        unsafe {
            rustyray_ffi::EndTextureMode();
        }
    }

    pub fn should_close(&self) -> bool {
        unsafe { rustyray_ffi::WindowShouldClose() }
    }

    pub fn vsync(self, v: bool) -> Self {
        self.set_vsync(v);
        self
    }

    pub fn set_vsync(&self, v: bool) {
        unsafe {
            if v {
                rustyray_ffi::SetWindowState(ConfigFlags::FlagVsyncHint.into());
            } else {
                rustyray_ffi::ClearWindowState(ConfigFlags::FlagVsyncHint.into());
            }
        }
    }

    pub fn fps(self, v: i32) -> Self {
        self.set_fps(v);
        self
    }

    pub fn set_fps(&self, v: i32) {
        unsafe {
            rustyray_ffi::SetTargetFPS(v);
        }
    }

    pub fn change_size(width: i32, height: i32) {
        unsafe {
            rustyray_ffi::SetWindowSize(width, height);
        }
    }

    pub fn is_mouse_down(&self, mb: MouseButton) -> bool {
        unsafe { rustyray_ffi::IsMouseButtonDown(mb as i32) }
    }

    pub fn get_mouse_pos(&self) -> Vector2 {
        unsafe { rustyray_ffi::GetMousePosition().into() }
    }

    pub fn get_screen_size(&self) -> Vector2i {
        Vector2i {
            x: self.get_screen_width(),
            y: self.get_screen_height(),
        }
    }

    pub fn get_screen_width(&self) -> i32 {
        unsafe { rustyray_ffi::GetScreenWidth() }
    }

    pub fn get_screen_height(&self) -> i32 {
        unsafe { rustyray_ffi::GetScreenHeight() }
    }

    pub fn dt(&self) -> f32 {
        unsafe { rustyray_ffi::GetFrameTime() }
    }
}

impl Drop for Window {
    fn drop(&mut self) {
        unsafe {
            rustyray_ffi::CloseWindow();
        }
    }
}
