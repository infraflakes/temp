fn main() {
<<<<<<< HEAD
<<<<<<< HEAD
<<<<<<< HEAD
=======
>>>>>>> 2dd0a21 (Static major libraries)
    let pc = |name: &str| -> pkg_config::Library {
        let mut cfg = pkg_config::Config::new();
        cfg.cargo_metadata(false);
        cfg.statik(true);
        cfg.probe(name).unwrap()
    };

    let x11 = pc("x11");
    let xft = pc("xft");
    let xinerama = pc("xinerama");
    let xrender = pc("xrender");
    let fontconfig = pc("fontconfig");
    let freetype = pc("freetype2");
<<<<<<< HEAD

    let mut build = cc::Build::new();
    build
        .files(&[
            "c-src/core/wm.c",
            "c-src/core/events.c",
            "c-src/core/setup.c",
            "c-src/ui/bar.c",
            "c-src/ui/canvas.c",
            "c-src/ui/drw.c",
            "c-src/input/mouse.c",
            "c-src/input/workspace.c",
            "c-src/bridge/bridge.c",
            "c-src/util/util.c",
        ])
        .include("c-src/include")
        .include("c-src/bridge")
        .define("XINERAMA", None)
        .flag_if_supported("-Wno-unused-parameter")
        .flag_if_supported("-Wno-sign-compare");

    for lib in [&x11, &xft, &xinerama, &xrender, &fontconfig, &freetype] {
        for path in &lib.include_paths {
            build.include(path);
        }
    }

    build.compile("srwm");

    let out_path = std::path::PathBuf::from(std::env::var("OUT_DIR").unwrap());

    let mut bindgen_builder = bindgen::Builder::default()
        .header("c-src/bridge/bridge.h")
        .allowlist_function("srwm_.*")
        .allowlist_var("running");

    for lib in [&x11, &xft, &xinerama, &xrender, &fontconfig, &freetype] {
        for path in &lib.include_paths {
            bindgen_builder = bindgen_builder.clang_arg(format!("-I{}", path.display()));
        }
    }

    bindgen_builder
        .blocklist_function("srwm_handle_key")
        .blocklist_function("srwm_handle_mouse")
        .generate()
        .expect("Unable to generate bindings")
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");

    println!("cargo:rustc-link-search=native=/usr/lib");
    println!("cargo:rustc-link-search=native=/usr/lib/x86_64-linux-gnu");

    for name in [
        "X11",
        "Xinerama",
        "Xft",
        "Xrender",
        "fontconfig",
        "freetype",
    ] {
        println!("cargo:rustc-link-lib=static={}", name);
    }
    for name in [
        "xcb",
        "Xext",
        "Xau",
        "Xdmcp",
        "expat",
        "z",
        "bz2",
        "png16",
        "brotlidec",
        "brotlicommon",
    ] {
        println!("cargo:rustc-link-lib=static={}", name);
    }

=======
    // Phase 1: Compile C core into a static library
    cc::Build::new()
=======
    // Discover include/lib paths via pkg-config
    let x11 = pkg_config::probe_library("x11").unwrap();
    let xft = pkg_config::probe_library("xft").unwrap();
    let xinerama = pkg_config::probe_library("xinerama").unwrap();
    let xrender = pkg_config::probe_library("xrender").unwrap();
    let fontconfig = pkg_config::probe_library("fontconfig").unwrap();
    let freetype = pkg_config::probe_library("freetype2").unwrap();
=======
>>>>>>> 2dd0a21 (Static major libraries)

    let mut build = cc::Build::new();
    build
>>>>>>> 1a2036c (Dagger)
        .files(&[
            "c-src/wm.c",
            "c-src/bar.c",
            "c-src/bridge.c",
            "c-src/canvas.c",
            "c-src/drw.c",
            "c-src/events.c",
            "c-src/mouse.c",
            "c-src/setup.c",
            "c-src/util.c",
            "c-src/workspace.c",
        ])
        .include("c-src")
        .define("XINERAMA", None)
        .flag_if_supported("-Wno-unused-parameter")
        .flag_if_supported("-Wno-sign-compare");

    for lib in [&x11, &xft, &xinerama, &xrender, &fontconfig, &freetype] {
        for path in &lib.include_paths {
            build.include(path);
        }
    }

    build.compile("srwm");

    let out_path = std::path::PathBuf::from(std::env::var("OUT_DIR").unwrap());

    let mut bindgen_builder = bindgen::Builder::default()
        .header("c-src/bridge.h")
        .allowlist_function("srwm_.*")
        .allowlist_var("running");

    for lib in [&x11, &xft, &xinerama, &xrender, &fontconfig, &freetype] {
        for path in &lib.include_paths {
            bindgen_builder = bindgen_builder.clang_arg(format!("-I{}", path.display()));
        }
    }

    bindgen_builder
        .blocklist_function("srwm_handle_key")
        .blocklist_function("srwm_handle_mouse")
        .generate()
        .expect("Unable to generate bindings")
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");

    println!("cargo:rustc-link-search=native=/usr/lib");
    println!("cargo:rustc-link-search=native=/usr/lib/x86_64-linux-gnu");

    for name in [
        "X11",
        "Xinerama",
        "Xft",
        "Xrender",
        "fontconfig",
        "freetype",
    ] {
        println!("cargo:rustc-link-lib=static={}", name);
    }
    for name in [
        "xcb",
        "Xext",
        "Xau",
        "Xdmcp",
        "expat",
        "z",
        "bz2",
        "png16",
        "brotlidec",
        "brotlicommon",
    ] {
        println!("cargo:rustc-link-lib=static={}", name);
    }

<<<<<<< HEAD
    // Transitive deps (fontconfig/freetype pull these in)
    println!("cargo:rustc-link-lib=static=expat");
    println!("cargo:rustc-link-lib=static=z");
    println!("cargo:rustc-link-lib=static=bz2");
    println!("cargo:rustc-link-lib=static=png16");
    println!("cargo:rustc-link-lib=static=brotlidec");
    println!("cargo:rustc-link-lib=static=brotlicommon");

    // Rebuild if C sources change
>>>>>>> 9436516 (Scaffold)
=======
>>>>>>> 2dd0a21 (Static major libraries)
    println!("cargo:rerun-if-changed=c-src/");
}
