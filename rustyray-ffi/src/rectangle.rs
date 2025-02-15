use std::fmt::Display;

use crate::vector::*;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
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
