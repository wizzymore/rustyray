use std::{fmt::Display, ops};

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

    pub fn to_vector2i(&self) -> Vector2i {
        Vector2i {
            x: self.x as i32,
            y: self.y as i32,
        }
    }
}

impl Vector2i {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    pub fn to_vector2(&self) -> Vector2 {
        Vector2 {
            x: self.x as f32,
            y: self.y as f32,
        }
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

impl ops::Add<Vector2> for Vector2 {
    type Output = Vector2;

    fn add(self, rhs: Vector2) -> Self::Output {
        Self::Output {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl ops::Add<Vector2i> for Vector2 {
    type Output = Vector2;

    fn add(self, rhs: Vector2i) -> Self::Output {
        Self::Output {
            x: self.x + rhs.x as f32,
            y: self.y + rhs.y as f32,
        }
    }
}

impl ops::Add<f32> for Vector2 {
    type Output = Vector2;

    fn add(self, rhs: f32) -> Self::Output {
        Self::Output {
            x: self.x + rhs,
            y: self.y + rhs,
        }
    }
}

impl ops::Add<i32> for Vector2 {
    type Output = Vector2;

    fn add(self, rhs: i32) -> Self::Output {
        Self::Output {
            x: self.x + rhs as f32,
            y: self.y + rhs as f32,
        }
    }
}

impl ops::Sub<Vector2> for Vector2 {
    type Output = Vector2;

    fn sub(self, rhs: Vector2) -> Self::Output {
        Self::Output {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl ops::Sub<Vector2i> for Vector2 {
    type Output = Vector2;

    fn sub(self, rhs: Vector2i) -> Self::Output {
        Self::Output {
            x: self.x - rhs.x as f32,
            y: self.y - rhs.y as f32,
        }
    }
}

impl ops::Sub<f32> for Vector2 {
    type Output = Vector2;

    fn sub(self, rhs: f32) -> Self::Output {
        Self::Output {
            x: self.x - rhs,
            y: self.y - rhs,
        }
    }
}

impl ops::Sub<i32> for Vector2 {
    type Output = Vector2;

    fn sub(self, rhs: i32) -> Self::Output {
        Self::Output {
            x: self.x - rhs as f32,
            y: self.y - rhs as f32,
        }
    }
}

impl ops::Mul<Vector2> for Vector2 {
    type Output = Vector2;

    fn mul(self, rhs: Vector2) -> Self::Output {
        Self::Output {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
        }
    }
}

impl ops::Mul<Vector2i> for Vector2 {
    type Output = Vector2;

    fn mul(self, rhs: Vector2i) -> Self::Output {
        Self::Output {
            x: self.x * rhs.x as f32,
            y: self.y * rhs.y as f32,
        }
    }
}

impl ops::Mul<f32> for Vector2 {
    type Output = Vector2;

    fn mul(self, rhs: f32) -> Self::Output {
        Self::Output {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

impl ops::Mul<i32> for Vector2 {
    type Output = Vector2;

    fn mul(self, rhs: i32) -> Self::Output {
        Self::Output {
            x: self.x * rhs as f32,
            y: self.y * rhs as f32,
        }
    }
}

impl ops::Div<Vector2> for Vector2 {
    type Output = Vector2;

    fn div(self, rhs: Vector2) -> Self::Output {
        Self::Output {
            x: self.x / rhs.x,
            y: self.y / rhs.y,
        }
    }
}

impl ops::Div<Vector2i> for Vector2 {
    type Output = Vector2;

    fn div(self, rhs: Vector2i) -> Self::Output {
        Self::Output {
            x: self.x / rhs.x as f32,
            y: self.y / rhs.y as f32,
        }
    }
}

impl ops::Div<f32> for Vector2 {
    type Output = Vector2;

    fn div(self, rhs: f32) -> Self::Output {
        Self::Output {
            x: self.x / rhs,
            y: self.y / rhs,
        }
    }
}

impl ops::Div<i32> for Vector2 {
    type Output = Vector2;

    fn div(self, rhs: i32) -> Self::Output {
        Self::Output {
            x: self.x / rhs as f32,
            y: self.y / rhs as f32,
        }
    }
}

impl ops::AddAssign<Vector2> for Vector2 {
    fn add_assign(&mut self, rhs: Vector2) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl ops::AddAssign<Vector2i> for Vector2 {
    fn add_assign(&mut self, rhs: Vector2i) {
        self.x += rhs.x as f32;
        self.y += rhs.y as f32;
    }
}

impl ops::AddAssign<f32> for Vector2 {
    fn add_assign(&mut self, rhs: f32) {
        self.x += rhs;
        self.y += rhs;
    }
}

impl ops::AddAssign<i32> for Vector2 {
    fn add_assign(&mut self, rhs: i32) {
        self.x += rhs as f32;
        self.y += rhs as f32;
    }
}

impl ops::SubAssign<Vector2> for Vector2 {
    fn sub_assign(&mut self, rhs: Vector2) {
        self.x -= rhs.x;
        self.y -= rhs.y;
    }
}

impl ops::SubAssign<Vector2i> for Vector2 {
    fn sub_assign(&mut self, rhs: Vector2i) {
        self.x -= rhs.x as f32;
        self.y -= rhs.y as f32;
    }
}

impl ops::SubAssign<f32> for Vector2 {
    fn sub_assign(&mut self, rhs: f32) {
        self.x -= rhs;
        self.y -= rhs;
    }
}

impl ops::SubAssign<i32> for Vector2 {
    fn sub_assign(&mut self, rhs: i32) {
        self.x -= rhs as f32;
        self.y -= rhs as f32;
    }
}

impl ops::MulAssign<Vector2> for Vector2 {
    fn mul_assign(&mut self, rhs: Vector2) {
        self.x *= rhs.x;
        self.y *= rhs.y;
    }
}

impl ops::MulAssign<Vector2i> for Vector2 {
    fn mul_assign(&mut self, rhs: Vector2i) {
        self.x *= rhs.x as f32;
        self.y *= rhs.y as f32;
    }
}

impl ops::MulAssign<f32> for Vector2 {
    fn mul_assign(&mut self, rhs: f32) {
        self.x *= rhs;
        self.y *= rhs;
    }
}

impl ops::MulAssign<i32> for Vector2 {
    fn mul_assign(&mut self, rhs: i32) {
        self.x *= rhs as f32;
        self.y *= rhs as f32;
    }
}

impl ops::DivAssign<Vector2> for Vector2 {
    fn div_assign(&mut self, rhs: Vector2) {
        self.x /= rhs.x;
        self.y /= rhs.y;
    }
}

impl ops::DivAssign<Vector2i> for Vector2 {
    fn div_assign(&mut self, rhs: Vector2i) {
        self.x /= rhs.x as f32;
        self.y /= rhs.y as f32;
    }
}

impl ops::DivAssign<f32> for Vector2 {
    fn div_assign(&mut self, rhs: f32) {
        self.x /= rhs;
        self.y /= rhs;
    }
}

impl ops::DivAssign<i32> for Vector2 {
    fn div_assign(&mut self, rhs: i32) {
        self.x /= rhs as f32;
        self.y /= rhs as f32;
    }
}

impl ops::Add<Vector2> for Vector2i {
    type Output = Vector2i;

    fn add(self, rhs: Vector2) -> Self::Output {
        Self::Output {
            x: self.x + rhs.x as i32,
            y: self.y + rhs.y as i32,
        }
    }
}

impl ops::Add<Vector2i> for Vector2i {
    type Output = Vector2i;

    fn add(self, rhs: Vector2i) -> Self::Output {
        Self::Output {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl ops::Add<f32> for Vector2i {
    type Output = Vector2i;

    fn add(self, rhs: f32) -> Self::Output {
        Self::Output {
            x: self.x + rhs as i32,
            y: self.y + rhs as i32,
        }
    }
}

impl ops::Add<i32> for Vector2i {
    type Output = Vector2i;

    fn add(self, rhs: i32) -> Self::Output {
        Self::Output {
            x: self.x + rhs,
            y: self.y + rhs,
        }
    }
}

impl ops::Sub<Vector2> for Vector2i {
    type Output = Vector2i;

    fn sub(self, rhs: Vector2) -> Self::Output {
        Self::Output {
            x: self.x - rhs.x as i32,
            y: self.y - rhs.y as i32,
        }
    }
}

impl ops::Sub<Vector2i> for Vector2i {
    type Output = Vector2i;

    fn sub(self, rhs: Vector2i) -> Self::Output {
        Self::Output {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl ops::Sub<f32> for Vector2i {
    type Output = Vector2i;

    fn sub(self, rhs: f32) -> Self::Output {
        Self::Output {
            x: self.x - rhs as i32,
            y: self.y - rhs as i32,
        }
    }
}

impl ops::Sub<i32> for Vector2i {
    type Output = Vector2i;

    fn sub(self, rhs: i32) -> Self::Output {
        Self::Output {
            x: self.x - rhs,
            y: self.y - rhs,
        }
    }
}

impl ops::Mul<Vector2> for Vector2i {
    type Output = Vector2i;

    fn mul(self, rhs: Vector2) -> Self::Output {
        Self::Output {
            x: self.x * rhs.x as i32,
            y: self.y * rhs.y as i32,
        }
    }
}

impl ops::Mul<Vector2i> for Vector2i {
    type Output = Vector2i;

    fn mul(self, rhs: Vector2i) -> Self::Output {
        Self::Output {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
        }
    }
}

impl ops::Mul<f32> for Vector2i {
    type Output = Vector2i;

    fn mul(self, rhs: f32) -> Self::Output {
        Self::Output {
            x: self.x * rhs as i32,
            y: self.y * rhs as i32,
        }
    }
}

impl ops::Mul<i32> for Vector2i {
    type Output = Vector2i;

    fn mul(self, rhs: i32) -> Self::Output {
        Self::Output {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

impl ops::Div<Vector2> for Vector2i {
    type Output = Vector2i;

    fn div(self, rhs: Vector2) -> Self::Output {
        Self::Output {
            x: self.x / rhs.x as i32,
            y: self.y / rhs.y as i32,
        }
    }
}

impl ops::Div<Vector2i> for Vector2i {
    type Output = Vector2i;

    fn div(self, rhs: Vector2i) -> Self::Output {
        Self::Output {
            x: self.x / rhs.x,
            y: self.y / rhs.y,
        }
    }
}

impl ops::Div<f32> for Vector2i {
    type Output = Vector2i;

    fn div(self, rhs: f32) -> Self::Output {
        Self::Output {
            x: self.x / rhs as i32,
            y: self.y / rhs as i32,
        }
    }
}

impl ops::Div<i32> for Vector2i {
    type Output = Vector2i;

    fn div(self, rhs: i32) -> Self::Output {
        Self::Output {
            x: self.x / rhs,
            y: self.y / rhs,
        }
    }
}

impl ops::AddAssign<Vector2> for Vector2i {
    fn add_assign(&mut self, rhs: Vector2) {
        self.x += rhs.x as i32;
        self.y += rhs.y as i32;
    }
}

impl ops::AddAssign<Vector2i> for Vector2i {
    fn add_assign(&mut self, rhs: Vector2i) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl ops::AddAssign<f32> for Vector2i {
    fn add_assign(&mut self, rhs: f32) {
        self.x += rhs as i32;
        self.y += rhs as i32;
    }
}

impl ops::AddAssign<i32> for Vector2i {
    fn add_assign(&mut self, rhs: i32) {
        self.x += rhs;
        self.y += rhs;
    }
}

impl ops::SubAssign<Vector2> for Vector2i {
    fn sub_assign(&mut self, rhs: Vector2) {
        self.x -= rhs.x as i32;
        self.y -= rhs.y as i32;
    }
}

impl ops::SubAssign<Vector2i> for Vector2i {
    fn sub_assign(&mut self, rhs: Vector2i) {
        self.x -= rhs.x;
        self.y -= rhs.y;
    }
}

impl ops::SubAssign<f32> for Vector2i {
    fn sub_assign(&mut self, rhs: f32) {
        self.x -= rhs as i32;
        self.y -= rhs as i32;
    }
}

impl ops::SubAssign<i32> for Vector2i {
    fn sub_assign(&mut self, rhs: i32) {
        self.x -= rhs;
        self.y -= rhs;
    }
}

impl ops::MulAssign<Vector2> for Vector2i {
    fn mul_assign(&mut self, rhs: Vector2) {
        self.x *= rhs.x as i32;
        self.y *= rhs.y as i32;
    }
}

impl ops::MulAssign<Vector2i> for Vector2i {
    fn mul_assign(&mut self, rhs: Vector2i) {
        self.x *= rhs.x;
        self.y *= rhs.y;
    }
}

impl ops::MulAssign<f32> for Vector2i {
    fn mul_assign(&mut self, rhs: f32) {
        self.x *= rhs as i32;
        self.y *= rhs as i32;
    }
}

impl ops::MulAssign<i32> for Vector2i {
    fn mul_assign(&mut self, rhs: i32) {
        self.x *= rhs;
        self.y *= rhs;
    }
}

impl ops::DivAssign<Vector2> for Vector2i {
    fn div_assign(&mut self, rhs: Vector2) {
        self.x /= rhs.x as i32;
        self.y /= rhs.y as i32;
    }
}

impl ops::DivAssign<Vector2i> for Vector2i {
    fn div_assign(&mut self, rhs: Vector2i) {
        self.x /= rhs.x;
        self.y /= rhs.y;
    }
}

impl ops::DivAssign<f32> for Vector2i {
    fn div_assign(&mut self, rhs: f32) {
        self.x /= rhs as i32;
        self.y /= rhs as i32;
    }
}

impl ops::DivAssign<i32> for Vector2i {
    fn div_assign(&mut self, rhs: i32) {
        self.x /= rhs;
        self.y /= rhs;
    }
}
