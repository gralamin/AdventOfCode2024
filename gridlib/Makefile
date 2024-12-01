prog=$(shell basename $(CURDIR))

build:
	cargo build --verbose

test:
	cargo test --verbose

clean:
	cargo clean --verbose
	- rm $(prog).day

format:
	cargo fmt

lint:
	cargo fmt -- --check

coverage:
	cargo tarpaulin --doc --tests --verbose --all-features --workspace --timeout 120 --engine llvm --fail-under 70

all: build

.PHONY: build test all clean format lint coverage
