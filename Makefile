SHELL := bash
.ONESHELL:
.SHELLFLAGS := -eu -o pipefail -c
MAKEFLAGS += --warn-undefined-variables
MAKEFLAGS += --no-builtin-rules

TARGET=target/release/himalaya-ui

all: build

run:
	RUST_LOG=warn cargo run -p himalaya-ui --release

run-debug:
	RUST_LOG=himalaya_ui=trace cargo run -p himalaya-ui --release

run-debug-all-logs:
	RUST_BACKTRACE=1 RUST_LOG=debug cargo run -p himalaya-ui --release

run-lldb: build-debug
	lldb ./target/debug/himalaya-ui

build:
	cargo build -p himalaya-ui --release

build-debug:
	cargo build -p himalaya-ui

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
