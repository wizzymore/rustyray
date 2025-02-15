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

use libc::{c_char, c_double, c_float, c_int, c_uchar, c_uint, c_void};

use crate::{
    audio::{AudioCallback, AudioStream, Music, Sound, Wave},
    color::Color,
    consts::{ConfigFlag, MouseButton},
    rectangle::Rectangle,
    texture::{Image, RenderTexture, RenderTexture2D, Texture},
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
    /// use rustyray_sys::{ffi::is_window_state, consts::ConfigFlag};
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
    /// use rustyray_sys::{ffi::set_window_state, consts::ConfigFlag};
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
    /// use rustyray_sys::{ffi::clear_window_state, consts::ConfigFlag};
    ///
    /// unsafe { clear_window_state(ConfigFlag::VsyncHint | ConfigFlag::FullscreenMode); }
    /// ```
    #[link_name = "ClearWindowState"]
    pub fn clear_window_state(flags: ConfigFlag);
    /// Toggle window state: fullscreen/windowed, resizes monitor to match window resolution
    #[link_name = "ToggleFullscreen"]
    pub fn toggle_fullscreen();
    /// Toggle window state: borderless windowed, resizes window to match monitor resolution
    #[link_name = "ToggleBorderlessWindowed"]
    pub fn toggle_borderless_windowed();
    /// Set window state: maximized, if resizable
    #[link_name = "MaximizeWindow"]
    pub fn maximize_window();
    /// Set window state: minimized, if resizable
    #[link_name = "MinimizeWindow"]
    pub fn minimize_window();
    /// Set window state: not minimized/maximized
    #[link_name = "RestoreWindow"]
    pub fn restore_window();
    /// Set icon for window (single image, RGBA 32bit)
    #[link_name = "SetWindowIcon"]
    pub fn set_window_icon(image: Image);
    /// Set icon for window (multiple images, RGBA 32bit)
    #[link_name = "SetWindowIcons"]
    pub fn set_window_icons(images: *const Image);
    /// Set title for window
    #[link_name = "SetWindowTitle"]
    pub fn set_window_title(images: *const c_char);
    /// Set window position on screen
    #[link_name = "SetWindowPosition"]
    pub fn set_window_position(x: c_int, y: c_int);
    /// Set monitor for the current window
    #[link_name = "SetWindowMonitor"]
    pub fn set_window_monitor(monitor: c_int);
    /// Set window minimum dimensions (for [ConfigFlag::WindowResizable])
    #[link_name = "SetWindowMinSize"]
    pub fn set_window_min_size(width: c_int, height: c_int);
    /// Set window maximum dimensions (for [ConfigFlag::WindowResizable])
    #[link_name = "SetWindowMaxSize"]
    pub fn set_window_max_size(width: c_int, height: c_int);
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
    /// Get current render width (it considers HiDPI)
    #[link_name = "GetRenderWidth"]
    pub fn get_render_width() -> c_int;
    /// Get current render height (it considers HiDPI)
    #[link_name = "GetRenderHeight"]
    pub fn get_render_height() -> c_int;
    /// Get number of connected monitors
    #[link_name = "GetMonitorCount"]
    pub fn get_monitor_count() -> c_int;
    /// Get current monitor where window is placed
    #[link_name = "GetCurrentMonitor"]
    pub fn get_current_monitor() -> c_int;
    /// Get specified monitor position
    #[link_name = "GetMonitorPosition"]
    pub fn get_monitor_position(monitor: c_int) -> Vector2;
    /// Get specified monitor width (current video mode used by monitor)
    #[link_name = "GetMonitorWidth"]
    pub fn get_monitor_width(monitor: c_int) -> c_int;
    /// Get specified monitor height (current video mode used by monitor)
    #[link_name = "GetMonitorHeight"]
    pub fn get_monitor_height(monitor: c_int) -> c_int;
    /// Get specified monitor physical width in millimetres
    #[link_name = "GetMonitorPhysicalWidth"]
    pub fn get_monitor_physical_width(monitor: c_int) -> c_int;
    /// Get specified monitor physical height in millimetres
    #[link_name = "GetMonitorPhysicalHeight"]
    pub fn get_monitor_physical_height(monitor: c_int) -> c_int;
    /// Get specified monitor refresh rate
    #[link_name = "GetMonitorRefreshRate"]
    pub fn get_monitor_refresh_rate(monitor: c_int) -> c_int;
    /// Get position XY on monitor
    #[link_name = "GetWindowPosition"]
    pub fn get_window_position() -> Vector2;
    /// Get window scale DPI factor
    #[link_name = "GetWindowScaleDPI"]
    pub fn get_window_scale_dpi() -> Vector2;
    /// Get the human-readable, UTF-8 encoded name of the specified monitor
    #[link_name = "GetMonitorName"]
    pub fn get_monitor_name(monitor: c_int) -> *const c_char;
    /// Set clipboard text content
    #[link_name = "SetClipboardText"]
    pub fn set_clipboard_text(text: *const c_char);
    /// Get clipboard text content
    #[link_name = "GetClipboardText"]
    pub fn get_clipboard_text() -> *const c_char;
    /// Get clipboard image content
    #[link_name = "GetClipboardImage"]
    pub fn get_clipboard_image() -> Image;
    /// Enable waiting for events on [end_drawing], no automatic event polling
    #[link_name = "EnableEventWaiting"]
    pub fn enable_event_waiting();
    /// Disable waiting for events on [end_drawing], automatic event polling
    #[link_name = "DisableEventWaiting"]
    pub fn disable_event_waiting();
}

