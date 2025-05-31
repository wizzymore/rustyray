use std::{
    ffi::{CStr, CString},
    str::FromStr,
};

use rustyray_sys::{
    ffi::{self, load_image_from_memory, load_texture_from_image},
    texture::{self, Image, RenderTexture, RenderTextureLoadError, Texture, TextureLoadError},
};

use super::math::Vector2i;
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
pub struct OwnedTexture(Texture);

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
pub struct OwnedRenderTexture(RenderTexture);

impl OwnedTexture {
    #[inline]
    pub fn new(path: String) -> Result<Self, TextureLoadError> {
        Ok(Self(Texture::new(path)?))
    }

    pub fn size(&self) -> Vector2i {
        Vector2i {
            x: self.0.width,
            y: self.0.height,
        }
    }

    pub fn width(&self) -> i32 {
        self.0.width
    }

    pub fn height(&self) -> i32 {
        self.0.height
    }

    pub fn from_bytes(bytes: &[u8]) -> Self {
        let image =
            unsafe { load_image_from_memory(c".png".as_ptr(), bytes.as_ptr(), bytes.len() as i32) };

        OwnedTexture::from(image)
    }
}

impl OwnedRenderTexture {
    #[inline]
    pub fn new(width: i32, height: i32) -> Result<Self, RenderTextureLoadError> {
        Ok(Self(RenderTexture::new(width, height)?))
    }

    pub fn size(&self) -> Vector2i {
        Vector2i {
            x: self.0.texture.width,
            y: self.0.texture.height,
        }
    }

    pub fn width(&self) -> i32 {
        self.0.texture.width
    }

    pub fn height(&self) -> i32 {
        self.0.texture.height
    }

    pub fn texture(&self) -> &Texture {
        &self.0.texture
    }
}

impl From<Image> for OwnedTexture {
    fn from(value: Image) -> Self {
        Self(unsafe { load_texture_from_image(value) })
    }
}

impl From<texture::Texture> for OwnedTexture {
    fn from(value: texture::Texture) -> Self {
        Self(value)
    }
}

impl From<texture::RenderTexture> for OwnedRenderTexture {
    fn from(value: texture::RenderTexture) -> Self {
        Self(value)
    }
}

impl From<OwnedTexture> for texture::Texture {
    fn from(value: OwnedTexture) -> Self {
        value.0.clone()
    }
}

impl From<&OwnedTexture> for texture::Texture {
    fn from(value: &OwnedTexture) -> Self {
        value.0.clone()
    }
}

impl From<OwnedRenderTexture> for texture::RenderTexture {
    fn from(value: OwnedRenderTexture) -> Self {
        value.0.clone()
    }
}

impl From<&OwnedRenderTexture> for texture::RenderTexture {
    fn from(value: &OwnedRenderTexture) -> Self {
        value.0.clone()
    }
}

impl From<OwnedRenderTexture> for texture::Texture {
    fn from(value: OwnedRenderTexture) -> Self {
        value.texture().clone()
    }
}

impl From<&OwnedRenderTexture> for texture::Texture {
    fn from(value: &OwnedRenderTexture) -> Self {
        value.texture().clone()
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
