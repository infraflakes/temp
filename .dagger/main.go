package main

import (
	"context"
	"srwm/internal/dagger"
)

type Srwm struct{}

func (m *Srwm) Build(ctx context.Context, source *dagger.Directory) *dagger.File {
	return dag.Container().
		From("rust:latest").
		WithEnvVariable("DEBIAN_FRONTEND", "noninteractive").
		WithExec([]string{"apt-get", "update"}).
		WithExec([]string{"apt-get", "install", "-y",
			// srwm deps
			"gcc", "pkg-config",
			"libx11-dev", "libxinerama-dev", "libxft-dev", "libxrender-dev", "libx11-xcb-dev",
			"libfontconfig1-dev", "libfreetype6-dev",
			"libclang-dev",
			// srcom deps
			"meson", "ninja-build", "cmake", "git",
			"libxcb1-dev", "libxcb-composite0-dev", "libxcb-damage0-dev",
			"libxcb-dpms0-dev", "libxcb-image0-dev", "libxcb-present-dev",
			"libxcb-randr0-dev", "libxcb-render0-dev", "libxcb-render-util0-dev",
			"libxcb-shape0-dev", "libxcb-xfixes0-dev", "libxcb-xinerama0-dev",
			"libxcb-ewmh-dev", "libxcb-icccm4-dev", "libxcb-sync-dev",
			"libxcb-shm0-dev", "libxcb-util-dev",
			"libgl-dev", "libegl-dev", "libepoxy-dev",
			"libev-dev", "libpcre2-dev", "libpixman-1-dev",
			"libconfig-dev", "uthash-dev",
		}).
		WithDirectory("/src", source.WithoutDirectory("target")).
		WithWorkdir("/src").
		WithExec([]string{"cargo", "build", "--release"}).
		File("target/release/srwm")
}
