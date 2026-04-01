package main

import (
	"context"
	"srwm/internal/dagger"
)

type Srwm struct{}

// BuildCompositor builds srcom as a standalone binary (ubuntu:22.04, known working).
func (m *Srwm) BuildCompositor(ctx context.Context, source *dagger.Directory) *dagger.File {
	return dag.Container().
		From("ubuntu:24.04").
		WithEnvVariable("DEBIAN_FRONTEND", "noninteractive").
		WithExec([]string{"apt-get", "update"}).
		WithExec([]string{"apt-get", "install", "-y",
			"meson", "ninja-build", "gcc", "pkg-config",
			"libx11-dev", "libxcb1-dev", "libxcb-composite0-dev", "libxcb-damage0-dev",
			"libxcb-dpms0-dev", "libxcb-image0-dev", "libxcb-present-dev",
			"libxcb-randr0-dev", "libxcb-render0-dev", "libxcb-render-util0-dev",
			"libxcb-shape0-dev", "libxcb-xfixes0-dev", "libxcb-xinerama0-dev",
			"libxcb-ewmh-dev", "libxcb-icccm4-dev", "libxcb-sync-dev",
			"libxcb-shm0-dev", "libxcb-util-dev",
			"libgl-dev", "libegl-dev", "libepoxy-dev",
			"libev-dev", "libpcre2-dev", "libpixman-1-dev",
			"libconfig-dev", "uthash-dev", "libx11-xcb-dev",
			"cmake", "git",
		}).
		WithDirectory("/src", source.Directory("compositor")).
		WithWorkdir("/src").
		WithExec([]string{"meson", "setup", "build", "--buildtype=release", "--default-library=static", "-Dstrip=true"}).
		WithExec([]string{"ninja", "-C", "build"}).
		File("build/src/srcom")
}

// Build produces the srwm binary with the compositor embedded.
func (m *Srwm) Build(ctx context.Context, source *dagger.Directory) *dagger.File {
	// Step 1: build srcom separately
	srcomBin := m.BuildCompositor(ctx, source)

	// Step 2: place the pre-built binary where build.rs expects it
	srcWithCompositor := source.
		WithoutDirectory("target").
		WithFile("embedded/srcom", srcomBin)

	return dag.Container().
		From("rust:latest").
		WithEnvVariable("DEBIAN_FRONTEND", "noninteractive").
		WithExec([]string{"apt-get", "update"}).
		WithExec([]string{"apt-get", "install", "-y",
			"gcc", "pkg-config",
			"libx11-dev", "libxinerama-dev", "libxft-dev", "libxrender-dev", "libx11-xcb-dev",
			"libfontconfig1-dev", "libfreetype6-dev",
			"libclang-dev", "upx-ucl",
		}).
		WithDirectory("/src", srcWithCompositor).
		WithWorkdir("/src").
		WithExec([]string{"cargo", "build", "--release", "--features", "embedded-compositor"}).
		WithExec([]string{"upx", "--best", "--lzma", "target/release/srwm"}).
		File("target/release/srwm")
}
