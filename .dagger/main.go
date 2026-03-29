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
			"gcc", "pkg-config", "upx-ucl",
			"libx11-dev", "libxinerama-dev", "libxft-dev", "libxrender-dev",
			"libxcb1-dev", "libx11-xcb-dev", "libxext-dev",
			"libxau-dev", "libxdmcp-dev",
			"libfontconfig-dev", "libfreetype-dev",
			"libexpat1-dev", "zlib1g-dev", "libbz2-dev",
			"libpng-dev", "libbrotli-dev",
			"libclang-dev",
		}).
		WithDirectory("/src", source.WithoutDirectory("target")).
		WithWorkdir("/src").
		WithExec([]string{"cargo", "build", "--release"}).
		WithExec([]string{"upx", "--best", "--lzma", "target/release/srwm"}).
		File("target/release/srwm")
}
