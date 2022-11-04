SHELL := bash
.ONESHELL:
.SHELLFLAGS := -eu -o pipefail -c
MAKEFLAGS += --warn-undefined-variables
MAKEFLAGS += --no-builtin-rules

all: build

SCHEME_DIR = ~/Library/Application\ Support/glib-2.0/schemas

install:
	mkdir -p ${SCHEME_DIR}
	cp src/com.paulrouget.himalaya-gui.gschema.xml  ${SCHEME_DIR}
	glib-compile-schemas ${SCHEME_DIR}

run: install
	GSETTINGS_SCHEMA_DIR=${SCHEME_DIR} cargo run

build:
	cargo build --all-features

doc:
	cargo doc --all-features --no-deps

fmt:
	cargo +nightly fmt

check-fmt:
	cargo +nightly fmt --check

readme:
	cargo doc2readme --expand-macros --out Readme.md

check-readme:
	cargo doc2readme --expand-macros --out Readme.md --check

fix: fmt readme
	cargo +nightly cranky --all-features --fix

check-udeps:
	cargo +nightly udeps --all-features

check-cranky:
	cargo +nightly cranky --all-features -- -D warnings

check: doc check-readme check-fmt check-udeps check-cranky

test:
	cargo test --all-features

setup:
	rustup install nightly
	rustup component add rustfmt --toolchain nightly
	cargo install cargo-doc2readme
	cargo install cargo-cranky
	cargo +nightly install cargo-udeps --locked

setup-mac:
	brew install gtk4 libadwaita

clean:
	rm -rf target
