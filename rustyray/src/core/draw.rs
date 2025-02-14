use super::{
    color::Color,
    math::{Rectangle, Vector2, Vector2i},
};

#[derive(Debug)]
pub struct Texture(pub(crate) rustyray_sys::Texture);
#[derive(Debug)]
pub struct WeakTexture(pub(crate) rustyray_sys::Texture);

#[derive(Debug)]
pub struct RenderTexture(pub(crate) rustyray_sys::RenderTexture);

pub struct DrawHandler;

impl Texture {
    pub fn new(path: &String) -> Self {
        unsafe { rustyray_sys::LoadTexture(path.as_ptr() as *const i8).into() }
    }

    pub fn size(&self) -> Vector2i {
        Vector2i::new(self.0.width.clone(), self.0.height.clone())
    }
}

impl WeakTexture {
    pub fn size(&self) -> Vector2i {
        Vector2i::new(self.0.width.clone(), self.0.height.clone())
    }
}

impl RenderTexture {
    pub fn new(width: i32, height: i32) -> Self {
        unsafe { rustyray_sys::LoadRenderTexture(width, height).into() }
    }

    pub fn texture(&self) -> WeakTexture {
        unsafe { std::mem::transmute(self.0.texture.clone()) }
    }
}

impl DrawHandler {
    pub fn draw_fps(&self, pos: &Vector2i) {
        unsafe {
            rustyray_sys::DrawFPS(pos.x, pos.y);
        }
    }

    pub fn clear(&self, color: impl Into<rustyray_sys::Color>) {
        unsafe {
            rustyray_sys::ClearBackground(color.into());
        }
    }

    pub fn draw_render_texture(&self, render_texture: &RenderTexture) {
        unsafe {
            let screen_height = rustyray_sys::GetScreenHeight() as f32;
            let screen_width = rustyray_sys::GetScreenWidth() as f32;
            let size: Vector2 = render_texture.texture().size().into();
            rustyray_sys::DrawTexturePro(
                *(&render_texture.0.texture),
                Rectangle::new(0.0, 0.0, size.x, -size.y).into(),
                Rectangle::new(0.0, 0.0, screen_width, screen_height).into(),
                Vector2::new(0.0, 0.0).into(),
                0.0,
                Color::WHITE.into(),
            );
        }
    }
}

impl Drop for Texture {
    fn drop(&mut self) {
        // debug_assert!(self.id != 0, "The texture was created but never loaded in VRAM. Please use Texture::new() to create a texture.");
        println!("Dropped texture");
        unsafe {
            rustyray_sys::UnloadTexture(*(&self.0));
        }
    }
}

impl Drop for RenderTexture {
    fn drop(&mut self) {
        // debug_assert!(self.id != 0, "The texture was created but never loaded in VRAM. Please use Texture::new() to create a texture.");
        println!("Dropped render_texture");
        unsafe {
            rustyray_sys::UnloadRenderTexture(*(&self.0));
        }
    }
}

impl From<rustyray_sys::RenderTexture> for RenderTexture {
    fn from(value: rustyray_sys::RenderTexture) -> Self {
        Self { 0: value }
    }
}

impl From<rustyray_sys::Texture> for Texture {
    fn from(value: rustyray_sys::Texture) -> Self {
        Self { 0: value }
    }
}
