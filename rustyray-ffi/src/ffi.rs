use libc::{c_char, c_double, c_float, c_int};

use crate::{
    color::Color,
    consts::{ConfigFlag, MouseButton},
    rectangle::Rectangle,
    texture::{RenderTexture, RenderTexture2D, Texture},
    vector::Vector2,
};

// Window-related functions
unsafe extern "C" {
    /// Initialize window and OpenGL context
    #[link_name = "InitWindow"]
    pub fn init_window(width: c_int, height: c_int, title: *const c_char);
    /// Close window and unload OpenGL context
    #[link_name = "CloseWindow"]
    pub fn close_window();
    /// Check if application should close (KEY_ESCAPE pressed or windows close icon clicked)
    #[link_name = "WindowShouldClose"]
    pub fn window_should_close() -> bool;
    /// Check if window has been initialized successfully
    #[link_name = "IsWindowReady"]
    pub fn is_window_ready() -> bool;
    /// Check if window is currently fullscreen
    #[link_name = "IsWindowFullscreen"]
    pub fn is_window_fullscreen() -> bool;
    /// Check if window is currently hidden
    #[link_name = "IsWindowHidden"]
    pub fn is_window_hidden() -> bool;
    /// Check if window is currently minimized
    #[link_name = "IsWindowMinimized"]
    pub fn is_window_minimized() -> bool;
    /// Check if window is currently maximized
    #[link_name = "IsWindowMaximized"]
    pub fn is_window_maximized() -> bool;
    /// Check if window is currently focused
    #[link_name = "IsWindowFocused"]
    pub fn is_window_focused() -> bool;
    /// Check if window has been resized last frame
    #[link_name = "IsWindowResized"]
    pub fn is_window_resized() -> bool;
    /// Check if one specific window flag is enabled
    ///
    /// Flags should be of values defined in [ConfigFlag].
    ///
    /// Use this value as a bitmask.
    ///
    /// # Examples
    /// ```
    /// use rustyray_ffi::{is_window_state, ConfigFlag};
    ///
    /// is_window_state(ConfigFlag::VsyncHint | ConfigFlag::FullscreenMode)
    /// ```
    #[link_name = "IsWindowState"]
    pub fn is_window_state(flags: ConfigFlag) -> bool;
    /// Set window configuration state using flags
    ///
    /// Flags should be of values defined in [ConfigFlag].
    ///
    /// Use this value as a bitmask.
    ///
    /// # Examples
    /// ```
    /// use rustyray_ffi::{set_window_state, ConfigFlag};
    ///
    /// set_window_state(ConfigFlag::VsyncHint | ConfigFlag::FullscreenMode)
    /// ```
    #[link_name = "SetWindowState"]
    pub fn set_window_state(flags: ConfigFlag);
    /// Clear window configuration state flags
    ///
    /// Flags should be of values defined in [ConfigFlag].
    ///
    /// Use this value as a bitmask.
    ///
    /// # Examples
    /// ```
    /// use rustyray_ffi::{clear_window_state, ConfigFlag};
    ///
    /// clear_window_state(ConfigFlag::VsyncHint | ConfigFlag::FullscreenMode)
    /// ```
    #[link_name = "ClearWindowState"]
    pub fn clear_window_state(flags: ConfigFlag);
    /// Set window dimension
    #[link_name = "SetWindowSize"]
    pub fn set_window_size(width: c_int, height: c_int);
    /// Set window opacity [0.0..1.0]
    #[link_name = "SetWindowOpacity"]
    pub fn set_window_opacity(opacity: c_float);
    /// Set window focused
    #[link_name = "SetWindowFocused"]
    pub fn set_window_focused();
    /// Get current screen width
    #[link_name = "GetScreenWidth"]
    pub fn get_screen_width() -> c_int;
    /// Get current screen height
    #[link_name = "GetScreenHeight"]
    pub fn get_screen_height() -> c_int;
}

