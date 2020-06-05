CARGO_HOME=$(HOME)/.cargo
CARGO_BIN_DIR=$(CARGO_HOME)/bin
CARGO_WATCH_DIR=$(CARGO_BIN_DIR)/cargo-watch
CARGO_SYSTEMFD_DIR=$(CARGO_BIN_DIR)/systemfd
CARGO_CLIPPY_DIR=$(CARGO_BIN_DIR)/cargo-clippy
CARGO_FORMAT_DIR =$(CARGO_BIN_DIR)/rustfmt

.PHONY: all
all: test lint build

.PHONY: clean
clean: ## Cleans up build artifacts
	cargo clean

.PHONY: build
build: ## Builds the binary
	cargo build

.PHONY: cross
cross-raspi: ## Cross-compiles the binary targeting the raspberry-pi
	cross build --release --target armv7-unknown-linux-gnueabihf

.PHONY: install-raspi
install-raspi: cross-raspi ## Installs the binary on the raspberry pi from a remote machine
	scp target/armv7-unknown-linux-gnueabihf/release/fireplace-rs bgoldberg@192.168.2.100:

.PHONY: release
release: ## Builds the binary in release mode
	cargo build --release

$(CARGO_WATCH_DIR):
	cargo install cargo-watch

$(CARGO_SYSTEMFD_DIR):
	cargo install systemfd

.PHONY: watch
watch: $(CARGO_WATCH_DIR) $(CARGO_SYSTEMFD_DIR) ## Runs the server in development mode with hot reloading
	systemfd --no-pid -s http::8000 -- cargo watch -x 'run -- server'

test: ## Runs all tests
	cargo test

$(CARGO_CLIPPY_DIR):
	cargo install cargo-clippy

.PHONY: lint
lint: $(CARGO_CLIPPY_DIR) ## Lints the code with Cargo Clippy
	cargo clippy --all-targets --all-features -- -D warnings

$(CARGO_FORMAT_DIR):
	cargo install rustfmt

.PHONY: fmt
fmt: $(CARGO_FORMAT_DIR) ## Formats code
	cargo fmt

.PHONY: help
help: ## Prints this help command
	@grep -E '^[a-zA-Z0-9_-]+:.*?## .*$$' $(MAKEFILE_LIST) |\
		sort | \
		awk 'BEGIN {FS = ":.*?## "}; {printf "\033[36m%-30s\033[0m %s\n", $$1, $$2}'
