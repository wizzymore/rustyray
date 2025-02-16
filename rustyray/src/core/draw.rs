use std::ffi::CString;

use rustyray_sys::{
    color::Color,
    ffi,
    rectangle::Rectangle,
    texture::{RenderTexture, Texture, TextureLoadError},
    vector::Vector2,
};

use super::window::Window;

/// This is a [`rustyray_ffi::texture::Texture`] that uses the concept of RAII.
///
/// It implements the Drop trait so when it goes out of scope the texture will automatically unload.
///
/// # Examples
/// ```no_run
/// use rustyray::prelude::OwnedTexture;
///
/// let texture = OwnedTexture::new(String::from("assets/character.png"));
/// ```
#[repr(C)]
#[derive(Debug)]
pub struct OwnedTexture(pub Texture);

/// This is a `raylib` [`rustyray_ffi::texture::RenderTexture`] that uses the concept of RAII.
///
/// It implements the Drop trait so when it goes out of scope the texture will automatically unload.
///
/// # Examples
/// ```no_run
/// use rustyray::prelude::OwnedTexture;
///
/// let texture = OwnedTexture::new(String::from("assets/character.png"));
/// ```
#[repr(C)]
#[derive(Debug)]
pub struct OwnedRenderTexture(pub RenderTexture);

pub struct DrawHandler;

impl OwnedTexture {
    #[inline]
    pub fn new(path: String) -> Result<Self, TextureLoadError> {
        Ok(Self(Texture::new(path)?))
    }
}

impl OwnedRenderTexture {
    #[inline]
    pub fn new(width: i32, height: i32) -> Self {
        Self(RenderTexture::new(width, height))
    }
}

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

    pub fn draw_render_texture<T: AsRef<RenderTexture>>(&self, render_texture_ref: T) {
        let render_texture = render_texture_ref.as_ref();
        let size = render_texture.texture.size();
        unsafe {
            let screen_height = ffi::get_screen_height() as f32;
            let screen_width = ffi::get_screen_width() as f32;
            ffi::draw_texture_pro(
                render_texture.texture.clone(),
                Rectangle::new(0.0, 0.0, size.x as f32, -size.y as f32),
                Rectangle::new(0.0, 0.0, screen_width, screen_height),
                Vector2::zero(),
                0.0,
                Color::WHITE,
            );
        }
    }

    #[inline]
    pub fn draw_texture<T: AsRef<Texture>>(&self, texture: T, x: i32, y: i32, tint: Color) {
        unsafe {
            ffi::draw_texture(texture.as_ref().clone(), x, y, tint);
        }
    }

    /// Draw a color-filled rectangle
    #[inline]
    pub fn draw_rect(&self, rect: Rectangle, tint: Color) {
        unsafe {
            ffi::draw_rectangle_rec(rect, tint);
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
        unsafe { ffi::draw_circle_v(center, radius, color) }
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

impl Drop for OwnedTexture {
    fn drop(&mut self) {
        unsafe {
            ffi::unload_texture(self.0.clone());
        }
    }
}

impl Drop for OwnedRenderTexture {
    fn drop(&mut self) {
        unsafe {
            ffi::unload_render_texture(self.0.clone());
        }
    }
}

impl AsRef<Texture> for OwnedTexture {
    fn as_ref(&self) -> &Texture {
        &self.0
    }
}

impl AsRef<RenderTexture> for OwnedRenderTexture {
    fn as_ref(&self) -> &RenderTexture {
        &self.0
    }
}
