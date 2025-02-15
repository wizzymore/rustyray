use std::ffi::CString;
use std::fmt::Debug;
use std::fs;

use libc::{c_int, c_uint};
use thiserror::Error;

use crate::ffi::{is_window_ready, load_render_texture, load_texture};
use crate::vector::Vector2i;

/// Texture, tex data stored in GPU memory (VRAM)
#[repr(C)]
#[derive(Debug, Clone)]
pub struct Texture {
    /// OpenGL texture id
    id: c_uint,
    /// Texture base width
    width: c_int,
    /// Texture base height
    height: c_int,
    /// Mipmap levels, 1 by default
    mipmaps: c_int,
    /// Data format (PixelFormat type)
    format: c_int,
}

/// RenderTexture, fbo for texture rendering
#[repr(C)]
#[derive(Debug, Clone)]
pub struct RenderTexture {
    /// OpenGL framebuffer object id
    id: c_uint,
    /// Color buffer attachment texture
    pub texture: Texture,
    /// Depth buffer attachment texture
    pub depth: Texture,
}
pub type RenderTexture2D = RenderTexture;

#[derive(Error)]
pub enum TextureLoadError {
    #[error("could not find file at path: {0}")]
    FileNotFound(String),
    #[error("you must first create a Window before loading textures")]
    WindowNotReady(),
}

impl Debug for TextureLoadError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}

impl Texture {
    pub fn new(path: String) -> Result<Self, TextureLoadError> {
        let window_ready = unsafe { is_window_ready() };
        if !window_ready {
            return Err(TextureLoadError::WindowNotReady());
        }
        let metadata = fs::metadata(&path);
        if metadata.is_err() {
            return Err(TextureLoadError::FileNotFound(path));
        }
        if !metadata.unwrap().is_file() {
            return Err(TextureLoadError::FileNotFound(path));
        }

        unsafe { Ok(load_texture(CString::new(path).unwrap().as_ptr())) }
    }

    pub fn width(&self) -> i32 {
        self.width
    }

    pub fn height(&self) -> i32 {
        self.height
    }

    pub fn size(&self) -> Vector2i {
        Vector2i {
            x: self.width,
            y: self.height,
        }
    }
}

impl RenderTexture {
    pub fn new(width: i32, height: i32) -> Self {
        unsafe { load_render_texture(width, height) }
    }
}
