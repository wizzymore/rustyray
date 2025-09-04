use super::{
    color::Color,
    image::OwnedRenderTexture,
    math::{Rectangle, Vector2, Vector2i},
};
use rustyray_sys::{ffi, texture::Texture};
use std::{ffi::CString, fmt::Debug, marker::PhantomData};

#[derive(Debug, Clone, Copy)]
pub struct Camera2D {
    pub offset: Vector2, // Camera offset (displacement from target)
    pub target: Vector2, // Camera target (rotation and zoom origin)
    pub rotation: f32,   // Camera rotation in degrees
    pub zoom: f32,       // Camera zoom (scaling), should be 1.0f by default
}

impl From<Camera2D> for rustyray_sys::camera::Camera2D {
    fn from(val: Camera2D) -> Self {
        unsafe { std::mem::transmute(val) }
    }
}

impl From<rustyray_sys::camera::Camera2D> for Camera2D {
    fn from(val: rustyray_sys::camera::Camera2D) -> Self {
        unsafe { std::mem::transmute(val) }
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

pub trait Draw {
    #[inline]
    fn draw_fps(&self, x: i32, y: i32) {
        unsafe {
            ffi::draw_fps(x, y);
        }
    }

    #[inline]
    fn clear(&self, color: Color) {
        unsafe {
            ffi::clear_background(color);
        }
    }

    fn draw_render_texture(&self, render_texture: &OwnedRenderTexture) {
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
    fn draw_texture(&self, texture: impl Into<Texture>, x: i32, y: i32, tint: Color) {
        unsafe {
            ffi::draw_texture(texture.into(), x, y, tint);
        }
    }

    #[inline]
    fn draw_texture_pro(
        &self,
        texture: impl Into<Texture>,
        source: Rectangle,
        dest: Rectangle,
        origin: Vector2,
        rotation: f32,
        tint: Color,
    ) {
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
    fn draw_rect(&self, rect: Rectangle, tint: Color) {
        unsafe {
            ffi::draw_rectangle_rec(rect.into(), tint);
        }
    }

    #[inline]
    fn draw_rect_pro(&self, rect: Rectangle, origin: Vector2, rotation: f32, tint: Color) {
        unsafe {
            ffi::draw_rectangle_pro(rect.into(), origin.into(), rotation, tint);
        }
    }

    /// Draw rectangle outline
    #[inline]
    fn draw_rect_lines(&self, rect: Rectangle, tint: Color) {
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
    fn draw_rect_lines_ex(&self, rect: Rectangle, line_thickness: f32, tint: Color) {
        unsafe {
            ffi::draw_rectangle_lines_ex(rect.into(), line_thickness, tint);
        }
    }

    #[inline]
    fn draw_circle(&self, center: Vector2, radius: f32, color: Color) {
        unsafe { ffi::draw_circle_v(center.into(), radius, color) }
    }

    #[inline]
    fn draw_ellipse(&self, center: Vector2i, radius: Vector2, color: Color) {
        unsafe { ffi::draw_ellipse(center.x, center.y, radius.x, radius.y, color) }
    }

    /// Draw text (using default font)
    #[inline]
    fn draw_text<T>(&self, text: T, pos_x: i32, pos_y: i32, size: i32, tint: Color)
    where
        T: AsRef<str>,
    {
        unsafe {
            ffi::draw_text(
                CString::new(text.as_ref()).unwrap().as_ptr(),
                pos_x,
                pos_y,
                size,
                tint,
            );
        }
    }
}

pub struct DrawHandler<'a, T>(PhantomData<&'a mut T>);

impl<'a, T> Draw for DrawHandler<'a, T> {}
impl<'a, T> TextureModeExt for DrawHandler<'a, T> {}

pub trait DrawingExt
where
    Self: Sized,
{
    #[inline]
    fn draw(&mut self, callback: impl FnOnce(DrawHandler<Self>)) {
        let d = self.begin_drawing();
        callback(d);
    }

    #[inline]
    #[must_use]
    fn begin_drawing<'a>(&'a mut self) -> DrawHandler<'a, Self> {
        unsafe {
            ffi::begin_drawing();
        }
        DrawHandler(PhantomData)
    }
}

impl<'a, T> Drop for DrawHandler<'a, T> {
    fn drop(&mut self) {
        unsafe {
            ffi::end_drawing();
        }
    }
}

pub struct TextureModeHandler<'a, 'b, T>(&'a mut T, PhantomData<&'b OwnedRenderTexture>);

impl<'a, 'b, T> Draw for TextureModeHandler<'a, 'b, T> {}

impl<'a, 'b, T> Drop for TextureModeHandler<'a, 'b, T> {
    fn drop(&mut self) {
        unsafe {
            ffi::end_texture_mode();
        }
    }
}

impl<'a, 'b, T: 'a> std::ops::Deref for TextureModeHandler<'a, 'b, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        self.0
    }
}

impl<'a, 'b, T: 'a> std::ops::DerefMut for TextureModeHandler<'a, 'b, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.0
    }
}

pub trait TextureModeExt
where
    Self: Sized,
{
    #[inline]
    fn draw_texture_mode<'a, 'b>(
        &'a mut self,
        render_texture: &'b OwnedRenderTexture,
        callback: impl FnOnce(TextureModeHandler<'a, 'b, Self>),
    ) {
        let dt = self.begin_texture_mode(render_texture);
        callback(dt);
    }

    #[inline]
    #[must_use]
    fn begin_texture_mode<'a, 'b>(
        &'a mut self,
        render_texture: &'b OwnedRenderTexture,
    ) -> TextureModeHandler<'a, 'b, Self> {
        unsafe {
            ffi::begin_texture_mode(render_texture.into());
        }
        TextureModeHandler(self, PhantomData)
    }
}

pub struct Mode2DHandler<'a, 'b, T>(&'a mut T, PhantomData<&'b Camera2D>);

impl<'a, 'b, T> Draw for Mode2DHandler<'a, 'b, T> {}

impl<'a, 'b, T: 'a> Drop for Mode2DHandler<'a, 'b, T> {
    fn drop(&mut self) {
        unsafe {
            ffi::end_mode_2d();
        }
    }
}

impl<'a, 'b, T: 'a> std::ops::Deref for Mode2DHandler<'a, 'b, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        self.0
    }
}

impl<'a, 'b, T: 'a> std::ops::DerefMut for Mode2DHandler<'a, 'b, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.0
    }
}

pub trait Mode2DExt
where
    Self: Sized,
{
    #[inline]
    fn draw_mode_2d<'a, 'b>(
        &'a mut self,
        camera: &'b Camera2D,
        callback: impl FnOnce(Mode2DHandler<'a, 'b, Self>),
    ) {
        let dc = self.begin_mode_2d(camera);
        callback(dc);
    }

    #[inline]
    #[must_use]
    fn begin_mode_2d<'a, 'b>(&'a mut self, camera: &'b Camera2D) -> Mode2DHandler<'a, 'b, Self> {
        unsafe {
            ffi::begin_mode_2d((*camera).into());
        }
        Mode2DHandler(self, PhantomData)
    }
}

impl<'a, T> Mode2DExt for DrawHandler<'a, T> {}
impl<'a, 'b, T: 'a> Mode2DExt for TextureModeHandler<'a, 'b, T> {}
