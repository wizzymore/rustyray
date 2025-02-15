use std::env;
use std::path::{Path, PathBuf};
use std::process::exit;

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    let target = env::var("TARGET").expect("Cargo build scripts always have TARGET");
    let _ = env::current_dir()
        .unwrap()
        .join("raylib")
        .read_dir()
        .unwrap_or_else(|_| {
            eprintln!(" ERROR Could not find the raylib source code directory");
            exit(1)
        })
        .next()
        .unwrap_or_else(|| {
            eprintln!(
                " ERROR You need to initialize the submodules of this repository to compile the library."
            );
            exit(1)
        });

    // make sure cmake knows that it should bundle glfw in
    if target.contains("wasm") {
        if let Err(e) = env::var("EMCC_CFLAGS") {
            if e == std::env::VarError::NotPresent {
                panic!("\nYou must set the following environment variables yourself to compile for WASM. We are sorry for the inconvienence; this will be fixed in 5.1.0.\n{}{}\"\n",{
                    #[cfg(target_family = "windows")]
                    {"Paste this before executing the command: set EMCC_CFLAGS="}
                    #[cfg(not(target_family = "windows"))]
                    {"Prefix your command with this (you may want to make a build script for obvious reasons...): EMCC_CFLAGS="}
                },"\"-O3 -sUSE_GLFW=3 -sGL_ENABLE_GET_PROC_ADDRESS -sWASM=1 -sALLOW_MEMORY_GROWTH=1 -sWASM_MEM_MAX=512MB -sTOTAL_MEMORY=512MB -sABORTING_MALLOC=0 -sASYNCIFY -sFORCE_FILESYSTEM=1 -sASSERTIONS=1 -sERROR_ON_UNDEFINED_SYMBOLS=0 -sEXPORTED_RUNTIME_METHODS=ccallcwrap\"");
            } else {
                panic!("\nError regarding EMCC_CFLAGS: {:?}\n", e);
            }
        }
    }
    let (platform, platform_os) = platform_from_target(&target);

    // Donwload raylib source
    let src = cp_raylib();
    build_with_cmake(&src);

    link(platform, platform_os);

    // gen_rgui();
}

#[cfg(feature = "nobuild")]
fn build_with_cmake(_src_path: &str) {}

#[cfg(not(feature = "nobuild"))]
fn build_with_cmake(src_path: &str) {
    // CMake uses different lib directories on different systems.
    // I do not know how CMake determines what directory to use,
    // so we will check a few possibilities and use whichever is present.
    fn join_cmake_lib_directory(path: PathBuf) -> PathBuf {
        let possible_cmake_lib_directories = ["lib", "lib64", "lib32"];
        for lib_directory in &possible_cmake_lib_directories {
            let lib_path = path.join(lib_directory);
            if lib_path.exists() {
                return lib_path;
            }
        }
        path
    }

    let target = env::var("TARGET").expect("Cargo build scripts always have TARGET");

    let (platform, platform_os) = platform_from_target(&target);

    let mut conf = cmake::Config::new(src_path);
    let mut builder;
    let profile;
    #[cfg(debug_assertions)]
    {
        builder = conf.profile("Debug");
        builder = builder.define("CMAKE_BUILD_TYPE", "Debug");
        profile = "Debug";
    }

    #[cfg(not(debug_assertions))]
    {
        builder = conf.profile("Release");
        builder = builder.define("CMAKE_BUILD_TYPE", "Release");
        profile = "Release";
    }

    builder
        .define("BUILD_EXAMPLES", "OFF")
        .define("CMAKE_BUILD_TYPE", profile)
        // turn off until this is fixed
        .define("SUPPORT_BUSY_WAIT_LOOP", "OFF")
        .define("SUPPORT_FILEFORMAT_JPG", "ON")
        .define("RAYMATH_STATIC_INLINE", "ON");

    #[cfg(feature = "custom_frame_control")]
    {
        builder.define("SUPPORT_CUSTOM_FRAME_CONTROL", "ON");
    }

    // Enable wayland cmake flag if feature is specified
    #[cfg(feature = "wayland")]
    {
        builder.define("USE_WAYLAND", "ON");
        builder.define("USE_EXTERNAL_GLFW", "ON"); // Necessary for wayland support in my testing
    }

    // This seems redundant, but I felt it was needed incase raylib changes it's default
    #[cfg(not(feature = "wayland"))]
    builder.define("USE_WAYLAND", "OFF");

    // Scope implementing flags for forcing OpenGL version
    // See all possible flags at https://github.com/raysan5/raylib/wiki/CMake-Build-Options
    {
        #[cfg(feature = "opengl_33")]
        builder.define("OPENGL_VERSION", "3.3");

        #[cfg(feature = "opengl_21")]
        builder.define("OPENGL_VERSION", "2.1");

        // #[cfg(feature = "opengl_11")]
        // builder.define("OPENGL_VERSION", "1.1");

        #[cfg(feature = "opengl_es_20")]
        builder.define("OPENGL_VERSION", "ES 2.0");

        // Once again felt this was necessary incase a default was changed :)
        #[cfg(not(any(
            feature = "opengl_33",
            feature = "opengl_21",
            // feature = "opengl_11",
            feature = "opengl_es_20"
        )))]
        builder.define("OPENGL_VERSION", "OFF");
    }

    match platform {
        Platform::Desktop => conf.define("PLATFORM", "Desktop"),
        Platform::Web => conf
            .define("PLATFORM", "Web")
            .define("CMAKE_C_FLAGS", "-s ASYNCIFY"),
    };

    let dst = conf.build();
    let dst_lib = join_cmake_lib_directory(dst);
    // on windows copy the static library to the proper file name
    if platform_os == PlatformOS::Windows {
        if Path::new(&dst_lib.join("raylib.lib")).exists() {
            // DO NOTHING
        } else if Path::new(&dst_lib.join("raylib_static.lib")).exists() {
            std::fs::copy(
                dst_lib.join("raylib_static.lib"),
                dst_lib.join("raylib.lib"),
            )
            .expect("failed to create windows library");
        } else if Path::new(&dst_lib.join("libraylib_static.a")).exists() {
            std::fs::copy(
                dst_lib.join("libraylib_static.a"),
                dst_lib.join("libraylib.a"),
            )
            .expect("failed to create windows library");
        } else if Path::new(&dst_lib.join("libraylib.a")).exists() {
            // DO NOTHING
        } else {
            panic!("failed to create windows library");
        }
    } // on web copy libraylib.bc to libraylib.a
    if platform == Platform::Web && !Path::new(&dst_lib.join("libraylib.a")).exists() {
        std::fs::copy(dst_lib.join("libraylib.bc"), dst_lib.join("libraylib.a"))
            .expect("failed to create wasm library");
    }
    // println!("cmake build {}", c.display());
    println!("cargo:rustc-link-search=native={}", dst_lib.display());
}

