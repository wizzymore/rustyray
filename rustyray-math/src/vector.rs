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

impl Vector2 {
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }

    pub fn normalized(&self) -> Self {
        let mut result = *self;
        result.normalize();
        result
    }

    pub fn normalize(&mut self) {
        let length = ((self.x * self.x) + (self.y * self.y)).sqrt();

        if length > 0. {
            let ilength = 1. / length;
            self.x *= ilength;
            self.y *= ilength;
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

impl ops::Add<Vector2> for f32 {
    type Output = Vector2;

    fn add(self, rhs: Vector2) -> Self::Output {
        Self::Output {
            x: self + rhs.x,
            y: self + rhs.y,
        }
    }
}

impl ops::Add<Vector2> for i32 {
    type Output = Vector2;

    fn add(self, rhs: Vector2) -> Self::Output {
        Self::Output {
            x: self as f32 + rhs.x,
            y: self as f32 + rhs.y,
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

impl ops::Sub<Vector2> for f32 {
    type Output = Vector2;

    fn sub(self, rhs: Vector2) -> Self::Output {
        Self::Output {
            x: self - rhs.x,
            y: self - rhs.y,
        }
    }
}

impl ops::Sub<Vector2> for i32 {
    type Output = Vector2;

    fn sub(self, rhs: Vector2) -> Self::Output {
        Self::Output {
            x: self as f32 - rhs.x,
            y: self as f32 - rhs.y,
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

impl ops::Mul<Vector2> for f32 {
    type Output = Vector2;

    fn mul(self, rhs: Vector2) -> Self::Output {
        Self::Output {
            x: self * rhs.x,
            y: self * rhs.y,
        }
    }
}

impl ops::Mul<Vector2> for i32 {
    type Output = Vector2;

    fn mul(self, rhs: Vector2) -> Self::Output {
        Self::Output {
            x: self as f32 * rhs.x,
            y: self as f32 * rhs.y,
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

impl ops::Div<Vector2> for f32 {
    type Output = Vector2;

    fn div(self, rhs: Vector2) -> Self::Output {
        Self::Output {
            x: self / rhs.x,
            y: self / rhs.y,
        }
    }
}

impl ops::Div<Vector2> for i32 {
    type Output = Vector2;

    fn div(self, rhs: Vector2) -> Self::Output {
        Self::Output {
            x: self as f32 / rhs.x,
            y: self as f32 / rhs.y,
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

impl ops::Add<Vector2i> for f32 {
    type Output = Vector2i;

    fn add(self, rhs: Vector2i) -> Self::Output {
        Self::Output {
            x: self as i32 + rhs.x,
            y: self as i32 + rhs.y,
        }
    }
}

impl ops::Add<Vector2i> for i32 {
    type Output = Vector2i;

    fn add(self, rhs: Vector2i) -> Self::Output {
        Self::Output {
            x: self + rhs.x,
            y: self + rhs.y,
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

impl ops::Sub<Vector2i> for f32 {
    type Output = Vector2i;

    fn sub(self, rhs: Vector2i) -> Self::Output {
        Self::Output {
            x: self as i32 - rhs.x,
            y: self as i32 - rhs.y,
        }
    }
}

impl ops::Sub<Vector2i> for i32 {
    type Output = Vector2i;

    fn sub(self, rhs: Vector2i) -> Self::Output {
        Self::Output {
            x: self - rhs.x,
            y: self - rhs.y,
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

impl ops::Mul<Vector2i> for f32 {
    type Output = Vector2i;

    fn mul(self, rhs: Vector2i) -> Self::Output {
        Self::Output {
            x: self as i32 * rhs.x,
            y: self as i32 * rhs.y,
        }
    }
}

impl ops::Mul<Vector2i> for i32 {
    type Output = Vector2i;

    fn mul(self, rhs: Vector2i) -> Self::Output {
        Self::Output {
            x: self * rhs.x,
            y: self * rhs.y,
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

impl ops::Div<Vector2i> for f32 {
    type Output = Vector2i;

    fn div(self, rhs: Vector2i) -> Self::Output {
        Self::Output {
            x: self as i32 / rhs.x,
            y: self as i32 / rhs.y,
        }
    }
}

impl ops::Div<Vector2i> for i32 {
    type Output = Vector2i;

    fn div(self, rhs: Vector2i) -> Self::Output {
        Self::Output {
            x: self / rhs.x,
            y: self / rhs.y,
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

#[cfg(test)]
mod tests {
    use crate::vector::{Vector2, Vector2i};

    #[test]
    fn vector2_add_vector2() {
        let result = Vector2::new(0., 0.) + Vector2::new(1., 1.);
        assert_eq!(result.x, 1.);
        assert_eq!(result.y, 1.);
    }

    #[test]
    fn vector2_add_vector2i() {
        let result = Vector2::new(0., 0.) + Vector2i::new(1, 1);
        assert_eq!(result.x, 1.);
        assert_eq!(result.y, 1.);
    }

    #[test]
    fn vector2_add_f32() {
        let result = Vector2::new(0., 0.) + 1.;
        assert_eq!(result.x, 1.);
        assert_eq!(result.y, 1.);
    }

    #[test]
    fn vector2_add_i32() {
        let result = Vector2::new(0., 0.) + 1;
        assert_eq!(result.x, 1.);
        assert_eq!(result.y, 1.);
    }

    #[test]
    fn f32_add_vector2() {
        let result = 1. + Vector2::new(0., 0.);
        assert_eq!(result.x, 1.);
        assert_eq!(result.y, 1.);
    }

    #[test]
    fn i32_add_vector2() {
        let result = 1 + Vector2::new(0., 0.);
        assert_eq!(result.x, 1.);
        assert_eq!(result.y, 1.);
    }

    #[test]
    fn vector2_sub_vector2() {
        let result = Vector2::new(1., 1.) - Vector2::new(1., 1.);
        assert_eq!(result.x, 0.);
        assert_eq!(result.y, 0.);
    }

    #[test]
    fn vector2_sub_vector2i() {
        let result = Vector2::new(1., 1.) - Vector2i::new(1, 1);
        assert_eq!(result.x, 0.);
        assert_eq!(result.y, 0.);
    }

    #[test]
    fn vector2_sub_f32() {
        let result = Vector2::new(1., 1.) - 1.;
        assert_eq!(result.x, 0.);
        assert_eq!(result.y, 0.);
    }

    #[test]
    fn vector2_sub_i32() {
        let result = Vector2::new(1., 1.) - 1;
        assert_eq!(result.x, 0.);
        assert_eq!(result.y, 0.);
    }

    #[test]
    fn f32_sub_vector2() {
        let result = 0. - Vector2::new(1., 1.);
        assert_eq!(result.x, -1.);
        assert_eq!(result.y, -1.);
    }

    #[test]
    fn i32_sub_vector2() {
        let result = 0 - Vector2::new(1., 1.);
        assert_eq!(result.x, -1.);
        assert_eq!(result.y, -1.);
    }

    #[test]
    fn vector2_mul_vector2() {
        let result = Vector2::new(1., 1.) * Vector2::new(2., 2.);
        assert_eq!(result.x, 2.);
        assert_eq!(result.y, 2.);
    }

    #[test]
    fn vector2_mul_vector2i() {
        let result = Vector2::new(1., 1.) * Vector2i::new(2, 2);
        assert_eq!(result.x, 2.);
        assert_eq!(result.y, 2.);
    }

    #[test]
    fn vector2_mul_f32() {
        let result = Vector2::new(1., 1.) * 2.;
        assert_eq!(result.x, 2.);
        assert_eq!(result.y, 2.);
    }

    #[test]
    fn vector2_mul_i32() {
        let result = Vector2::new(1., 1.) * 2;
        assert_eq!(result.x, 2.);
        assert_eq!(result.y, 2.);
    }

    #[test]
    fn f32_mul_vector2() {
        let result = 1. * Vector2::new(2., 2.);
        assert_eq!(result.x, 2.);
        assert_eq!(result.y, 2.);
    }

    #[test]
    fn i32_mul_vector2() {
        let result = 1 * Vector2::new(2., 2.);
        assert_eq!(result.x, 2.);
        assert_eq!(result.y, 2.);
    }

    #[test]
    fn vector2_div_vector2() {
        let result = Vector2::new(2., 2.) / Vector2::new(2., 2.);
        assert_eq!(result.x, 1.);
        assert_eq!(result.y, 1.);
    }

    #[test]
    fn vector2_div_vector2i() {
        let result = Vector2::new(2., 2.) / Vector2i::new(2, 2);
        assert_eq!(result.x, 1.);
        assert_eq!(result.y, 1.);
    }

    #[test]
    fn vector2_div_f32() {
        let result = Vector2::new(2., 2.) / 2.;
        assert_eq!(result.x, 1.);
        assert_eq!(result.y, 1.);
    }

    #[test]
    fn vector2_div_i32() {
        let result = Vector2::new(2., 2.) / 2;
        assert_eq!(result.x, 1.);
        assert_eq!(result.y, 1.);
    }

    #[test]
    fn f32_div_vector2() {
        let result = 1. / Vector2::new(2., 2.);
        assert_eq!(result.x, 0.5);
        assert_eq!(result.y, 0.5);
    }

    #[test]
    fn i32_div_vector2() {
        let result = 1 / Vector2::new(2., 2.);
        assert_eq!(result.x, 0.5);
        assert_eq!(result.y, 0.5);
    }

    #[test]
    fn vector2_add_assign_vector2() {
        let mut result = Vector2::new(0., 0.);
        result += Vector2::new(1., 1.);
        assert_eq!(result.x, 1.);
        assert_eq!(result.y, 1.);
    }

    #[test]
    fn vector2_add_assign_vector2i() {
        let mut result = Vector2::new(0., 0.);
        result += Vector2i::new(1, 1);
        assert_eq!(result.x, 1.);
        assert_eq!(result.y, 1.);
    }

    #[test]
    fn vector2_add_assign_f32() {
        let mut result = Vector2::new(0., 0.);
        result += 1.;
        assert_eq!(result.x, 1.);
        assert_eq!(result.y, 1.);
    }

    #[test]
    fn vector2_add_assign_i32() {
        let mut result = Vector2::new(0., 0.);
        result += 1;
        assert_eq!(result.x, 1.);
        assert_eq!(result.y, 1.);
    }

    #[test]
    fn vector2_sub_assign_vector2() {
        let mut result = Vector2::new(2., 2.);
        result -= Vector2::new(1., 1.);
        assert_eq!(result.x, 1.);
        assert_eq!(result.y, 1.);
    }

    #[test]
    fn vector2_sub_assign_vector2i() {
        let mut result = Vector2::new(2., 2.);
        result -= Vector2i::new(1, 1);
        assert_eq!(result.x, 1.);
        assert_eq!(result.y, 1.);
    }

    #[test]
    fn vector2_sub_assign_f32() {
        let mut result = Vector2::new(2., 2.);
        result -= 1.;
        assert_eq!(result.x, 1.);
        assert_eq!(result.y, 1.);
    }

    #[test]
    fn vector2_sub_assign_i32() {
        let mut result = Vector2::new(2., 2.);
        result -= 1;
        assert_eq!(result.x, 1.);
        assert_eq!(result.y, 1.);
    }

    #[test]
    fn vector2_mul_assign_vector2() {
        let mut result = Vector2::new(2., 2.);
        result *= Vector2::new(1., 1.);
        assert_eq!(result.x, 2.);
        assert_eq!(result.y, 2.);
    }

    #[test]
    fn vector2_mul_assign_vector2i() {
        let mut result = Vector2::new(2., 2.);
        result *= Vector2i::new(1, 1);
        assert_eq!(result.x, 2.);
        assert_eq!(result.y, 2.);
    }

    #[test]
    fn vector2_mul_assign_f32() {
        let mut result = Vector2::new(2., 2.);
        result *= 1.;
        assert_eq!(result.x, 2.);
        assert_eq!(result.y, 2.);
    }

    #[test]
    fn vector2_mul_assign_i32() {
        let mut result = Vector2::new(2., 2.);
        result *= 1;
        assert_eq!(result.x, 2.);
        assert_eq!(result.y, 2.);
    }

    #[test]
    fn vector2_div_assign_vector2() {
        let mut result = Vector2::new(2., 2.);
        result /= Vector2::new(1., 1.);
        assert_eq!(result.x, 2.);
        assert_eq!(result.y, 2.);
    }

    #[test]
    fn vector2_div_assign_vector2i() {
        let mut result = Vector2::new(2., 2.);
        result /= Vector2i::new(1, 1);
        assert_eq!(result.x, 2.);
        assert_eq!(result.y, 2.);
    }

    #[test]
    fn vector2_div_assign_f32() {
        let mut result = Vector2::new(2., 2.);
        result /= 1.;
        assert_eq!(result.x, 2.);
        assert_eq!(result.y, 2.);
    }

    #[test]
    fn vector2_div_assign_i32() {
        let mut result = Vector2::new(2., 2.);
        result /= 1;
        assert_eq!(result.x, 2.);
        assert_eq!(result.y, 2.);
    }

    #[test]
    fn vector2i_add_vector2() {
        let result = Vector2i::new(0, 0) + Vector2::new(1., 1.);
        assert_eq!(result.x, 1);
        assert_eq!(result.y, 1);
    }

    #[test]
    fn vector2i_add_vector2i() {
        let result = Vector2i::new(0, 0) + Vector2i::new(1, 1);
        assert_eq!(result.x, 1);
        assert_eq!(result.y, 1);
    }

    #[test]
    fn vector2i_add_f32() {
        let result = Vector2i::new(0, 0) + 1.;
        assert_eq!(result.x, 1);
        assert_eq!(result.y, 1);
    }

    #[test]
    fn vector2i_add_i32() {
        let result = Vector2i::new(0, 0) + 1;
        assert_eq!(result.x, 1);
        assert_eq!(result.y, 1);
    }

    #[test]
    fn f32_add_vector2i() {
        let result = 1. + Vector2i::new(1, 1);
        assert_eq!(result.x, 2);
        assert_eq!(result.y, 2);
    }

    #[test]
    fn i32_add_vector2i() {
        let result = 1 + Vector2i::new(1, 1);
        assert_eq!(result.x, 2);
        assert_eq!(result.y, 2);
    }

    #[test]
    fn vector2i_sub_vector2() {
        let result = Vector2i::new(1, 1) - Vector2::new(1., 1.);
        assert_eq!(result.x, 0);
        assert_eq!(result.y, 0);
    }

    #[test]
    fn vector2i_sub_vector2i() {
        let result = Vector2i::new(1, 1) - Vector2i::new(1, 1);
        assert_eq!(result.x, 0);
        assert_eq!(result.y, 0);
    }

    #[test]
    fn vector2i_sub_f32() {
        let result = Vector2i::new(1, 1) - 1.;
        assert_eq!(result.x, 0);
        assert_eq!(result.y, 0);
    }

    #[test]
    fn vector2i_sub_i32() {
        let result = Vector2i::new(1, 1) - 1;
        assert_eq!(result.x, 0);
        assert_eq!(result.y, 0);
    }

    #[test]
    fn f32_sub_vector2i() {
        let result = 0. - Vector2i::new(1, 1);
        assert_eq!(result.x, -1);
        assert_eq!(result.y, -1);
    }

    #[test]
    fn i32_sub_vector2i() {
        let result = 0 - Vector2i::new(1, 1);
        assert_eq!(result.x, -1);
        assert_eq!(result.y, -1);
    }

    #[test]
    fn vector2i_mul_vector2() {
        let result = Vector2i::new(1, 1) * Vector2::new(2., 2.);
        assert_eq!(result.x, 2);
        assert_eq!(result.y, 2);
    }

    #[test]
    fn vector2i_mul_vector2i() {
        let result = Vector2i::new(1, 1) * Vector2i::new(2, 2);
        assert_eq!(result.x, 2);
        assert_eq!(result.y, 2);
    }

    #[test]
    fn vector2i_mul_f32() {
        let result = Vector2i::new(1, 1) * 2.;
        assert_eq!(result.x, 2);
        assert_eq!(result.y, 2);
    }

    #[test]
    fn vector2i_mul_i32() {
        let result = Vector2i::new(1, 1) * 2;
        assert_eq!(result.x, 2);
        assert_eq!(result.y, 2);
    }

    #[test]
    fn f32_mul_vector2i() {
        let result = -1. * Vector2i::new(1, 1);
        assert_eq!(result.x, -1);
        assert_eq!(result.y, -1);
    }

    #[test]
    fn i32_mul_vector2i() {
        let result = -1 * Vector2i::new(1, 1);
        assert_eq!(result.x, -1);
        assert_eq!(result.y, -1);
    }

    #[test]
    fn vector2i_div_vector2() {
        let result = Vector2i::new(2, 2) / Vector2::new(2., 2.);
        assert_eq!(result.x, 1);
        assert_eq!(result.y, 1);
    }

    #[test]
    fn vector2i_div_vector2i() {
        let result = Vector2i::new(2, 2) / Vector2i::new(2, 2);
        assert_eq!(result.x, 1);
        assert_eq!(result.y, 1);
    }

    #[test]
    fn vector2i_div_f32() {
        let result = Vector2i::new(2, 2) / 2.;
        assert_eq!(result.x, 1);
        assert_eq!(result.y, 1);
    }

    #[test]
    fn vector2i_div_i32() {
        let result = Vector2i::new(2, 2) / 2;
        assert_eq!(result.x, 1);
        assert_eq!(result.y, 1);
    }

    #[test]
    fn f32_div_vector2i() {
        let result = -4. / Vector2i::new(2, 2);
        assert_eq!(result.x, -2);
        assert_eq!(result.y, -2);
    }

    #[test]
    fn i32_div_vector2i() {
        let result = -4 / Vector2i::new(2, 2);
        assert_eq!(result.x, -2);
        assert_eq!(result.y, -2);
    }

    #[test]
    fn vector2i_add_assign_vector2() {
        let mut result = Vector2i::new(0, 0);
        result += Vector2::new(1., 1.);
        assert_eq!(result.x, 1);
        assert_eq!(result.y, 1);
    }

    #[test]
    fn vector2i_add_assign_vector2i() {
        let mut result = Vector2i::new(0, 0);
        result += Vector2i::new(1, 1);
        assert_eq!(result.x, 1);
        assert_eq!(result.y, 1);
    }

    #[test]
    fn vector2i_add_assign_f32() {
        let mut result = Vector2i::new(0, 0);
        result += 1.;
        assert_eq!(result.x, 1);
        assert_eq!(result.y, 1);
    }

    #[test]
    fn vector2i_add_assign_i32() {
        let mut result = Vector2i::new(0, 0);
        result += 1;
        assert_eq!(result.x, 1);
        assert_eq!(result.y, 1);
    }

    #[test]
    fn vector2i_sub_assign_vector2() {
        let mut result = Vector2i::new(2, 2);
        result -= Vector2::new(1., 1.);
        assert_eq!(result.x, 1);
        assert_eq!(result.y, 1);
    }

    #[test]
    fn vector2i_sub_assign_vector2i() {
        let mut result = Vector2i::new(2, 2);
        result -= Vector2i::new(1, 1);
        assert_eq!(result.x, 1);
        assert_eq!(result.y, 1);
    }

    #[test]
    fn vector2i_sub_assign_f32() {
        let mut result = Vector2i::new(2, 2);
        result -= 1.;
        assert_eq!(result.x, 1);
        assert_eq!(result.y, 1);
    }

    #[test]
    fn vector2i_sub_assign_i32() {
        let mut result = Vector2i::new(2, 2);
        result -= 1;
        assert_eq!(result.x, 1);
        assert_eq!(result.y, 1);
    }

    #[test]
    fn vector2i_mul_assign_vector2() {
        let mut result = Vector2i::new(2, 2);
        result *= Vector2::new(1., 1.);
        assert_eq!(result.x, 2);
        assert_eq!(result.y, 2);
    }

    #[test]
    fn vector2i_mul_assign_vector2i() {
        let mut result = Vector2i::new(2, 2);
        result *= Vector2i::new(1, 1);
        assert_eq!(result.x, 2);
        assert_eq!(result.y, 2);
    }

    #[test]
    fn vector2i_mul_assign_f32() {
        let mut result = Vector2i::new(2, 2);
        result *= 1.;
        assert_eq!(result.x, 2);
        assert_eq!(result.y, 2);
    }

    #[test]
    fn vector2i_mul_assign_i32() {
        let mut result = Vector2i::new(2, 2);
        result *= 1;
        assert_eq!(result.x, 2);
        assert_eq!(result.y, 2);
    }

    #[test]
    fn vector2i_div_assign_vector2() {
        let mut result = Vector2i::new(2, 2);
        result /= Vector2::new(1., 1.);
        assert_eq!(result.x, 2);
        assert_eq!(result.y, 2);
    }

    #[test]
    fn vector2i_div_assign_vector2i() {
        let mut result = Vector2i::new(2, 2);
        result /= Vector2i::new(1, 1);
        assert_eq!(result.x, 2);
        assert_eq!(result.y, 2);
    }

    #[test]
    fn vector2i_div_assign_f32() {
        let mut result = Vector2i::new(2, 2);
        result /= 1.;
        assert_eq!(result.x, 2);
        assert_eq!(result.y, 2);
    }

    #[test]
    fn vector2i_div_assign_i32() {
        let mut result = Vector2i::new(2, 2);
        result /= 1;
        assert_eq!(result.x, 2);
        assert_eq!(result.y, 2);
    }

    #[test]
    fn vector2_normalize() {
        let mut result = Vector2::new(1., 0.);
        result.normalize();
        assert_eq!(result.x, 1.);
        assert_eq!(result.y, 0.);
    }

    #[test]
    fn vector2_normalize_2() {
        let mut result = Vector2::new(0.2, 0.);
        result.normalize();
        assert_eq!(result.x, 1.);
        assert_eq!(result.y, 0.);
    }

    #[test]
    fn vector2_normalize_3() {
        let mut result = Vector2::new(0.2, 0.2);
        result.normalize();
        assert_eq!(result.x, 0.70710677);
        assert_eq!(result.y, 0.70710677);
    }

    #[test]
    fn vector2_normalized() {
        let result = Vector2::new(1., 0.).normalized();
        assert_eq!(result.x, 1.);
        assert_eq!(result.y, 0.);
    }

    #[test]
    fn vector2_normalized_2() {
        let result = Vector2::new(0.2, 0.).normalized();
        assert_eq!(result.x, 1.);
        assert_eq!(result.y, 0.);
    }

    #[test]
    fn vector2_normalized_3() {
        let result = Vector2::new(0.2, 0.2).normalized();
        assert_eq!(result.x, 0.70710677);
        assert_eq!(result.y, 0.70710677);
    }
}
