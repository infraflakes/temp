# Makefile for Serein Window Manager

VERSION ?= $(shell git describe --tags --always --dirty --first-parent 2>/dev/null || echo "dev")
.PHONY: all build config clean fmt lint

all: build build-static

build: clean
	@echo "Building srwm static $(VERSION)..."
	dagger call build --source=. --progress=plain export --path=./target/release/srwm

fmt:
	@echo "Formatting code..."
	go fmt ./...

lint:
	@echo "Linting..."

clean:
	@echo "Cleaning..."
	rm -rf ./target/release
