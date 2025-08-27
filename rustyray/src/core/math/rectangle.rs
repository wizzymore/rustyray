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
    #[inline]
    pub fn size(&self) -> Vector2 {
        Vector2 {
            x: self.width,
            y: self.height,
        }
    }

    #[inline]
    pub fn collides_rect(&self, other: &Rectangle) -> bool {
        if (self.x < (other.x + other.width) && (self.x + self.width) > other.x)
            && (self.y < (other.y + other.height) && (self.y + self.height) > other.y)
        {
            return true;
        }

        false
    }

    #[inline]
    pub fn collides_point(&self, point: &Vector2) -> bool {
        if (point.x >= self.x)
            && (point.x < (self.x + self.width))
            && (point.y >= self.y)
            && (point.y < (self.y + self.height))
        {
            return true;
        }
        false
    }

    pub fn get_collision_rect(&self, other: &Rectangle) -> Rectangle {
        let mut overlap = Rectangle::new(0.0, 0.0, 0.0, 0.0);

        let left = if self.x > other.x { self.x } else { other.x };
        let right1 = self.x + self.width;
        let right2 = other.x + other.width;
        let right = if right1 < right2 { right1 } else { right2 };
        let top = if self.y > other.y { self.y } else { other.y };
        let bottom1 = self.y + self.height;
        let bottom2 = other.y + other.height;
        let bottom = if bottom1 < bottom2 { bottom1 } else { bottom2 };

        if (left < right) && (top < bottom) {
            overlap.x = left;
            overlap.y = top;
            overlap.width = right - left;
            overlap.height = bottom - top;
        }

        overlap
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
