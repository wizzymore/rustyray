/// Mouse buttons
#[repr(i32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum MouseButton {
    /// Mouse button left
    Left,
    /// Mouse button right
    Right,
    /// Mouse button middle (pressed wheel)
    Middle,
    /// Mouse button side (advanced mouse device)
    Side,
    /// Mouse button extra (advanced mouse device)
    Extra,
    /// Mouse button forward (advanced mouse device)
    Forward,
    /// Mouse button back (advanced mouse device)
    Back,
}

/// Mouse cursor
#[repr(i32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum MouseCursor {
    /// Default pointer shape
    Default,
    /// Arrow shape
    Arrow,
    /// Text writing cursor shape
    Ibeam,
    /// Cross shape
    Crosshair,
    /// Pointing hand cursor
    PointingHand,
    /// Horizontal resize/move arrow shape
    ResizeEW,
    /// Vertical resize/move arrow shape
    ResizeNS,
    /// Top-left to bottom-right diagonal resize/move arrow shape
    ResizeNWSE,
    /// Top-right to bottom-left diagonal resize/move arrow shape
    ResizeNESW,
    /// The omnidirectional resize/move cursor shape
    ResizeAll,
    /// The operation-not-allowed shape
    NotAllowed,
}

/// Gamepad buttons
#[repr(i32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum GamepadButton {
    /// Unknown button, just for error checking
    UNKNOWN = 0,
    /// Gamepad left DPAD up button
    LeftFaceUp,
    /// Gamepad left DPAD right button
    LeftFaceRight,
    /// Gamepad left DPAD down button
    LeftFaceDown,
    /// Gamepad left DPAD left button
    LeftFaceLeft,
    /// Gamepad right button up (i.e. PS3: Triangle, Xbox: Y)
    RightFaceUp,
    /// Gamepad right button right (i.e. PS3: Circle, Xbox: B)
    RightFaceRight,
    /// Gamepad right button down (i.e. PS3: Cross, Xbox: A)
    RightFaceDown,
    /// Gamepad right button left (i.e. PS3: Square, Xbox: X)
    RightFaceLeft,
    /// Gamepad top/back trigger left (first), it could be a trailing button
    LeftTrigger1,
    /// Gamepad top/back trigger left (second), it could be a trailing button
    LeftTrigger2,
    /// Gamepad top/back trigger right (first), it could be a trailing button
    RightTrigger1,
    /// Gamepad top/back trigger right (second), it could be a trailing button
    RightTrigger2,
    /// Gamepad center buttons, left one (i.e. PS3: Select)
    MiddleLeft,
    /// Gamepad center buttons, middle one (i.e. PS3: PS, Xbox: XBOX)
    Middle,
    /// Gamepad center buttons, right one (i.e. PS3: Start)
    MiddleRight,
    /// Gamepad joystick pressed button left
    LeftThumb,
    /// Gamepad joystick pressed button right
    RightThumb,
}

/// Gamepad axis
#[repr(i32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum GamepadAxis {
    /// Gamepad left stick X axis
    LeftX,
    /// Gamepad left stick Y axis
    LeftY,
    /// Gamepad right stick X axis
    RightX,
    /// Gamepad right stick Y axis
    RightY,
    /// Gamepad back trigger left, pressure level: [1..-1]
    TriggerLeft,
    /// Gamepad back trigger right, pressure level: [1..-1]
    TriggerRight,
}

/// Material map index
#[repr(i32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum MaterialMap {
    /// Albedo material (same as: [MATERIAL_MAP_DIFFUSE])
    Albedo,
    /// Metalness material (same as: [MATERIAL_MAP_SPECULAR])
    Metalness,
    /// Normal material
    Normal,
    /// Roughness material
    Roughness,
    /// Ambient occlusion material
    Occlusion,
    /// Emission material
    Emission,
    /// Heightmap material
    Height,
    /// Cubemap material (**NOTE**: Uses GL_TEXTURE_CUBE_MAP)
    Cubemap,
    /// Irradiance material (**NOTE**: Uses GL_TEXTURE_CUBE_MAP)
    Irradiance,
    /// Prefilter material (**NOTE**: Uses GL_TEXTURE_CUBE_MAP)
    Prefilter,
    /// BRDF material
    BRDF,
}

pub const MATERIAL_MAP_DIFFUSE: MaterialMap = MaterialMap::Albedo;
pub const MATERIAL_MAP_SPECULAR: MaterialMap = MaterialMap::Metalness;

/// [crate::texture::Texture] parameters: filter mode
/// - **NOTE 1**: Filtering considers mipmaps if available in the texture
/// - **NOTE 2**: Filter is accordingly set for minification and magnification
#[repr(i32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum TextureFilter {
    /// No filter, just pixel approximation
    Point,
    /// Linear filtering
    Bilinear,
    /// Trilinear filtering (linear with mipmaps)
    Trilinear,
    /// Anisotropic filtering 4x
    Anisotropic4X,
    /// Anisotropic filtering 8x
    Anisotropic8X,
    /// Anisotropic filtering 16x
    Anisotropic16X,
}

