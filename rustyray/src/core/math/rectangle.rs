use std::fmt::Display;

use super::Vector2;

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Rectangle {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
}

impl Rectangle {
    pub fn new(x: f32, y: f32, width: f32, height: f32) -> Self {
        Self {
            x,
            y,
            width,
            height,
        }
    }

    /// Get the x and y position of the rectangle as a [Vector2]
    pub fn position(&self) -> Vector2 {
        Vector2 {
            x: self.x,
            y: self.y,
        }
    }

    /// Get the width and height of the rectangle as a [Vector2]
    pub fn size(&self) -> Vector2 {
        Vector2 {
            x: self.width,
            y: self.height,
        }
    }

    pub fn collides_rect(&self, other: &Rectangle) -> bool {
        unsafe { rustyray_sys::ffi::check_collision_recs((*self).into(), (*other).into()) }
    }

    pub fn get_collision_rect(&self, other: &Rectangle) -> Rectangle {
        unsafe {
            Rectangle::from(rustyray_sys::ffi::get_collision_rec(
                (*self).into(),
                (*other).into(),
            ))
        }
    }
}

impl From<rustyray_sys::math::Rectangle> for Rectangle {
    fn from(value: rustyray_sys::math::Rectangle) -> Self {
        unsafe { std::mem::transmute(value) }
    }
}

impl From<Rectangle> for rustyray_sys::math::Rectangle {
    fn from(value: Rectangle) -> Self {
        unsafe { std::mem::transmute(value) }
    }
}

impl From<&Rectangle> for rustyray_sys::math::Rectangle {
    fn from(value: &Rectangle) -> Self {
        unsafe { std::mem::transmute_copy(value) }
    }
}

impl Display for Rectangle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(
            format!(
                "Rectangle{{x: {}, y: {}, width: {}, height: {}}}",
                self.x, self.y, self.width, self.height
            )
            .as_str(),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_intersects() {
        let rect_a = Rectangle::new(0.0, 0.0, 10.0, 10.0);
        let rect_b = Rectangle::new(5.0, 5.0, 6.0, 6.0);
        assert!(rect_a.collides_rect(&rect_b));
        assert!(rect_b.collides_rect(&rect_a));
    }
}
