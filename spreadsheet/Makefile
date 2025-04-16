# Makefile for running Rust project in release mode

PROJECT_DIR := spreadsheet
CARGO := cargo
CARGO_FLAGS := --manifest-path $(PROJECT_DIR)/Cargo.toml

.PHONY: all build run clean

all: build 

build:
	$(CARGO) build --release

run:
	$(CARGO) run --release

clean:
	$(CARGO) clean

