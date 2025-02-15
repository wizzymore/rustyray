use std::ffi::CString;

use super::{
    color::Color,
    math::vector::{Rectangle, Vector2, Vector2i},
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
pub struct OwnedTexture(pub(crate) rustyray_ffi::Texture);
/// This is the same type as [OwnedTexture], but,
/// instead of implementing Drop and unloading the texture,
/// the [WeakTexture] implements Copy so you can move the texture around easily,
/// or use it in another structs that clean their own textures like [RenderTexture]
///
/// You should not create a WeakTexture manually.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub(crate) struct WeakTexture(pub(crate) rustyray_ffi::Texture);

#[repr(C)]
#[derive(Debug)]
pub struct RenderTexture {
    id: u32,
    texture: WeakTexture,
    depth: WeakTexture,
}

pub struct DrawHandler;

pub trait Texture {
    fn size(&self) -> Vector2i;
    fn width(&self) -> i32;
    fn height(&self) -> i32;
}

impl OwnedTexture {
    /// Creates a new [`OwnedTexture`] from the specified path.
    ///
    /// This will also load the texture in the GPU's memory.
    #[inline]
    pub fn new(path: String) -> Self {
        unsafe { rustyray_ffi::LoadTexture(CString::new(path).unwrap().as_ptr()).into() }
    }
}

// impl OwnedTexture {
//     pub(crate) fn weak(&self) -> WeakTexture {
//         WeakTexture(self.0)
//     }
// }

impl Texture for OwnedTexture {
    #[inline]
    fn size(&self) -> Vector2i {
        Vector2i::new(self.width(), self.height())
    }

    #[inline]
    fn width(&self) -> i32 {
        self.0.width
    }

    #[inline]
    fn height(&self) -> i32 {
        self.0.height
    }
}

impl Texture for WeakTexture {
    #[inline]
    fn size(&self) -> Vector2i {
        Vector2i::new(self.width(), self.height())
    }

    #[inline]
    fn width(&self) -> i32 {
        self.0.width
    }

    #[inline]
    fn height(&self) -> i32 {
        self.0.height
    }
}

impl RenderTexture {
    #[inline]
    pub fn new(width: i32, height: i32) -> Self {
        unsafe { rustyray_ffi::LoadRenderTexture(width, height).into() }
    }

    #[inline]
    pub fn size(&self) -> Vector2i {
        self.texture.size()
    }
}

impl DrawHandler {
    #[inline]
    pub fn draw_fps(&self, x: i32, y: i32) {
        unsafe {
            rustyray_ffi::DrawFPS(x, y);
        }
    }

    #[inline]
    pub fn clear(&self, color: impl Into<rustyray_ffi::Color>) {
        unsafe {
            rustyray_ffi::ClearBackground(color.into());
        }
    }

    pub fn draw_render_texture(&self, render_texture: &RenderTexture) {
        let size: Vector2 = render_texture.size().into();
        unsafe {
            let screen_height = rustyray_ffi::GetScreenHeight() as f32;
            let screen_width = rustyray_ffi::GetScreenWidth() as f32;
            rustyray_ffi::DrawTexturePro(
                render_texture.into(),
                Rectangle::new(0.0, 0.0, size.x, -size.y).into(),
                Rectangle::new(0.0, 0.0, screen_width, screen_height).into(),
                Vector2::new(0.0, 0.0).into(),
                0.0,
                Color::WHITE.into(),
            );
        }
    }

    #[inline]
    pub fn draw_texture(&self, texture: &OwnedTexture, x: i32, y: i32, tint: Color) {
        unsafe {
            rustyray_ffi::DrawTexture(texture.into(), x, y, tint.into());
        }
    }

    #[inline]
    pub fn draw_rect(&self, rect: Rectangle, tint: Color) {
        unsafe {
            rustyray_ffi::DrawRectangleRec(rect.into(), tint.into());
        }
    }

    #[inline]
    pub fn draw_text(&self, text: String, pos_x: i32, pos_y: i32, size: i32, tint: Color) {
        unsafe {
            rustyray_ffi::DrawText(
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
        println!("Dropped texture");
        unsafe {
            rustyray_ffi::UnloadTexture(*(&self.0));
        }
    }
}

impl Drop for RenderTexture {
    fn drop(&mut self) {
        println!("Dropped render_texture");
        unsafe {
            rustyray_ffi::UnloadRenderTexture((&*self).into());
        }
    }
}

impl From<&OwnedTexture> for rustyray_ffi::Texture {
    fn from(value: &OwnedTexture) -> Self {
        value.0
    }
}

impl From<&RenderTexture> for rustyray_ffi::RenderTexture {
    fn from(value: &RenderTexture) -> Self {
        unsafe { std::mem::transmute_copy(value) }
    }
}

impl From<&RenderTexture> for rustyray_ffi::Texture {
    fn from(value: &RenderTexture) -> Self {
        value.texture.into()
    }
}

impl From<WeakTexture> for rustyray_ffi::Texture {
    fn from(value: WeakTexture) -> Self {
        value.0
    }
}

impl From<rustyray_ffi::Texture> for WeakTexture {
    fn from(value: rustyray_ffi::Texture) -> Self {
        Self(value)
    }
}

impl From<rustyray_ffi::RenderTexture> for RenderTexture {
    fn from(value: rustyray_ffi::RenderTexture) -> Self {
        Self {
            id: value.id,
            texture: value.texture.into(),
            depth: value.depth.into(),
        }
    }
}

impl From<rustyray_ffi::Texture> for OwnedTexture {
    fn from(value: rustyray_ffi::Texture) -> Self {
        Self(value)
    }
}