// Drawing related functions
unsafe extern "C" {
    /// Set background color (framebuffer clear color)
    #[link_name = "ClearBackground"]
    pub fn clear_background(color: Color);
    #[link_name = "BeginDrawing"]
    pub fn begin_drawing();
    #[link_name = "EndDrawing"]
    pub fn end_drawing();
    #[link_name = "BeginTextureMode"]
    pub fn begin_texture_mode(render_texture: RenderTexture2D);
    #[link_name = "EndTextureMode"]
    pub fn end_texture_mode();
    #[link_name = "LoadTexture"]
    pub fn load_texture(path: *const c_char) -> Texture;
    #[link_name = "LoadRenderTexture"]
    pub fn load_render_texture(width: c_int, height: c_int) -> RenderTexture;
    #[link_name = "UnloadTexture"]
    pub fn unload_texture(texture: Texture);
    #[link_name = "UnloadRenderTexture"]
    pub fn unload_render_texture(render_texture: RenderTexture);
}

// Text drawing functions
unsafe extern "C" {
    /// Draw current FPS
    #[link_name = "DrawFPS"]
    pub fn draw_fps(pos_x: c_int, pos_y: c_int);
    /// Draw text (using default font)
    #[link_name = "DrawText"]
    pub fn draw_text(
        text: *const c_char,
        pos_x: c_int,
        pos_y: c_int,
        font_size: c_int,
        color: Color,
    );
}

// Texture drawing functions
unsafe extern "C" {
    /// Draw a [Texture]
    #[link_name = "DrawTexture"]
    pub fn draw_texture(texture: Texture, pos_x: c_int, pos_y: c_int, tint: Color);
    /// Draw a part of a [Texture] defined by a [Rectangle] with 'pro' parameters
    #[link_name = "DrawTexturePro"]
    pub fn draw_texture_pro(
        texture: Texture,
        source: Rectangle,
        dest: Rectangle,
        origin: Vector2,
        rotation: c_float,
        tint: Color,
    );
}

// Basic shapes drawing functions
unsafe extern "C" {
    /// Draw a color-filled rectangle
    #[link_name = "DrawRectangleRec"]
    pub fn draw_rectangle_rec(rec: Rectangle, color: Color);
}

// Input-related functions: mouse
unsafe extern "C" {
    /// Check if a mouse button is beening pressed
    #[link_name = "IsMouseButtonDown"]
    pub fn is_mouse_button_down(button: MouseButton) -> bool;
    /// Get mouse position X
    #[link_name = "GetMouseX"]
    pub fn get_mouse_x() -> c_int;
    /// Get mouse position Y
    #[link_name = "GetMouseY"]
    pub fn get_mouse_y() -> c_int;
    /// Get mouse position XY
    #[link_name = "GetMousePosition"]
    pub fn get_mouse_position() -> Vector2;
}

// Timing-related functions
unsafe extern "C" {
    /// Set target FPS (maximum)
    #[link_name = "SetTargetFPS"]
    pub fn set_target_fps(fps: c_int);
    /// Get time in seconds for last frame drawn (delta time)
    #[link_name = "GetFrameTime"]
    pub fn get_frame_time() -> c_float;
    /// Get elapsed time in seconds since InitWindow()
    #[link_name = "GetTime"]
    pub fn get_time() -> c_double;
    /// Get current FPS
    #[link_name = "GetFPS"]
    pub fn get_fps() -> c_int;
}

// Misc functions
unsafe extern "C" {
    /// Setup init configuration flags
    ///
    /// Flags should be of values defined in [ConfigFlag].
    ///
    /// Use this value as a bitmask.
    ///
    /// # Examples
    /// ```
    /// use rustyray_ffi::{set_config_flags, ConfigFlag};
    ///
    /// set_config_flags(ConfigFlag::FlagVsyncHint | ConfigFlag::FlagFullscreenMode)
    /// ```
    #[link_name = "SetConfigFlags"]
    pub fn set_config_flags(flags: ConfigFlag);
}
