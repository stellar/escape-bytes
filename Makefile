.PHONY: all test fuzz build doc readme clean fmt

export RUSTFLAGS=-Dwarnings -Dclippy::all -Dclippy::pedantic

all: build test

test:
	cargo hack test --tests --feature-powerset --exclude-features docs

fuzz:
	cargo +nightly fuzz run roundtrip

build:
	cargo hack clippy --feature-powerset --exclude-features docs --all-targets

doc:
	cargo hack test --doc --all-features
	RUSTDOCFLAGS="--cfg doc" cargo +nightly doc --all-features --open

readme:
	cargo readme > README.md

clean:
	cargo clean

fmt:
	cargo fmt --all
