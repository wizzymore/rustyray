pub mod color;
pub mod draw;
pub mod math;
pub mod window;

#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum MouseButton {
    MouseButtonLeft = 0,
    MouseButtonRight = 1,
    MouseButtonMiddle = 2,
    MouseButtonSide = 3,
    MouseButtonExtra = 4,
    MouseButtonForward = 5,
    MouseButtonBack = 6,
}

#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum MouseCursor {
    MouseCursorDefault = 0,
    MouseCursorArrow = 1,
    MouseCursorIbeam = 2,
    MouseCursorCrosshair = 3,
    MouseCursorPointingHand = 4,
    MouseCursorResizeEW = 5,
    MouseCursorResizeNS = 6,
    MouseCursorResizeNWSE = 7,
    MouseCursorResizeNESW = 8,
    MouseCursorResizeAll = 9,
    MouseCursorNotAllowed = 10,
}

impl From<MouseButton> for rustyray_sys::MouseButton {
    fn from(value: MouseButton) -> Self {
        unsafe { std::mem::transmute(value) }
    }
}

impl From<MouseCursor> for rustyray_sys::MouseCursor {
    fn from(value: MouseCursor) -> Self {
        unsafe { std::mem::transmute(value) }
    }
}