// Cursor-related functions
unsafe extern "C" {
    /// Shows cursor
    #[link_name = "ShowCursor"]
    pub fn show_cursor();
    /// Hides cursor
    #[link_name = "HideCursor"]
    pub fn hide_cursor();
    /// Check if cursor is not visible
    #[link_name = "IsCursorHidden"]
    pub fn is_cursor_hidden();
    /// Enables cursor (unlock cursor)
    #[link_name = "EnableCursor"]
    pub fn enable_cursor();
    /// Disabled cursor (lock cursor)
    #[link_name = "DisableCursor"]
    pub fn disable_cursor();
    /// Check if cursor is on the screen
    #[link_name = "IsCursorOnScreen"]
    pub fn is_cursor_on_screen();
}

// Drawing related functions
unsafe extern "C" {
    /// Set background color (framebuffer clear color)
    #[link_name = "ClearBackground"]
    pub fn clear_background(color: Color);
    /// Setup canvas (framebuffer) to start drawing
    #[link_name = "BeginDrawing"]
    pub fn begin_drawing();
    /// End canvas drawing and swap buffers (double buffering)
    #[link_name = "EndDrawing"]
    pub fn end_drawing();
    /// Begin drawing to render texture
    #[link_name = "BeginTextureMode"]
    pub fn begin_texture_mode(render_texture: RenderTexture2D);
    /// Ends drawing to render texture
    #[link_name = "EndTextureMode"]
    pub fn end_texture_mode();
}

// Texture loading functions
// Note: These function require GPU access
unsafe extern "C" {
    /// Load texture from file into GPU memory (VRAM)
    #[link_name = "LoadTexture"]
    pub fn load_texture(path: *const c_char) -> Texture;
    /// Load texture from image data
    #[link_name = "LoadTextureFromImage"]
    pub fn load_texture_from_image(image: Image) -> Texture;
    /// Load texture for rendering (framebuffer)
    #[link_name = "LoadRenderTexture"]
    pub fn load_render_texture(width: c_int, height: c_int) -> RenderTexture;
    /// Check if a texture is valid (loaded in GPU)
    #[link_name = "IsTextureValid"]
    pub fn is_texture_valid(texture: Texture) -> bool;
    /// Unload texture from GPU memory (VRAM)
    #[link_name = "UnloadTexture"]
    pub fn unload_texture(texture: Texture);
    /// Check if a render texture is valid (loaded in GPU)
    #[link_name = "IsRenderTextureValid"]
    pub fn is_render_texture_valid(target: RenderTexture) -> bool;
    /// Unload render texture from GPU memory (VRAM)
    #[link_name = "UnloadRenderTexture"]
    pub fn unload_render_texture(render_texture: RenderTexture);
    /// Update GPU texture with new data
    #[link_name = "UpdateTexture"]
    pub fn update_texture(texture: Texture, pixels: *const c_void);
    /// Update GPU texture rectangle with new data
    #[link_name = "UpdateTextureRec"]
    pub fn update_texture_rec(texture: Texture, rec: Rectangle, pixels: *const c_void);
}

