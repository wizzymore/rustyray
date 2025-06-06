use crate::math::Vector2;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct Camera2D {
    pub offset: Vector2, // Camera offset (displacement from target)
    pub target: Vector2, // Camera target (rotation and zoom origin)
    pub rotation: f32,   // Camera rotation in degrees
    pub zoom: f32,       // Camera zoom (scaling), should be 1.0f by default
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
