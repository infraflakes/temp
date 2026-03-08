# Makefile for Serein Window Manager

VERSION ?= $(shell git describe --tags --always --dirty --first-parent 2>/dev/null || echo "dev")
<<<<<<< HEAD
<<<<<<< HEAD
<<<<<<< HEAD
.PHONY: all build config clean fmt lint

all: build

build: clean
	@echo "Building srwm $(VERSION)..."
	dagger call build --source=. export --path=./target/release/srwm

build-verbose: clean
	@echo "Building srwm $(VERSION)..."
	dagger call build --source=. --progress=plain export --path=./target/release/srwm

fmt:
	@echo "Formatting code..."
	cargo fmt

lint:
	@echo "Linting..."

clean:
	@echo "Cleaning..."
	rm -rf ./target/release
=======
=======
CONFIG_H = internal/core/config.h
CONFIG_DEF = internal/core/config.def.h
>>>>>>> 2430b91 (Initial scaffolding)

=======
>>>>>>> 46f7a59 (Add UPX)
.PHONY: all build config clean fmt lint

all: build

build:
	@echo "Building srwm $(VERSION)..."
	go build -ldflags="-w -s -X main.Version=$(VERSION)" -o ./bin/srwm .
	upx --best --lzma ./bin/srwm

fmt:
	@echo "Formatting code..."
	go fmt ./...

lint:
	@echo "Linting..."
	golangci-lint run

clean:
	@echo "Cleaning..."
<<<<<<< HEAD
	rm -rf ./bin/swm
<<<<<<< HEAD
>>>>>>> f1a06f2 (Flakes)
=======
=======
	rm -rf ./bin/srwm
>>>>>>> 269cb88 (Renaming to srwm)
	rm -f $(CONFIG_H)
>>>>>>> 2430b91 (Initial scaffolding)