// Texture configuration function
unsafe extern "C" {
    /// Generate GPU mipmaps for a texture
    #[link_name = "GenTextureMipmaps"]
    pub fn gen_texture_mipmaps(texture: *mut Texture);
    /// Set texture scaling filter mode
    #[link_name = "SetTextureFilter"]
    pub fn set_texture_filter(texture: Texture, filter: c_int);
    /// Set texture wrapping mode
    #[link_name = "SetTextureWrap"]
    pub fn set_texture_wrap(texture: Texture, wrap: c_int);
}

// Texture drawing functions
unsafe extern "C" {
    /// Draw a [Texture]
    #[link_name = "DrawTexture"]
    pub fn draw_texture(texture: Texture, pos_x: c_int, pos_y: c_int, tint: Color);
    /// Draw a [Texture] with position defined as [Vector2]
    #[link_name = "DrawTextureV"]
    pub fn draw_texture_v(texture: Texture, pos: Vector2, tint: Color);
    /// Draw a [Texture] with extended parameters
    #[link_name = "DrawTextureEx"]
    pub fn draw_texture_ex(
        texture: Texture,
        pos: Vector2,
        rotation: c_float,
        scale: c_float,
        tint: Color,
    );
    /// Draw a part of a [Texture] defined by a [Rectangle]
    #[link_name = "DrawTextureRec"]
    pub fn draw_texture_rec(texture: Texture, source: Rectangle, position: Vector2, tint: Color);
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
    // TODO: Add draw_texture_npatch when NPatchInfo is implemented
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

// Text font info functions
unsafe extern "C" {
    /// Measure string width for default font
    #[link_name = "MeasureText"]
    pub fn measure_text(text: *const c_char, font_size: c_int);
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
    /// use rustyray_sys::{ffi::set_config_flags, consts::ConfigFlag};
    ///
    /// unsafe { set_config_flags(ConfigFlag::VsyncHint | ConfigFlag::FullscreenMode) }
    /// ```
    #[link_name = "SetConfigFlags"]
    pub fn set_config_flags(flags: ConfigFlag);
}

// Audio device management functions
unsafe extern "C" {
    /// Initialize audio device and context
    #[link_name = "InitAudioDevice"]
    pub fn init_audio_device();
    /// Close the audio device and context
    #[link_name = "CloseAudioDevice"]
    pub fn close_audio_device();
    /// Check if audio device has been initialized successfully
    #[link_name = "IsAudioDeviceReady"]
    pub fn is_audio_device_ready() -> bool;
    /// Set master volume (listener)
    #[link_name = "SetMasterVolume"]
    pub fn set_master_volume(volume: c_float);
    /// Get master volume (listener)
    #[link_name = "GetMasterVolume"]
    pub fn get_master_volume() -> c_float;
}

// Wave/Sound loading/unloading functions
unsafe extern "C" {
    /// Load wave data from file
    #[link_name = "LoadWave"]
    pub fn load_wave(file_name: *const c_char) -> Wave;
    /// Load wave from memory buffer, file_type refers to extension: i.e. `.wav`
    #[link_name = "LoadWaveFromMemory"]
    pub fn load_wave_from_memory(
        file_type: *const c_char,
        file_data: *const c_uchar,
        data_size: c_int,
    ) -> Wave;
    /// Checks if wave data is valid (data loaded and parameters)
    #[link_name = "IsWaveValid"]
    pub fn is_wave_valid(wave: Wave) -> bool;
    /// Load sound from file
    #[link_name = "LoadSound"]
    pub fn load_sound(file_name: *const c_char) -> Sound;
    /// Load sound from wave data
    #[link_name = "LoadSoundFromWave"]
    pub fn load_sound_from_wave(wave: Wave) -> Sound;
    /// Create a new sound that shares the same sample data as the source sound, does not own the sound data
    #[link_name = "LoadSoundAlias"]
    pub fn load_sound_alias(source: Sound) -> Sound;
    /// Checks if sound is valid (data loaded and buffers initialized)
    #[link_name = "IsSoundValid"]
    pub fn is_sound_valid(sound: Sound) -> bool;
    /// Update sound buffer with new data
    #[link_name = "UpdateSound"]
    pub fn update_sound(sound: Sound, data: *const c_void, sample_count: c_int);
    /// Unload wave data
    #[link_name = "UnloadWave"]
    pub fn unload_wave(wave: Wave);
    /// Unload sound
    #[link_name = "UnloadSound"]
    pub fn unload_sound(sound: Sound);
    /// Unload a sound alias (does not deallocate sample data)
    #[link_name = "UnloadSoundAlias"]
    pub fn unload_sound_alias(alias: Sound);
    /// Export wave data to file, returns true on success
    #[link_name = "ExportWave"]
    pub fn export_wave(wave: Wave, file_name: *const c_char) -> bool;
    /// Export wave sample data to code (.h), returns true on success
    #[link_name = "ExportWaveAsCode"]
    pub fn export_wave_as_code(wave: Wave, file_name: *const c_char) -> bool;
}

// Wave/Sound management functions
unsafe extern "C" {
    /// Play a sound
    #[link_name = "PlaySound"]
    pub fn play_sound(sound: Sound);
    /// Stop playing a sound
    #[link_name = "StopSound"]
    pub fn stop_sound(sound: Sound);
    /// Pause a sound
    #[link_name = "PauseSound"]
    pub fn pause_sound(sound: Sound);
    /// Resume a paused sound
    #[link_name = "ResumeSound"]
    pub fn resume_sound(sound: Sound);
    /// Check if a sound is currently playing
    #[link_name = "IsSoundPlaying"]
    pub fn is_sound_playing(sound: Sound) -> bool;
    /// Set volume for a sound (1.0 is max level)
    #[link_name = "SetSoundVolume"]
    pub fn set_sound_volume(sound: Sound, volume: c_float);
    /// Set pitch for a sound (1.0 is base level)
    #[link_name = "SetSoundPitch"]
    pub fn set_sound_pitch(sound: Sound, pitch: c_float);
    /// Set pan for a sound (0.5 is center)
    #[link_name = "SetSoundPan"]
    pub fn set_sound_pan(sound: Sound, pan: c_float);
    /// Copy the wave to a new wave
    #[link_name = "WaveCopy"]
    pub fn wave_copy(wave: Wave);
    /// Crop a wave to defined frames range
    #[link_name = "WaveCrop"]
    pub fn wave_crop(wave: *mut Wave, init_frame: c_int, final_frame: c_int);
    /// Convert wave data to desired format
    #[link_name = "WaveFormat"]
    pub fn wave_format(wave: *mut Wave, sample_rate: c_int, sample_size: c_int, channels: c_int);
    /// Load samples data from wave as a 32bit float data array
    #[link_name = "LoadWaveSamples"]
    pub fn load_wave_samples(wave: Wave) -> *const c_float;
    /// Unload samples data loaded with LoadWaveSamples()
    #[link_name = "UnloadWaveSamples"]
    pub fn unload_wave_samples(samples: *const c_float);
}

unsafe extern "C" {
    /// Load music stream from file
    #[link_name = "LoadMusicStream"]
    pub fn load_music_stream(file_name: *const c_char) -> Music;
    /// Load music stream from file
    #[link_name = "LoadMusicStreamFromMemory"]
    pub fn load_music_stream_from_memory(
        file_type: *const c_char,
        data: *const c_uchar,
        data_size: c_int,
    ) -> Music;
    /// Checks if a music stream is valid (context and buffers initialized)
    #[link_name = "IsMusicValid"]
    pub fn is_music_valid(music: Music) -> bool;
    /// Unload music stream
    #[link_name = "UnloadMusicStream"]
    pub fn unload_music_stream(music: Music);
    /// Start music playing
    #[link_name = "PlayMusicStream"]
    pub fn play_music_stream(music: Music);
    /// Checks if music is playing
    #[link_name = "IsMusicStreamPlaying"]
    pub fn is_music_stream_playing(music: Music) -> bool;
    /// Updates buffers for music streaming
    #[link_name = "UpdateMusicStream"]
    pub fn update_music_stream(music: Music);
    /// Stop music playing
    #[link_name = "StopMusicStream"]
    pub fn stop_music_stream(music: Music);
    /// Pause music playing
    #[link_name = "PauseMusicStream"]
    pub fn pause_music_stream(music: Music);
    /// Resume playing paused music
    #[link_name = "ResumeMusicStream"]
    pub fn resume_music_stream(music: Music);
    /// Seek music to a position (in seconds)
    #[link_name = "SeekMusicStream"]
    pub fn seek_music_stream(music: Music, position: c_float);
    /// Set volume for music (1.0 is max level)
    #[link_name = "SetMusicVolume"]
    pub fn set_music_volume(music: Music, volume: c_float);
    /// Set pitch for music (1.0 is base level)
    #[link_name = "SetMusicPitch"]
    pub fn set_music_pitch(music: Music, pitch: c_float);
    /// Set pan for music (0.5 is center)
    #[link_name = "SetMusicPan"]
    pub fn set_music_pan(music: Music, pan: c_float);
    /// Get music time length (in seconds)
    #[link_name = "GetMusicTimeLength"]
    pub fn get_music_time_length(music: Music) -> c_float;
    /// Get current music time played (in seconds)
    #[link_name = "GetMusicTimePlayed"]
    pub fn get_music_time_played(music: Music) -> c_float;
}

// AudioStream management functions
unsafe extern "C" {
    /// Load audio stream (to stream raw audio pcm data)
    #[link_name = "LoadAudioStream"]
    pub fn load_audio_stream(
        sample_rate: c_uint,
        sample_size: c_uint,
        channels: c_uint,
    ) -> AudioStream;
    /// Checks if an audio stream is valid (buffers initialized)
    #[link_name = "IsAudioStreamValid"]
    pub fn is_audio_stream_valid(stream: AudioStream) -> bool;
    /// Unload audio stream and free memory
    #[link_name = "UnloadStreamValid"]
    pub fn unload_stream_valid(stream: AudioStream);
    /// Update audio stream buffers with data
    #[link_name = "UpdateStreamValid"]
    pub fn update_stream_valid(stream: AudioStream, data: *const c_void, frame_count: c_int);
    /// Check if any audio stream buffers requires refill
    #[link_name = "IsAudioStreamProcessed"]
    pub fn is_audio_stream_processed(stream: AudioStream) -> bool;
    /// Play audio stream
    #[link_name = "PlayAudioStream"]
    pub fn play_audio_stream(stream: AudioStream);
    /// Pause audio stream
    #[link_name = "PauseAudioStream"]
    pub fn pause_audio_stream(stream: AudioStream);
    /// Resume audio stream
    #[link_name = "ResumeAudioStream"]
    pub fn resume_audio_stream(stream: AudioStream);
    /// Check if audio stream is playing
    #[link_name = "IsAudioStreamPlaying"]
    pub fn is_audio_stream_playing(stream: AudioStream) -> bool;
    /// Stop audio stream
    #[link_name = "StopAudioStream"]
    pub fn stop_audio_stream(stream: AudioStream);
    /// Set volume for audio stream (1.0 is max level)
    #[link_name = "SetAudioStreamVolume"]
    pub fn set_audio_stream_volume(stream: AudioStream, volume: c_float);
    /// Set pitch for audio stream (1.0 is base level)
    #[link_name = "SetAudioStreamPitch"]
    pub fn set_audio_stream_pitch(stream: AudioStream, pitch: c_float);
    /// Set pan for audio stream (0.5 is centered)
    #[link_name = "SetAudioStreamPan"]
    pub fn set_audio_stream_pan(stream: AudioStream, pan: c_float);
    /// Default size for new audio streams
    #[link_name = "SetAudioStreamBufferSizeDefault"]
    pub fn set_audio_stream_buffer_size_default(size: c_int);
    /// Audio thread callback to request new data
    #[link_name = "SetAudioStreamCallback"]
    pub fn set_audio_stream_callback(stream: AudioStream, callback: AudioCallback);

    /// Attach audio stream processor to stream, receives the samples as 'float'
    #[link_name = "AttachAudioStreamProcessor"]
    pub fn attach_audio_stream_processor(stream: AudioStream, processor: AudioCallback);
    /// Detach audio stream processor from stream
    #[link_name = "DetachAudioStreamProcessor"]
    pub fn detach_audio_stream_processor(stream: AudioStream, processor: AudioCallback);
    /// Attach audio stream processor to the entire audio pipeline, receives the samples as 'float'
    #[link_name = "AttachAudioMixedProcessor"]
    pub fn attach_audio_mixed_processor(processor: AudioCallback);
    /// Detach audio stream processor from the entire audio pipeline
    #[link_name = "DetachAudioMixedProcessor"]
    pub fn detach_audio_mixed_processor(processor: AudioCallback);
}
