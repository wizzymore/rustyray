use std::fmt::Display;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[repr(C)]
#[derive(Debug, Clone, Copy, Default)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct Vector2 {
    pub x: f32,
    pub y: f32,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, Default)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct Vector2i {
    pub x: i32,
    pub y: i32,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, Default)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct Rectangle {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
}

impl Vector2 {
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }

    pub fn normalized(&self) -> Self {
        let mut result = self.clone();
        result.normalize();
        result
    }

    pub fn normalize(&mut self) {
        let length = f32::sqrt((self.x * self.x) + (self.y * self.y));

        if length > 0. {
            let ilength = 1. / length;
            self.x = self.x * ilength;
            self.y = self.y * ilength;
        } else {
            self.x = 0.;
            self.y = 0.;
        }
    }
}

impl Vector2i {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
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
}

impl Display for Vector2 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(format!("Vector2{{x: {},y: {}}}", self.x, self.y).as_str())
    }
}

impl Display for Vector2i {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(format!("Vector2i{{x: {},y: {}}}", self.x, self.y).as_str())
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

impl From<Vector2> for rustyray_ffi::Vector2 {
    fn from(value: Vector2) -> Self {
        unsafe { std::mem::transmute(value) }
    }
}

impl From<&Vector2> for rustyray_ffi::Vector2 {
    fn from(value: &Vector2) -> Self {
        value.to_owned().into()
    }
}

impl From<Vector2i> for Vector2 {
    fn from(value: Vector2i) -> Self {
        Self {
            x: value.x as f32,
            y: value.y as f32,
        }
    }
}

impl From<&Vector2i> for Vector2 {
    fn from(value: &Vector2i) -> Self {
        value.to_owned().into()
    }
}

impl From<rustyray_ffi::Vector2> for Vector2 {
    fn from(value: rustyray_ffi::Vector2) -> Self {
        unsafe { std::mem::transmute(value) }
    }
}

impl From<Vector2i> for rustyray_ffi::Vector2 {
    fn from(value: Vector2i) -> Self {
        Self {
            x: value.x as f32,
            y: value.y as f32,
        }
    }
}

impl From<&Vector2i> for rustyray_ffi::Vector2 {
    fn from(value: &Vector2i) -> Self {
        value.to_owned().into()
    }
}

impl From<Vector2> for Vector2i {
    fn from(value: Vector2) -> Self {
        Self {
            x: value.x as i32,
            y: value.y as i32,
        }
    }
}

impl From<&Vector2> for Vector2i {
    fn from(value: &Vector2) -> Self {
        value.to_owned().into()
    }
}

impl From<Rectangle> for rustyray_ffi::Rectangle {
    fn from(value: Rectangle) -> Self {
        unsafe { std::mem::transmute(value) }
    }
}

impl From<&Rectangle> for rustyray_ffi::Rectangle {
    fn from(value: &Rectangle) -> Self {
        value.to_owned().into()
    }
}
