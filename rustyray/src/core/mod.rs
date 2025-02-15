pub mod color;
pub mod draw;
pub use rustyray_math as math;
pub mod window;

#[repr(i32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum MouseButton {
    MouseButtonLeft,
    MouseButtonRight,
    MouseButtonMiddle,
    MouseButtonSide,
    MouseButtonExtra,
    MouseButtonForward,
    MouseButtonBack,
}

#[repr(i32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum MouseCursor {
    MouseCursorDefault,
    MouseCursorArrow,
    MouseCursorIbeam,
    MouseCursorCrosshair,
    MouseCursorPointingHand,
    MouseCursorResizeEW,
    MouseCursorResizeNS,
    MouseCursorResizeNWSE,
    MouseCursorResizeNESW,
    MouseCursorResizeAll,
    MouseCursorNotAllowed,
}

#[repr(i32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum ConfigFlags {
    FlagVsyncHint = 64,
    FlagFullscreenMode = 2,
    FlagWindowResizable = 4,
    FlagWindowUndecorated = 8,
    FlagWindowHidden = 128,
    FlagWindowMinimized = 512,
    FlagWindowMaximized = 1024,
    FlagWindowUnfocused = 2048,
    FlagWindowTopmost = 4096,
    FlagWindowAlwaysRun = 256,
    FlagWindowTransparent = 16,
    FlagWindowHighdpi = 8192,
    FlagWindowMousePassthrough = 16384,
    FlagBorderlessWindowedMode = 32768,
    FlagMsaa4xHint = 32,
    FlagInterlacedHint = 65536,
}

impl From<ConfigFlags> for u32 {
    fn from(value: ConfigFlags) -> Self {
        unsafe { std::mem::transmute(value) }
    }
}

impl From<MouseButton> for rustyray_ffi::MouseButton {
    fn from(value: MouseButton) -> Self {
        unsafe { std::mem::transmute(value) }
    }
}

impl From<MouseCursor> for rustyray_ffi::MouseCursor {
    fn from(value: MouseCursor) -> Self {
        unsafe { std::mem::transmute(value) }
    }
}
