use std::{ffi::CString, fmt::Debug};

use rustyray_sys::{ffi, texture::Texture};
use thiserror::Error;

#[derive(Debug, Clone, Copy)]
pub struct Camera2D {
    pub offset: Vector2, // Camera offset (displacement from target)
    pub target: Vector2, // Camera target (rotation and zoom origin)
    pub rotation: f32,   // Camera rotation in degrees
    pub zoom: f32,       // Camera zoom (scaling), should be 1.0f by default
}

impl Into<rustyray_sys::camera::Camera2D> for Camera2D {
    fn into(self) -> rustyray_sys::camera::Camera2D {
        unsafe { std::mem::transmute(self) }
    }
}

impl Into<Camera2D> for rustyray_sys::camera::Camera2D {
    fn into(self) -> Camera2D {
        unsafe { std::mem::transmute(self) }
    }
}

impl Default for Camera2D {
    fn default() -> Self {
        Self {
            offset: Vector2 { x: 0.0, y: 0.0 },
            target: Vector2 { x: 0.0, y: 0.0 },
            rotation: 0.0,
            zoom: 1.0,
        }
    }
}

#[derive(Error, Debug)]
pub enum DrawError {
    #[error("You must clear the screen everytime when you call draw, otherwise you will have a memory leak.")]
    DidNotClear,
}

#[derive(Default)]
pub struct DrawHandler {
    pub(crate) did_clear: bool,
}

pub struct Camera2DDrawHandler {
    pub did_end: bool,
}

impl Camera2DDrawHandler {
    pub fn new(camera: Camera2D) -> Self {
        unsafe {
            ffi::begin_mode_2d(camera.into());
        }
        Camera2DDrawHandler { did_end: false }
    }

    pub fn end(&mut self) {
        if !self.did_end {
            self.did_end = true;
            unsafe {
                ffi::end_mode_2d();
            }
        }
    }
}

impl Drop for Camera2DDrawHandler {
    fn drop(&mut self) {
        self.end();
    }
}

use super::{
    color::Color,
    image::{OwnedRenderTexture, OwnedTexture},
    math::{Rectangle, Vector2},
    window::Window,
};

impl Window {
    pub fn draw(&self, callback: impl FnOnce(&mut DrawHandler)) -> Result<(), DrawError> {
        unsafe {
            ffi::begin_drawing();
        }

        let mut draw_handler = DrawHandler {
            ..Default::default()
        };

        callback(&mut draw_handler);

        if !draw_handler.did_clear {
            return Err(DrawError::DidNotClear);
        }

        unsafe {
            ffi::end_drawing();
        }

        Ok(())
    }

    pub fn draw_render_texture(
        &self,
        render_texture: &OwnedRenderTexture,
        callback: impl Fn(&mut DrawHandler),
    ) {
        unsafe {
            ffi::begin_texture_mode(render_texture.into());
        }

        callback(&mut DrawHandler {
            ..Default::default()
        });

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
    pub fn clear(&mut self, color: Color) {
        self.did_clear = true;
        unsafe {
            ffi::clear_background(color);
        }
    }

    pub fn draw_render_texture(&self, render_texture: &OwnedRenderTexture) {
        let size = render_texture.size();
        unsafe {
            ffi::draw_texture_pro(
                render_texture.into(),
                Rectangle::new(0.0, 0.0, size.x as f32, -size.y as f32).into(),
                Rectangle::new(0.0, 0.0, size.x as f32, size.y as f32).into(),
                Vector2::ZERO.into(),
                0.0,
                Color::WHITE,
            );
        }
    }

    #[inline]
    pub fn draw_texture<T>(&self, texture: T, x: i32, y: i32, tint: Color)
    where
        T: Into<Texture>,
    {
        unsafe {
            ffi::draw_texture(texture.into(), x, y, tint);
        }
    }

    #[inline]
    pub fn draw_texture_pro<T>(
        &self,
        texture: T,
        source: Rectangle,
        dest: Rectangle,
        origin: Vector2,
        rotation: f32,
        tint: Color,
    ) where
        T: Into<Texture>,
    {
        unsafe {
            ffi::draw_texture_pro(
                texture.into(),
                source.into(),
                dest.into(),
                origin.into(),
                rotation,
                tint,
            );
        }
    }

    /// Draw a color-filled rectangle
    #[inline]
    pub fn draw_rect(&self, rect: Rectangle, tint: Color) {
        unsafe {
            ffi::draw_rectangle_rec(rect.into(), tint);
        }
    }

    #[inline]
    pub fn draw_rect_pro(&self, rect: Rectangle, origin: Vector2, rotation: f32, tint: Color) {
        unsafe {
            ffi::draw_rectangle_pro(rect.into(), origin.into(), rotation, tint);
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
    pub fn draw_rect_lines_ex(&self, rect: Rectangle, line_thickness: f32, tint: Color) {
        unsafe {
            ffi::draw_rectangle_lines_ex(rect.into(), line_thickness, tint);
        }
    }

    #[inline]
    pub fn draw_circle(&self, center: Vector2, radius: f32, color: Color) {
        unsafe { ffi::draw_circle_v(center.into(), radius, color) }
    }

    /// Draw text (using default font)
    #[inline]
    pub fn draw_text(&self, text: &str, pos_x: i32, pos_y: i32, size: i32, tint: Color) {
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

    #[inline]
    #[must_use]
    pub fn begin_mode_2d(&self, camera: Camera2D) -> Camera2DDrawHandler {
        Camera2DDrawHandler::new(camera)
    }
}
