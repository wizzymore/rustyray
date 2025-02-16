use std::ffi::CString;

use rustyray_sys::ffi;

pub struct DrawHandler;

use super::{
    color::Color,
    image::{OwnedRenderTexture, OwnedTexture},
    math::{Rectangle, Vector2},
    window::Window,
};

impl Window {
    pub fn draw(&self, callback: impl Fn(DrawHandler)) {
        unsafe {
            ffi::begin_drawing();
        }

        callback(DrawHandler {});

        unsafe {
            ffi::end_drawing();
        }
    }

    pub fn draw_render_texture(
        &self,
        render_texture: &OwnedRenderTexture,
        callback: impl Fn(DrawHandler),
    ) {
        unsafe {
            ffi::begin_texture_mode(render_texture.into());
        }

        callback(DrawHandler {});

        unsafe {
            ffi::end_texture_mode();
        }
    }
}

impl DrawHandler {
    #[inline]
    pub fn draw_fps(&self, x: i32, y: i32) {
        unsafe {
            ffi::draw_fps(x, y);
        }
    }

    #[inline]
    pub fn clear(&self, color: Color) {
        unsafe {
            ffi::clear_background(color);
        }
    }

    pub fn draw_render_texture(&self, render_texture: &OwnedRenderTexture) {
        let size = render_texture.size();
        unsafe {
            let screen_height = ffi::get_screen_height() as f32;
            let screen_width = ffi::get_screen_width() as f32;
            ffi::draw_texture_pro(
                render_texture.into(),
                Rectangle::new(0.0, 0.0, size.x as f32, -size.y as f32).into(),
                Rectangle::new(0.0, 0.0, screen_width, screen_height).into(),
                Vector2::zero().into(),
                0.0,
                Color::WHITE,
            );
        }
    }

    #[inline]
    pub fn draw_texture(&self, texture: &OwnedTexture, x: i32, y: i32, tint: Color) {
        unsafe {
            ffi::draw_texture(texture.into(), x, y, tint);
        }
    }

    /// Draw a color-filled rectangle
    #[inline]
    pub fn draw_rect(&self, rect: Rectangle, tint: Color) {
        unsafe {
            ffi::draw_rectangle_rec(rect.into(), tint);
        }
    }

    /// Draw rectangle outline
    #[inline]
    pub fn draw_rect_lines(&self, rect: Rectangle, tint: Color) {
        unsafe {
            ffi::draw_rectangle_lines(
                rect.x as i32,
                rect.y as i32,
                rect.width as i32,
                rect.height as i32,
                tint,
            );
        }
    }

    #[inline]
    pub fn draw_circle(&self, center: Vector2, radius: f32, color: Color) {
        unsafe { ffi::draw_circle_v(center.into(), radius, color) }
    }

    /// Draw text (using default font)
    #[inline]
    pub fn draw_text(&self, text: String, pos_x: i32, pos_y: i32, size: i32, tint: Color) {
        unsafe {
            ffi::draw_text(
                CString::new(text).unwrap().as_ptr(),
                pos_x,
                pos_y,
                size,
                tint,
            );
        }
    }
}
