use super::{
    color::Color,
    math::{Rectangle, Vector2, Vector2i},
};

#[repr(C)]
#[derive(Debug)]
pub struct Texture(pub(crate) rustyray_sys::Texture);
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub(crate) struct WeakTexture(pub(crate) rustyray_sys::Texture);

#[repr(C)]
#[derive(Debug)]
pub struct RenderTexture {
    id: u32,
    texture: WeakTexture,
    depth: WeakTexture,
}

pub struct DrawHandler;

pub trait Textured {
    fn size(&self) -> Vector2i;
    fn width(&self) -> i32;
    fn height(&self) -> i32;
}

impl Texture {
    pub fn new(path: &String) -> Self {
        unsafe { rustyray_sys::LoadTexture(path.as_ptr() as *const i8).into() }
    }
}

impl Textured for Texture {
    fn size(&self) -> Vector2i {
        Vector2i::new(self.width(), self.height())
    }
    fn width(&self) -> i32 {
        self.0.width
    }
    fn height(&self) -> i32 {
        self.0.height
    }
}

impl Textured for WeakTexture {
    fn size(&self) -> Vector2i {
        Vector2i::new(self.width(), self.height())
    }
    fn width(&self) -> i32 {
        self.0.width
    }
    fn height(&self) -> i32 {
        self.0.height
    }
}

impl RenderTexture {
    pub fn new(width: i32, height: i32) -> Self {
        unsafe { rustyray_sys::LoadRenderTexture(width, height).into() }
    }

    pub fn size(&self) -> Vector2i {
        self.texture.size()
    }
}

impl DrawHandler {
    pub fn draw_fps(&self, x: i32, y: i32) {
        unsafe {
            rustyray_sys::DrawFPS(x, y);
        }
    }

    pub fn clear(&self, color: impl Into<rustyray_sys::Color>) {
        unsafe {
            rustyray_sys::ClearBackground(color.into());
        }
    }

    pub fn draw_render_texture(&self, render_texture: &RenderTexture) {
        let size: Vector2 = render_texture.size().into();
        unsafe {
            let screen_height = rustyray_sys::GetScreenHeight() as f32;
            let screen_width = rustyray_sys::GetScreenWidth() as f32;
            rustyray_sys::DrawTexturePro(
                render_texture.into(),
                Rectangle::new(0.0, 0.0, size.x, -size.y).into(),
                Rectangle::new(0.0, 0.0, screen_width, screen_height).into(),
                Vector2::new(0.0, 0.0).into(),
                0.0,
                Color::WHITE.into(),
            );
        }
    }

    pub fn draw_texture(&self, texture: &Texture, x: i32, y: i32, tint: Color) {
        unsafe {
            rustyray_sys::DrawTexture(texture.into(), x, y, tint.into());
        }
    }

    pub fn draw_rect(&self, rect: Rectangle, tint: Color) {
        unsafe {
            rustyray_sys::DrawRectangleRec(rect.into(), tint.into());
        }
    }

    pub fn draw_text(&self, text: String, pos_x: i32, pos_y: i32, size: i32, tint: Color) {
        unsafe {
            rustyray_sys::DrawText(text.as_ptr() as *const i8, pos_x, pos_y, size, tint.into());
        }
    }
}

impl Drop for Texture {
    fn drop(&mut self) {
        println!("Dropped texture");
        unsafe {
            rustyray_sys::UnloadTexture(*(&self.0));
        }
    }
}

impl Drop for RenderTexture {
    fn drop(&mut self) {
        println!("Dropped render_texture");
        unsafe {
            rustyray_sys::UnloadRenderTexture((&*self).into());
        }
    }
}

impl From<&Texture> for rustyray_sys::Texture {
    fn from(value: &Texture) -> Self {
        unsafe { std::mem::transmute_copy(value) }
    }
}

impl From<&RenderTexture> for rustyray_sys::RenderTexture {
    fn from(value: &RenderTexture) -> Self {
        unsafe { std::mem::transmute_copy(value) }
    }
}

impl From<&RenderTexture> for rustyray_sys::Texture {
    fn from(value: &RenderTexture) -> Self {
        value.texture.into()
    }
}

impl From<WeakTexture> for rustyray_sys::Texture {
    fn from(value: WeakTexture) -> Self {
        unsafe { std::mem::transmute(value) }
    }
}

impl From<rustyray_sys::RenderTexture> for RenderTexture {
    fn from(value: rustyray_sys::RenderTexture) -> Self {
        unsafe { std::mem::transmute(value) }
    }
}

impl From<rustyray_sys::Texture> for Texture {
    fn from(value: rustyray_sys::Texture) -> Self {
        unsafe { std::mem::transmute(value) }
    }
}