/// [crate::texture::Texture] parameters: wrap mode
#[repr(i32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum TextureWrap {
    /// Repeats [crate::texture::Texture] in tiled mode
    Repeat,
    /// Clamps [crate::texture::Texture] to edge pixel in tiled mode
    Clamp,
    /// Mirrors and repeats the [crate::texture::Texture] in tiled mode
    MirrorRepeat,
    /// Mirrors and clamps to border the [crate::texture::Texture] in tiled mode
    MirrorClamp,
}

/// Font type, defines generation method
#[repr(i32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum FontType {
    /// Default font generation, anti-alised
    Default,
    /// Bitmap font generation, no anti-aliasing
    Bitmap,
    /// SDF font generation, requires external shader
    SDF,
}

/// Color blending modes (pre-defined)
#[repr(i32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum BlendMode {
    /// Blend textures considering alpha (default)
    Alpha,
    /// Blend textures adding colors
    Additive,
    /// Blend textures multiplying colors
    Multiplied,
    /// Blend textures adding colors (alternative)
    AddColors,
    /// Blend textures subtracting colors (alternative)
    SubtractColors,
    /// Blend premultiplied textures considering alpha
    AlphaPremultiply,
    /// Blend textures using custom src/dst factors (use rlSetBlendFactors())
    Custom,
    /// Blend textures using custom rgb/alpha separate src/dst factors (use rlSetBlendFactorsSeparate())
    CustomSeparate,
}

/// Gesture
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
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

/// Camera system modes
#[repr(i32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum CameraMode {
    /// Camera custom, controlled by user ([crate::ffi::update_camera] does nothing)
    Custom,
    /// Camera free mode
    Free,
    /// Camera orbital, around target, zoom supported
    Orbital,
    /// Camera first person
    FirstPerson,
    /// Camera third person
    ThridPerson,
}

/// Camera projection
#[repr(i32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum CameraProjection {
    /// Perspective projection
    Perspective,
    /// Orthographic projection
    Orthographic,
}

/// N-patch layout
#[repr(i32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum NPatchLayout {
    /// Npatch layout: 3x3 tiles
    NinePatch,
    /// Npatch layout: 1x3 tiles
    ThreePatchVertical,
    /// Npatch layout: 3x1 tiles
    ThreePatchHorizontal,
}

/// Trace log level
///
/// **NOTE**: Organized by priority level
#[repr(i32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum TraceLogLevel {
    /// Display all logs
    All,
    /// Trace logging, intended for interal use only
    Trace,
    /// Debug logging, used for internal debugging, it should be disabled on release builds
    Debug,
    /// Info logging, used for program execution info
    Info,
    /// Warning logging, used for recoverable failures
    Warning,
    /// Error logging, used for unrecoverable failures
    Error,
    /// Fatal logging, used to abort program: exit(EXIT_FAILURE)
    Fatal,
    /// Disable logging
    None,
}

