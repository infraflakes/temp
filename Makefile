# Makefile for Serein Window Manager

VERSION ?= $(shell git describe --tags --always --dirty --first-parent 2>/dev/null || echo "dev")
CONFIG_H = internal/core/config.h
CONFIG_DEF = internal/core/config.def.h

.PHONY: all build config clean fmt lint

all: build

# Generate config.h from config.def.h if it doesn't exist
config: $(CONFIG_H)
$(CONFIG_H):
	cp $(CONFIG_DEF) $(CONFIG_H)

build: config
	@echo "Building swm $(VERSION)..."
	go build -ldflags="-w -s -X main.Version=$(VERSION)" -o ./bin/swm .

fmt:
	@echo "Formatting code..."
	go fmt ./...

lint:
	@echo "Linting..."
	golangci-lint run

clean:
	@echo "Cleaning..."
	rm -rf ./bin/swm
	rm -f $(CONFIG_H)
