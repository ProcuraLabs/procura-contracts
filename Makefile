# Procura contracts — developer tasks.
#
# These targets wrap the same commands CI runs (see .github/workflows/ci.yml).
# Only currently-implemented tasks are included; deployment lands with
# scripts/deploy.sh in a later change.

.PHONY: help build test fmt fmt-check lint check clean

help: ## Show this help
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) \
		| awk 'BEGIN {FS = ":.*?## "}; {printf "  \033[36m%-12s\033[0m %s\n", $$1, $$2}'

build: ## Build optimized WASM for all contracts (wasm32v1-none)
	stellar contract build

test: ## Run the full test suite
	cargo test --workspace

fmt: ## Format all Rust sources
	cargo fmt --all

fmt-check: ## Check formatting without writing changes
	cargo fmt --all --check

lint: ## Run clippy with warnings denied
	cargo clippy --all-targets --workspace -- -D warnings

check: fmt-check lint test ## Run all CI checks locally

clean: ## Remove build artifacts
	cargo clean