#[cfg(feature = "nobuild")]
fn link(_platform: Platform, _platform_os: PlatformOS) {}

#[cfg(not(feature = "nobuild"))]
fn link(platform: Platform, platform_os: PlatformOS) {
    match platform_os {
        PlatformOS::Windows => {
            println!("cargo:rustc-link-lib=dylib=winmm");
            println!("cargo:rustc-link-lib=dylib=gdi32");
            println!("cargo:rustc-link-lib=dylib=user32");
            println!("cargo:rustc-link-lib=dylib=shell32");
        }
        PlatformOS::Linux => {
            // X11 linking
            #[cfg(not(feature = "wayland"))]
            {
                println!("cargo:rustc-link-search=/usr/local/lib");
                println!("cargo:rustc-link-lib=X11");
            }

            // Wayland linking
            #[cfg(feature = "wayland")]
            {
                println!("cargo:rustc-link-search=/usr/local/lib");
                println!("cargo:rustc-link-lib=wayland-client");
                println!("cargo:rustc-link-lib=glfw"); // Link against locally installed glfw
            }
        }
        PlatformOS::Osx => {
            println!("cargo:rustc-link-search=native=/usr/local/lib");
            println!("cargo:rustc-link-lib=framework=OpenGL");
            println!("cargo:rustc-link-lib=framework=Cocoa");
            println!("cargo:rustc-link-lib=framework=IOKit");
            println!("cargo:rustc-link-lib=framework=CoreFoundation");
            println!("cargo:rustc-link-lib=framework=CoreVideo");
        }
        _ => (),
    }
    if platform == Platform::Web {
        println!("cargo:rustc-link-lib=glfw");
    }

    println!("cargo:rustc-link-lib=static=raylib");
}

// cp_raylib copy raylib to an out dir
fn cp_raylib() -> String {
    let out = env::var("OUT_DIR").unwrap();
    let out = Path::new(&out); //.join("raylib_source");

    let mut options = fs_extra::dir::CopyOptions::new();
    options.skip_exist = true;
    fs_extra::dir::copy("raylib", out, &options)
        .unwrap_or_else(|_| panic!("failed to copy raylib source to {}", &out.to_string_lossy()));

    out.join("raylib").to_string_lossy().to_string()
}

fn platform_from_target(target: &str) -> (Platform, PlatformOS) {
    let platform = if target.contains("wasm") {
        Platform::Web
    } else {
        Platform::Desktop
    };

    let platform_os = if platform == Platform::Desktop {
        // Determine PLATFORM_OS in case PLATFORM_DESKTOP selected
        if env::var("OS")
            .unwrap_or("".to_owned())
            .contains("Windows_NT")
            || env::var("TARGET")
                .unwrap_or("".to_owned())
                .contains("windows")
        {
            // No uname.exe on MinGW!, but OS=Windows_NT on Windows!
            // ifeq ($(UNAME),Msys) -> Windows
            PlatformOS::Windows
        } else {
            let un: &str = &uname();
            match un {
                "Linux" => PlatformOS::Linux,
                "FreeBSD" => PlatformOS::Bsd,
                "OpenBSD" => PlatformOS::Bsd,
                "NetBSD" => PlatformOS::Bsd,
                "DragonFly" => PlatformOS::Bsd,
                "Darwin" => PlatformOS::Osx,
                _ => panic!("Unknown platform {}", uname()),
            }
        }
    } else {
        PlatformOS::Unknown
    };

    (platform, platform_os)
}

fn uname() -> String {
    use std::process::Command;
    String::from_utf8_lossy(
        &Command::new("uname")
            .output()
            .expect("failed to run uname")
            .stdout,
    )
    .trim()
    .to_owned()
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum Platform {
    Web,
    Desktop,
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum PlatformOS {
    Windows,
    Linux,
    Bsd,
    Osx,
    Unknown,
}
