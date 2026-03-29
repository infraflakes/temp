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
=======
>>>>>>> 7870993 (Modularize codebase)
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

    println!("cargo:rerun-if-changed=c-src/");
}
