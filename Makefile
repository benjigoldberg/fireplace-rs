CARGO_HOME=$(HOME)/.cargo
CARGO_BIN_DIR=$(CARGO_HOME)/bin
CARGO_WATCH_DIR=$(CARGO_BIN_DIR)/cargo-watch
CARGO_SYSTEMFD_DIR=$(CARGO_BIN_DIR)/systemfd

.PHONY: all
all: build

.PHONY: clean
clean: ## Cleans up build artifacts
	cargo clean

.PHONY: build
build: ## Builds the binary
	cargo build

.PHONY: release
release: ## Builds the binary in release mode
	cargo build --release

$(CARGO_WATCH_DIR):
	cargo install cargo-watch

$(CARGO_SYSTEMFD_DIR):
	cargo install systemfd

.PHONY: watch
watch: $(CARGO_WATCH_DIR) $(CARGO_SYSTEMFD_DIR) ## Runs the server in development mode with hot reloading
	systemfd --no-pid -s http::3000 -- cargo watch -x run

.PHONY: help
help: ## Prints this help command
	@grep -E '^[a-zA-Z0-9_-]+:.*?## .*$$' $(MAKEFILE_LIST) |\
		sort | \
		awk 'BEGIN {FS = ":.*?## "}; {printf "\033[36m%-30s\033[0m %s\n", $$1, $$2}'
