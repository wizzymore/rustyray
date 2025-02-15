use std::ffi::CString;

use rustyray_ffi::{
    color::Color,
    rectangle::Rectangle,
    texture::{RenderTexture, Texture, TextureLoadError},
    vector::Vector2,
};

/// This is a `raylib` texture that uses the concept of RAII.
///
/// It implements the Drop trait so when it goes out of scope the texture will automatically unload.
///
/// # Examples
/// ```
/// use rustyray::core::draw::OwnedTexture;
/// let texture = OwnedTexture::new(String::from("assets/character.png"));
/// ```
#[repr(C)]
#[derive(Debug)]
pub struct OwnedTexture(pub Texture);

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

impl DrawHandler {
    #[inline]
    pub fn draw_fps(&self, x: i32, y: i32) {
        unsafe {
            rustyray_ffi::ffi::draw_fps(x, y);
        }
    }

    #[inline]
    pub fn clear(&self, color: Color) {
        unsafe {
            rustyray_ffi::ffi::clear_background(color);
        }
    }

    pub fn draw_render_texture(&self, render_texture: &RenderTexture) {
        let size = render_texture.texture.size();
        unsafe {
            let screen_height = rustyray_ffi::ffi::get_screen_height() as f32;
            let screen_width = rustyray_ffi::ffi::get_screen_width() as f32;
            rustyray_ffi::ffi::draw_texture_pro(
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
    pub fn draw_texture(&self, texture: &OwnedTexture, x: i32, y: i32, tint: Color) {
        unsafe {
            rustyray_ffi::ffi::draw_texture(texture.0.clone(), x, y, tint);
        }
    }

    #[inline]
    pub fn draw_rect(&self, rect: Rectangle, tint: Color) {
        unsafe {
            rustyray_ffi::ffi::draw_rectangle_rec(rect, tint);
        }
    }

    #[inline]
    pub fn draw_text(&self, text: String, pos_x: i32, pos_y: i32, size: i32, tint: Color) {
        unsafe {
            rustyray_ffi::ffi::draw_text(
                CString::new(text).unwrap().as_ptr(),
                pos_x,
                pos_y,
                size,
                tint.into(),
            );
        }
    }
}

impl Drop for OwnedTexture {
    fn drop(&mut self) {
        unsafe {
            rustyray_ffi::ffi::unload_texture(self.0.clone());
        }
    }
}

impl Drop for OwnedRenderTexture {
    fn drop(&mut self) {
        unsafe {
            rustyray_ffi::ffi::unload_render_texture(self.0.clone());
        }
    }
}