/// Keyboard keys (US keyboard layout)
///
/// **NOTE**: Use [crate::ffi::get_key_pressed] to allow redefining
/// required keys for alternative layouts
#[repr(i32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum KeyboardKey {
    /// Key: NULL, used for no key pressed
    Null = 0,
    // Alphanumeric keys
    /// Key: `'`
    Apostrophe = 39,
    /// Key: `,`
    Comma = 44,
    /// Key: `-`
    Minus = 45,
    /// Key: `.`
    Period = 46,
    /// Key: `/`
    Slash = 47,
    /// Key: `0`
    Zero = 48,
    /// Key: `1`
    One = 49,
    /// Key: `2`
    Two = 50,
    /// Key: `3`
    Three = 51,
    /// Key: `4`
    Four = 52,
    /// Key: `5`
    Five = 53,
    /// Key: `6`
    Six = 54,
    /// Key: `7`
    Seven = 55,
    /// Key: `8`
    Eight = 56,
    /// Key: `9`
    Nine = 57,
    /// Key: `;`
    Semicolon = 59,
    /// Key: `=`
    Equal = 61,
    /// Key: `A` | `a`
    A = 65,
    /// Key: `B` | `b`
    B = 66,
    /// Key: `C` | `c`
    C = 67,
    /// Key: `D` | `d`
    D = 68,
    /// Key: `E` | `e`
    E = 69,
    /// Key: `F` | `f`
    F = 70,
    /// Key: `G` | `g`
    G = 71,
    /// Key: `H` | `h`
    H = 72,
    /// Key: `I` | `i`
    I = 73,
    /// Key: `J` | `j`
    J = 74,
    /// Key: `K` | `k`
    K = 75,
    /// Key: `L` | `l`
    L = 76,
    /// Key: `M` | `m`
    M = 77,
    /// Key: `N` | `n`
    N = 78,
    /// Key: `O` | `o`
    O = 79,
    /// Key: `P` | `p`
    P = 80,
    /// Key: `Q` | `q`
    Q = 81,
    /// Key: `R` | `r`
    R = 82,
    /// Key: `S` | `s`
    S = 83,
    /// Key: `T` | `t`
    T = 84,
    /// Key: `U` | `u`
    U = 85,
    /// Key: `V` | `v`
    V = 86,
    /// Key: `W` | `w`
    W = 87,
    /// Key: `X` | `x`
    X = 88,
    /// Key: `Y` | `y`
    Y = 89,
    /// Key: `Z` | `z`
    Z = 90,
    /// Key: `[`
    LeftBracket = 91,
    /// Key: `\`
    Backslash = 92,
    /// Key: `]`
    RightBracket = 93,
    /// Key  `\``
    Grave = 96,
    // Function keys
    /// Key: `Space`
    Space = 32,
    /// Key: `Esc`
    Escape = 256,
    /// Key: `Enter`
    Enter = 257,
    /// Key: `Tab`
    Tab = 258,
    /// Key: `Backspace`
    Backspace = 259,
    /// Key: `Ins`
    Insert = 260,
    /// Key: `Del`
    Delete = 261,
    /// Key: `Cursor right`
    Right = 262,
    /// Key: `Cursor left`
    Left = 263,
    /// Key: `Cursor down`
    Down = 264,
    /// Key: `Cursor up`
    Up = 265,
    /// Key: `Page up`
    PageUp = 266,
    /// Key: `Page down`
    PageDown = 267,
    /// Key: `Home`
    Home = 268,
    /// Key: `End`
    End = 269,
    /// Key: `Caps lock`
    CapsLock = 280,
    /// Key: `Scroll lock`
    ScrollLock = 281,
    /// Key: `Num lock`
    NumLock = 282,
    /// Key: `Print screen`
    PrintScreen = 283,
    /// Key: `Pause`
    Pause = 284,
    /// Key: `F1`
    F1 = 290,
    /// Key: `F2`
    F2 = 291,
    /// Key: `F3`
    F3 = 292,
    /// Key: `F4`
    F4 = 293,
    /// Key: `F5`
    F5 = 294,
    /// Key: `F6`
    F6 = 295,
    /// Key: `F7`
    F7 = 296,
    /// Key: `F8`
    F8 = 297,
    /// Key: `F9`
    F9 = 298,
    /// Key: `F10`
    F10 = 299,
    /// Key: `F11`
    F11 = 300,
    /// Key: `F12`
    F12 = 301,
    /// Key: `Shift left`
    LeftShift = 340,
    /// Key: `Control left`
    LeftControl = 341,
    /// Key: `Alt left`
    LeftAlt = 342,
    /// Key: `Super left`
    LeftSuper = 343,
    /// Key: `Shift right`
    RightShift = 344,
    /// Key: `Control right`
    RightControl = 345,
    /// Key: `Alt right`
    RightAlt = 346,
    /// Key: `Super right`
    RightSuper = 347,
    /// Key: `KB menu`
    KBMenu = 348,
    // Keypad keys
    /// Key: `Keypad 0`
    KP0 = 320,
    /// Key: `Keypad 1`
    KP1 = 321,
    /// Key: `Keypad 2`
    KP2 = 322,
    /// Key: `Keypad 3`
    KP3 = 323,
    /// Key: `Keypad 4`
    KP4 = 324,
    /// Key: `Keypad 5`
    KP5 = 325,
    /// Key: `Keypad 6`
    KP6 = 326,
    /// Key: `Keypad 7`
    KP7 = 327,
    /// Key: `Keypad 8`
    KP8 = 328,
    /// Key: `Keypad 9`
    KP9 = 329,
    /// Key: `Keypad .`
    KPDecimal = 330,
    /// Key: `Keypad /`
    KPDivide = 331,
    /// Key: `Keypad *`
    KPMultiply = 332,
    /// Key: `Keypad -`
    KPSubtract = 333,
    /// Key: `Keypad +`
    KPAdd = 334,
    /// Key: `Keypad Enter`
    KPEnter = 335,
    /// Key: `Keypad =`
    KPEqual = 336,
    // Android key button
    /// Key: `Android back button`
    Back = 4,
    /// Key: `Android menu button`
    Menu = 5,
    /// Key: `Android volume up button`
    VolumeUp = 24,
    /// Key: `Android volume down button`
    VolumeDown = 25,
}

/// System/Window config flags
///
/// **NOTE**: Every bit registers one state (use it with bit masks)
///
/// By default all flags are set to `0`
#[repr(i32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
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
