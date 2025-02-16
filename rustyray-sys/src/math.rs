#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Rectangle {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct Vector2 {
    pub x: f32,
    pub y: f32,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vector3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

/// Vector4, 4 components
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vector4 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}

/// Quaternion, 4 components (Vector4 alias)
pub type Quaternion = Vector4;

/// Matrix, 4x4 components, column major, OpenGL style, right-handed
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Matrix {
    m0: f32,
    m4: f32,
    m8: f32,
    m12: f32, // Matrix first row (4 components)
    m1: f32,
    m5: f32,
    m9: f32,
    m13: f32, // Matrix second row (4 components)
    m2: f32,
    m6: f32,
    m10: f32,
    m14: f32, // Matrix third row (4 components)
    m3: f32,
    m7: f32,
    m11: f32,
    m15: f32, // Matrix fourth row (4 components)
}
