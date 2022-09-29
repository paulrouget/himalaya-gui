SHELL := bash
.ONESHELL:
.SHELLFLAGS := -eu -o pipefail -c
MAKEFLAGS += --warn-undefined-variables
MAKEFLAGS += --no-builtin-rules

all: build

run:
	RUST_LOG=warn cargo run -p himalaya-gui --release

run-debug:
	RUST_LOG=himalaya_ui=trace cargo run -p himalaya-gui --release

run-debug-all-logs:
	RUST_BACKTRACE=1 RUST_LOG=debug cargo run -p himalaya-gui --release

run-lldb: build-debug
	lldb ./target/debug/himalaya-gui

build:
	cargo build -p himalaya-gui --release

build-debug:
	cargo build -p himalaya-gui

fmt:
	cargo fmt

fix: fmt
	cargo cranky --fix

check-fmt:
	cargo-fmt --check

check-udeps:
	cargo udeps

check-cranky:
	cargo cranky -- -D warnings

check: check-fmt check-udeps check-cranky

doc:
	cargo doc --no-deps

clean:
	rm -rf target
