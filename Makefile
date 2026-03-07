# Makefile for Serein Window Manager

VERSION ?= $(shell git describe --tags --always --dirty --first-parent 2>/dev/null || echo "dev")
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

.PHONY: all build generate fmt lint clean

all: build
generate:
	@echo "Building swm..."
	go build -ldflags="-w -X main.Version=$(VERSION)" -o ./bin/swm

fmt:
	@echo "Formatting code..."
	go fmt ./...

lint:
	CGO_ENABLED=1 CC="zig cc -target x86_64-windows-gnu" GOOS=windows golangci-lint run

clean:
	@echo "Cleaning..."
	rm -rf ./bin/swm
>>>>>>> f1a06f2 (Flakes)
