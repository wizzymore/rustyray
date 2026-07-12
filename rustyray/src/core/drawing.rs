use super::{
    assets::{AssetManager, Handle},
    color::Color,
    image::{RenderTexture, Texture},
    math::{Rectangle, Vector2, Vector2i},
};
use rustyray_sys::ffi;
use std::{ffi::CString, fmt::Debug};

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

pub trait HasAssetManager {
    fn assets(&self) -> &AssetManager;

    fn assets_mut(&mut self) -> &mut AssetManager;
}

pub trait Draw {
    fn assets(&self) -> &AssetManager;

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

    #[inline]
    fn draw_render_texture(&self, render_texture: &Handle<RenderTexture>) {
        let Some(rt) = self.assets().get(render_texture) else {
            return;
        };
        let size = rt.size();
        unsafe {
            ffi::draw_texture_pro(
                rt.as_ray().texture,
                Rectangle::new(0.0, 0.0, size.x as f32, -size.y as f32).into(),
                Rectangle::new(0.0, 0.0, size.x as f32, size.y as f32).into(),
                Vector2::ZERO.into(),
                0.0,
                Color::WHITE,
            );
        }
    }

    #[inline]
    fn draw_texture(&self, texture: &Handle<Texture>, x: i32, y: i32, tint: Color) {
        let Some(tex) = self.assets().get(texture) else {
            return;
        };
        unsafe {
            ffi::draw_texture(tex.as_ray(), x, y, tint);
        }
    }

    #[inline]
    fn draw_texture_pro(
        &self,
        texture: &Handle<Texture>,
        source: Rectangle,
        dest: Rectangle,
        origin: Vector2,
        rotation: f32,
        tint: Color,
    ) {
        let Some(tex) = self.assets().get(texture) else {
            return;
        };
        unsafe {
            ffi::draw_texture_pro(
                tex.as_ray(),
                source.into(),
                dest.into(),
                origin.into(),
                rotation,
                tint,
            );
        }
    }

    /// Draw a line
    #[inline]
    fn draw_line(&self, start: Vector2, end: Vector2, thickness: f32, color: Color) {
        unsafe {
            ffi::draw_line_ex(start.into(), end.into(), thickness, color);
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
    fn draw_circle_lines(&self, center: Vector2, radius: f32, color: Color) {
        unsafe { ffi::draw_circle_lines(center.x as i32, center.y as i32, radius, color) }
    }

    #[inline]
    fn draw_ellipse(&self, center: Vector2i, radius: Vector2, color: Color) {
        unsafe { ffi::draw_ellipse(center.x, center.y, radius.x, radius.y, color) }
    }

    #[inline]
    fn draw_triangle(&self, v1: Vector2, v2: Vector2, v3: Vector2, color: Color) {
        unsafe {
            ffi::draw_triangle(v1.into(), v2.into(), v3.into(), color);
        }
    }

    #[inline]
    fn draw_triangle_lines(&self, v1: Vector2, v2: Vector2, v3: Vector2, color: Color) {
        unsafe {
            ffi::draw_triangle_lines(v1.into(), v2.into(), v3.into(), color);
        }
    }

    #[inline]
    fn measure_text<T>(&self, text: T, size: i32) -> i32
    where
        T: AsRef<str>,
    {
        let Ok(cstr) = CString::new(text.as_ref()) else {
            return 0;
        };
        unsafe { ffi::measure_text(cstr.as_ptr(), size) }
    }

    /// Draw text (using default font)
    #[inline]
    fn draw_text<T>(&self, text: T, pos_x: i32, pos_y: i32, size: i32, tint: Color)
    where
        T: AsRef<str>,
    {
        let Ok(cstr) = CString::new(text.as_ref()) else {
            return;
        };
        unsafe {
            ffi::draw_text(cstr.as_ptr(), pos_x, pos_y, size, tint);
        }
    }
}

pub struct DrawHandler<'a> {
    assets: &'a AssetManager,
}

impl<'a> DrawHandler<'a> {
    pub(crate) fn new(assets: &'a AssetManager) -> Self {
        Self { assets }
    }
}

impl Draw for DrawHandler<'_> {
    fn assets(&self) -> &AssetManager {
        self.assets
    }
}

