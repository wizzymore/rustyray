use std::{ffi::CString, fmt::Debug};

use super::draw::DrawHandler;

use rustyray_sys::{
    consts::{ConfigFlag, MouseButton},
    ffi,
    texture::RenderTexture,
    vector::{Vector2, Vector2i},
};

#[derive(Debug)]
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

            ffi::init_window(
                self.width,
                self.height,
                CString::new(self.title.clone()).unwrap().as_ptr(),
            );
        }

        self
    }

    fn is_window_ready(&self) -> bool {
        unsafe { ffi::is_window_ready() }
    }

    pub fn draw(&self, callback: impl Fn(DrawHandler)) {
        unsafe {
            ffi::begin_drawing();
        }

        callback(DrawHandler {});

        unsafe {
            ffi::end_drawing();
        }
    }

    pub fn draw_render_texture<T: AsRef<RenderTexture>>(
        &self,
        render_texture: T,
        callback: impl Fn(DrawHandler),
    ) {
        unsafe {
            ffi::begin_texture_mode(render_texture.as_ref().clone());
        }

        callback(DrawHandler {});

        unsafe {
            ffi::end_texture_mode();
        }
    }

    pub fn should_close(&self) -> bool {
        unsafe { ffi::window_should_close() }
    }

    pub fn vsync(self, v: bool) -> Self {
        self.set_vsync(v);
        self
    }

    pub fn set_vsync(&self, v: bool) {
        unsafe {
            if v {
                ffi::set_window_state(ConfigFlag::VsyncHint);
            } else {
                ffi::clear_window_state(ConfigFlag::VsyncHint);
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

    pub fn is_mouse_down(&self, button: MouseButton) -> bool {
        unsafe { ffi::is_mouse_button_down(button) }
    }

    pub fn get_mouse_pos(&self) -> Vector2 {
        unsafe { ffi::get_mouse_position() }
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

impl Drop for Window {
    fn drop(&mut self) {
        unsafe {
            ffi::close_window();
        }
    }
}
