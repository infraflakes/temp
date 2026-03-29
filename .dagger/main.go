<<<<<<< HEAD
<<<<<<< HEAD
=======
// .dagger/main.go
>>>>>>> 1a2036c (Dagger)
=======
>>>>>>> 2dd0a21 (Static major libraries)
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
<<<<<<< HEAD
<<<<<<< HEAD
=======
>>>>>>> 2dd0a21 (Static major libraries)
			"gcc", "pkg-config", "upx-ucl",
			"libx11-dev", "libxinerama-dev", "libxft-dev", "libxrender-dev",
			"libxcb1-dev", "libx11-xcb-dev", "libxext-dev",
			"libxau-dev", "libxdmcp-dev",
			"libfontconfig-dev", "libfreetype-dev",
			"libexpat1-dev", "zlib1g-dev", "libbz2-dev",
			"libpng-dev", "libbrotli-dev",
<<<<<<< HEAD
			"libclang-dev",
		}).
		WithDirectory("/src", source.WithoutDirectory("target")).
		WithWorkdir("/src").
		WithExec([]string{"cargo", "build", "--release"}).
		WithExec([]string{"upx", "--best", "--lzma", "target/release/srwm"}).
=======
			// C compiler + pkg-config
			"gcc", "pkg-config",
			// X11 stack (needed by c-src/wm.h)
			"libx11-dev", "libxinerama-dev", "libxft-dev", "libx11-xcb-dev",
			// Fontconfig + Freetype (font rendering)
			"libfontconfig1-dev", "libfreetype6-dev",
			// libclang (needed by bindgen to parse bridge.h)
=======
>>>>>>> 2dd0a21 (Static major libraries)
			"libclang-dev",
		}).
		WithDirectory("/src", source.WithoutDirectory("target")).
		WithWorkdir("/src").
		WithExec([]string{"cargo", "build", "--release"}).
<<<<<<< HEAD
>>>>>>> 1a2036c (Dagger)
=======
		WithExec([]string{"upx", "--best", "--lzma", "target/release/srwm"}).
>>>>>>> 2dd0a21 (Static major libraries)
		File("target/release/srwm")
}