impl HasAssetManager for DrawHandler<'_> {
    fn assets(&self) -> &AssetManager {
        self.assets
    }

    fn assets_mut(&mut self) -> &mut AssetManager {
        panic!("DrawHandler does not support mutable asset access");
    }
}

pub trait DrawingExt: HasAssetManager
where
    Self: Sized,
{
    #[inline]
    fn draw(&mut self, callback: impl FnOnce(DrawHandler<'_>)) {
        let d = self.begin_drawing();
        callback(d);
    }

    #[inline]
    #[must_use]
    fn begin_drawing(&mut self) -> DrawHandler<'_> {
        self.assets_mut().process_assets();
        unsafe {
            ffi::begin_drawing();
        }
        DrawHandler::new(self.assets())
    }
}

impl<'a> Drop for DrawHandler<'a> {
    fn drop(&mut self) {
        unsafe {
            ffi::end_drawing();
        }
    }
}

pub struct TextureModeHandler<'a> {
    assets: &'a AssetManager,
}

impl<'a> TextureModeHandler<'a> {
    pub(crate) fn new(assets: &'a AssetManager) -> Self {
        Self { assets }
    }
}

impl Draw for TextureModeHandler<'_> {
    fn assets(&self) -> &AssetManager {
        self.assets
    }
}

impl HasAssetManager for TextureModeHandler<'_> {
    fn assets(&self) -> &AssetManager {
        self.assets
    }

    fn assets_mut(&mut self) -> &mut AssetManager {
        panic!("TextureModeHandler does not support mutable asset access");
    }
}

impl Drop for TextureModeHandler<'_> {
    fn drop(&mut self) {
        unsafe {
            ffi::end_texture_mode();
        }
    }
}

pub trait TextureModeExt: HasAssetManager
where
    Self: Sized,
{
    #[inline]
    fn draw_texture_mode(
        &mut self,
        render_texture: &Handle<RenderTexture>,
        callback: impl FnOnce(TextureModeHandler<'_>),
    ) {
        let Some(dt) = self.begin_texture_mode(render_texture) else {
            return;
        };
        callback(dt);
    }

    #[inline]
    #[must_use]
    fn begin_texture_mode(
        &mut self,
        render_texture: &Handle<RenderTexture>,
    ) -> Option<TextureModeHandler<'_>> {
        let assets = self.assets_mut();
        assets.process_assets();
        let rt = assets.get(render_texture)?;
        unsafe {
            ffi::begin_texture_mode(rt.as_ray());
        }
        Some(TextureModeHandler::new(self.assets()))
    }
}

impl TextureModeExt for DrawHandler<'_> {}

pub struct Mode2DHandler<'a> {
    assets: &'a AssetManager,
}

impl<'a> Mode2DHandler<'a> {
    pub(crate) fn new(assets: &'a AssetManager) -> Self {
        Self { assets }
    }
}

impl Draw for Mode2DHandler<'_> {
    fn assets(&self) -> &AssetManager {
        self.assets
    }
}

impl HasAssetManager for Mode2DHandler<'_> {
    fn assets(&self) -> &AssetManager {
        self.assets
    }

    fn assets_mut(&mut self) -> &mut AssetManager {
        panic!("Mode2DHandler does not support mutable asset access");
    }
}

impl Drop for Mode2DHandler<'_> {
    fn drop(&mut self) {
        unsafe {
            ffi::end_mode_2d();
        }
    }
}

pub trait Mode2DExt: HasAssetManager
where
    Self: Sized,
{
    #[inline]
    fn draw_mode_2d(&mut self, camera: &Camera2D, callback: impl FnOnce(Mode2DHandler<'_>)) {
        let dc = self.begin_mode_2d(camera);
        callback(dc);
    }

    #[inline]
    #[must_use]
    fn begin_mode_2d(&mut self, camera: &Camera2D) -> Mode2DHandler<'_> {
        unsafe {
            ffi::begin_mode_2d((*camera).into());
        }
        Mode2DHandler::new(self.assets())
    }
}

impl Mode2DExt for DrawHandler<'_> {}
impl Mode2DExt for TextureModeHandler<'_> {}
