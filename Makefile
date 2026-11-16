# figmaa: build and run shortcuts. Every target wraps a cargo command, so `make -n <target>`
# shows exactly what runs. The packages still carry their original `figcraft` names.

CARGO ?= cargo
# The app's control channel port for `make run-control` (loopback only).
PORT ?= 47800
# A file to open, e.g. `make run FILE=design.figcraft`.
FILE ?=

.PHONY: help build release run run-release run-control cli mcp server test test-agent fmt lint ci wasm clean

help: ## List the targets
	@grep -E '^[a-zA-Z_-]+:.*?## ' $(MAKEFILE_LIST) | awk 'BEGIN {FS = ":.*?## "}; {printf "  %-14s %s\n", $$1, $$2}'

build: ## Build the desktop app (debug)
	$(CARGO) build -p figcraft

release: ## Build the desktop app and CLI (release)
	$(CARGO) build --release -p figcraft -p figcraft-cli

run: ## Run the desktop app (debug), opening FILE if given
	$(CARGO) run -p figcraft -- $(FILE)

run-release: ## Run the desktop app (release), opening FILE if given
	$(CARGO) run --release -p figcraft -- $(FILE)

run-control: ## Run the app with the control channel on PORT
	$(CARGO) run -p figcraft -- --control $(PORT) $(FILE)

cli: ## Build the headless CLI
	$(CARGO) build --release -p figcraft-cli

mcp: ## Start the headless MCP server on stdio
	$(CARGO) run --release -p figcraft-cli -- mcp

server: ## Run the collaboration server
	$(CARGO) run --release -p figcraft-server

test: ## Run every test in the workspace
	$(CARGO) test --workspace

test-agent: ## Run the agent and MCP tool tests only
	$(CARGO) test -p figcraft-agent -p figcraft-mcp

fmt: ## Format the code
	$(CARGO) fmt --all

lint: ## Clippy with warnings as errors
	$(CARGO) clippy --workspace --all-targets -- -D warnings

ci: ## All checks: fmt, clippy, tests, layering, assets, wasm
	$(CARGO) xtask ci

wasm: ## Build the web version (needs trunk)
	$(CARGO) xtask wasm

clean: ## Remove build output
	$(CARGO) clean
