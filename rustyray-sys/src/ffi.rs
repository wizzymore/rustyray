/**********************************************************************************************
*
*   raylib v5.5 - A simple and easy-to-use library to enjoy videogames programming (www.raylib.com)
*
*   FEATURES:
*       - NO external dependencies, all required libraries included with raylib
*       - Multiplatform: Windows, Linux, FreeBSD, OpenBSD, NetBSD, DragonFly,
*                        MacOS, Haiku, Android, Raspberry Pi, DRM native, HTML5.
*       - Written in plain C code (C99) in PascalCase/camelCase notation
*       - Hardware accelerated with OpenGL (1.1, 2.1, 3.3, 4.3, ES2, ES3 - choose at compile)
*       - Unique OpenGL abstraction layer (usable as standalone module): [rlgl]
*       - Multiple Fonts formats supported (TTF, OTF, FNT, BDF, Sprite fonts)
*       - Outstanding texture formats support, including compressed formats (DXT, ETC, ASTC)
*       - Full 3d support for 3d Shapes, Models, Billboards, Heightmaps and more!
*       - Flexible Materials system, supporting classic maps and PBR maps
*       - Animated 3D models supported (skeletal bones animation) (IQM, M3D, GLTF)
*       - Shaders support, including Model shaders and Postprocessing shaders
*       - Powerful math module for Vector, Matrix and Quaternion operations: [raymath]
*       - Audio loading and playing with streaming support (WAV, OGG, MP3, FLAC, QOA, XM, MOD)
*       - VR stereo rendering with configurable HMD device parameters
*       - Bindings to multiple programming languages available!
*
*   NOTES:
*       - One default Font is loaded on InitWindow()->LoadFontDefault() [core, text]
*       - One default Texture2D is loaded on rlglInit(), 1x1 white pixel R8G8B8A8 [rlgl] (OpenGL 3.3 or ES2)
*       - One default Shader is loaded on rlglInit()->rlLoadShaderDefault() [rlgl] (OpenGL 3.3 or ES2)
*       - One default RenderBatch is loaded on rlglInit()->rlLoadRenderBatch() [rlgl] (OpenGL 3.3 or ES2)
*
*   DEPENDENCIES (included):
*       [rcore][GLFW] rglfw (Camilla Löwy - github.com/glfw/glfw) for window/context management and input
*       [rcore][RGFW] rgfw (ColleagueRiley - github.com/ColleagueRiley/RGFW) for window/context management and input
*       [rlgl] glad/glad_gles2 (David Herberth - github.com/Dav1dde/glad) for OpenGL 3.3 extensions loading
*       [raudio] miniaudio (David Reid - github.com/mackron/miniaudio) for audio device/context management
*
*   OPTIONAL DEPENDENCIES (included):
*       [rcore] msf_gif (Miles Fogle) for GIF recording
*       [rcore] sinfl (Micha Mettke) for DEFLATE decompression algorithm
*       [rcore] sdefl (Micha Mettke) for DEFLATE compression algorithm
*       [rcore] rprand (Ramon Snatamaria) for pseudo-random numbers generation
*       [rtextures] qoi (Dominic Szablewski - https://phoboslab.org) for QOI image manage
*       [rtextures] stb_image (Sean Barret) for images loading (BMP, TGA, PNG, JPEG, HDR...)
*       [rtextures] stb_image_write (Sean Barret) for image writing (BMP, TGA, PNG, JPG)
*       [rtextures] stb_image_resize2 (Sean Barret) for image resizing algorithms
*       [rtextures] stb_perlin (Sean Barret) for Perlin Noise image generation
*       [rtext] stb_truetype (Sean Barret) for ttf fonts loading
*       [rtext] stb_rect_pack (Sean Barret) for rectangles packing
*       [rmodels] par_shapes (Philip Rideout) for parametric 3d shapes generation
*       [rmodels] tinyobj_loader_c (Syoyo Fujita) for models loading (OBJ, MTL)
*       [rmodels] cgltf (Johannes Kuhlmann) for models loading (glTF)
*       [rmodels] m3d (bzt) for models loading (M3D, https://bztsrc.gitlab.io/model3d)
*       [rmodels] vox_loader (Johann Nadalutti) for models loading (VOX)
*       [raudio] dr_wav (David Reid) for WAV audio file loading
*       [raudio] dr_flac (David Reid) for FLAC audio file loading
*       [raudio] dr_mp3 (David Reid) for MP3 audio file loading
*       [raudio] stb_vorbis (Sean Barret) for OGG audio loading
*       [raudio] jar_xm (Joshua Reisenauer) for XM audio module loading
*       [raudio] jar_mod (Joshua Reisenauer) for MOD audio module loading
*       [raudio] qoa (Dominic Szablewski - https://phoboslab.org) for QOA audio manage
*
*
*   LICENSE: zlib/libpng
*
*   raylib is licensed under an unmodified zlib/libpng license, which is an OSI-certified,
*   BSD-like license that allows static linking with closed source software:
*
*   Copyright (c) 2013-2024 Ramon Santamaria (@raysan5)
*
*   This software is provided "as-is", without any express or implied warranty. In no event
*   will the authors be held liable for any damages arising from the use of this software.
*
*   Permission is granted to anyone to use this software for any purpose, including commercial
*   applications, and to alter it and redistribute it freely, subject to the following restrictions:
*
*     1. The origin of this software must not be misrepresented; you must not claim that you
*     wrote the original software. If you use this software in a product, an acknowledgment
*     in the product documentation would be appreciated but is not required.
*
*     2. Altered source versions must be plainly marked as such, and must not be misrepresented
*     as being the original software.
*
*     3. This notice may not be removed or altered from any source distribution.
*
**********************************************************************************************/

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
    /// ```no_run
    /// use rustyray_ffi::{ffi::is_window_state, consts::ConfigFlag};
    ///
    /// unsafe { is_window_state(ConfigFlag::VsyncHint | ConfigFlag::FullscreenMode) };
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
    /// ```no_run
    /// use rustyray_ffi::{ffi::set_window_state, consts::ConfigFlag};
    ///
    /// unsafe { set_window_state(ConfigFlag::VsyncHint | ConfigFlag::FullscreenMode); }
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
    /// ```no_run
    /// use rustyray_ffi::{ffi::clear_window_state, consts::ConfigFlag};
    ///
    /// unsafe { clear_window_state(ConfigFlag::VsyncHint | ConfigFlag::FullscreenMode); }
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
    /// ```no_run
    /// use rustyray_ffi::{ffi::set_config_flags, consts::ConfigFlag};
    ///
    /// unsafe { set_config_flags(ConfigFlag::VsyncHint | ConfigFlag::FullscreenMode) }
    /// ```
    #[link_name = "SetConfigFlags"]
    pub fn set_config_flags(flags: ConfigFlag);
}
