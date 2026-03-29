fn main() {
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
    // Build srcom compositor if source exists
    let compositor_dir =
        std::path::PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").unwrap()).join("compositor");

    if compositor_dir.join("meson.build").exists() {
        let out = std::path::PathBuf::from(std::env::var("OUT_DIR").unwrap());
        let build_dir = out.join("srcom-build");
        std::fs::create_dir_all(&build_dir).ok();

        let meson = std::process::Command::new("meson")
            .args([
                "setup",
                build_dir.to_str().unwrap(),
                compositor_dir.to_str().unwrap(),
                "--buildtype=release",
                "-Dregex=true",
                "-Dunittest=false",
            ])
            .status()
            .expect("meson failed");
        assert!(meson.success(), "meson setup failed");

        let ninja = std::process::Command::new("ninja")
            .args(["-C", build_dir.to_str().unwrap()])
            .status()
            .expect("ninja failed");
        assert!(ninja.success(), "ninja build failed");

        // Copy the built binary to OUT_DIR so include_bytes! can find it
        std::fs::copy(build_dir.join("src/srcom"), out.join("srcom"))
            .expect("failed to copy srcom binary");

        println!("cargo:rustc-cfg=feature=\"embedded-compositor\"");
        println!("cargo:rerun-if-changed=compositor/");
    }
    println!("cargo:rerun-if-changed=c-src/");
}
