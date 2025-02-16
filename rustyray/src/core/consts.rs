use bitmask_enum::bitmask;

pub type KeyboardKey = rustyray_sys::consts::KeyboardKey;
pub type MaterialMap = rustyray_sys::consts::MaterialMap;
pub type BlendMode = rustyray_sys::consts::BlendMode;
pub type CameraMode = rustyray_sys::consts::CameraMode;
pub type CameraProjection = rustyray_sys::consts::CameraProjection;
pub type FontType = rustyray_sys::consts::FontType;
pub type GamepadAxis = rustyray_sys::consts::GamepadAxis;
pub type GamepadButton = rustyray_sys::consts::GamepadButton;
pub type MouseButton = rustyray_sys::consts::MouseButton;
pub type MouseCursor = rustyray_sys::consts::MouseCursor;

/// Gesture
#[bitmask(u32)]
pub enum Gesture {
    /// No gesture
    None,
    /// Tap gesture
    Tap,
    /// Double tap gesture
    DoubleTap,
    /// Hold gesture
    Hold,
    /// Drag gesture
    Drag,
    /// Swipe right gesture
    SwipeRight,
    /// Swipe left gesture
    SwipeLeft,
    /// Swipe up gesture
    SwipeUp,
    /// Swipe down gesture
    SwipeDown,
    /// Pinch in gesture
    PinchIn,
    /// Pinch out gesture
    PinchOut,
}

impl From<Gesture> for rustyray_sys::consts::Gesture {
    fn from(value: Gesture) -> Self {
        unsafe { std::mem::transmute(value) }
    }
}

impl From<rustyray_sys::consts::Gesture> for Gesture {
    fn from(value: rustyray_sys::consts::Gesture) -> Self {
        unsafe { std::mem::transmute(value) }
    }
}

/// System/Window config flags
///
/// **NOTE**: Every bit registers one state (use it with bit masks)
///
/// By default all flags are set to `0`
#[bitmask(i32)]
pub enum ConfigFlag {
    /// Set to try enabling V-Sync on GPU
    VsyncHint = 0x00000040,
    /// Set to run program in fullscreen
    FullscreenMode = 0x00000002,
    /// Set to allow resizable window
    WindowResizable = 0x00000004,
    /// Set to disable window decoration (frame and buttons)
    WindowUndecorated = 0x00000008,
    /// Set to hide window
    WindowHidden = 0x00000080,
    /// Set to minimize window (iconify)
    WindowMinimized = 0x00000200,
    /// Set to maximize window (expanded to monitor)
    WindowMaximized = 0x00000400,
    /// Set to window non focused
    WindowUnfocused = 0x00000800,
    /// Set to window always on top
    WindowTopmost = 0x00001000,
    /// Set to allow windows running while
    WindowAlwaysRun = 0x00000100,
    /// Set to allow transparent framebuffer
    WindowTransparent = 0x00000010,
    /// Set to support HighDPI
    WindowHighdpi = 0x00002000,
    /// Set to support mouse passthrough, only supported when [ConfigFlag::WindowUndecorated]
    WindowMousePassthrough = 0x00004000,
    /// Set to run program in borderless windowed mode
    BorderlessWindowedMode = 0x00008000,
    /// Set to try enabling MSAA 4X
    Msaa4xHint = 0x00000020,
    /// Set to try enabling interlaced video format (for V3D)
    InterlacedHint = 0x00010000,
}

impl From<ConfigFlag> for rustyray_sys::consts::ConfigFlag {
    fn from(value: ConfigFlag) -> Self {
        unsafe { std::mem::transmute(value) }
    }
}

impl From<rustyray_sys::consts::ConfigFlag> for ConfigFlag {
    fn from(value: rustyray_sys::consts::ConfigFlag) -> Self {
        unsafe { std::mem::transmute(value) }
    }
}
