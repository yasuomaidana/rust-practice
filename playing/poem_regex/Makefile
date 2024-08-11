CC := gcc
RUSTC := rustc
CARGO := cargo
TARGET := poem_regex
SHELL := /bin/bash

# Colors for different outputs
COLOR_HELP := $(if $(COLOR),\033[0;36m,)
COLOR_CMD := $(if $(COLOR),\033[1;32m,)
COLOR_RESET := $(if $(COLOR),\033[0m,)

# Phony targets to provide helpful commands
.PHONY: all help clean run build lint format

# Default target - build
all: build

help:
	@echo "Available commands:"
	@echo "  ${COLOR_HELP}help${COLOR_RESET} - Display this help message"
	@echo "  ${COLOR_HELP}clean${COLOR_RESET} - Clean the project artifacts"
	@echo "  ${COLOR_HELP}run${COLOR_RESET}  - Build and run the program"
	@echo "  ${COLOR_HELP}build${COLOR_RESET}  - Build the program (optimized)"
	@echo "  ${COLOR_HELP}lint${COLOR_RESET}   - Lint the Rust code using Clippy"
	@echo "  ${COLOR_HELP}format${COLOR_RESET} - Format Rust code using fmt"

clean:
	@echo "Cleaning..."
	@$(CARGO) clean

run: build
	@echo "Running..."
	@$(CARGO) run @$(TARGET)

build:
	@echo "Building..."
	@$(CARGO) build --release

lint:
	@$(if $(shell which clippy >/dev/null 2>&1), \
        echo "Linting..." && $(CARGO) clippy -- -D warnings, \
        @echo "Clippy not installed. Please install it to use linting.";)

format:
	@rustup component add rustfmt
	cargo fmt

bump:
	@echo "Bumping version..."
	@echo "Current version: $(shell cargo pkgid | cut -d# -f2)"
	@read -p "Enter new version: " version; \
	updated_version=$$(cargo pkgid | cut -d#  -f2 | sed -E "s/([0-9]+\.[0-9]+\.[0-9]+)$$/$$version/"); \
	sed -i -E "s/^version = .*/version = \"$$updated_version\"/" Cargo.toml
	@echo "Updated version: $(shell cargo pkgid | cut -d# -f2)"

# Set the COLOR variable to enable colored output
COLOR ?= 1