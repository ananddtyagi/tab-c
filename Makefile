.PHONY: all build engine ime clean install test release help

# Variables
ENGINE_DIR = engine
IME_DIR = mac/IME
BUILD_DIR = build
RELEASE_DIR = dist

# Detect architecture
ARCH := $(shell uname -m)
OS := $(shell uname -s)

help: ## Show this help
	@echo "MacAutoComplete Build System"
	@echo ""
	@echo "Targets:"
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | sort | awk 'BEGIN {FS = ":.*?## "}; {printf "  \033[36m%-20s\033[0m %s\n", $$1, $$2}'

all: build ## Build everything

build: engine ime ## Build engine and IME

engine: ## Build Rust engine
	@echo "Building engine..."
	cd $(ENGINE_DIR) && cargo build --release
	@echo "Engine built successfully"

engine-debug: ## Build engine in debug mode
	@echo "Building engine (debug)..."
	cd $(ENGINE_DIR) && cargo build
	@echo "Engine built (debug)"

ime: ## Build macOS IME (requires Xcode)
	@echo "Building IME..."
	@if [ "$(OS)" != "Darwin" ]; then \
		echo "Warning: IME can only be built on macOS"; \
		exit 1; \
	fi
	@echo "Please build the Xcode project manually: open $(IME_DIR)/IMEApp.xcodeproj"

test: ## Run tests
	@echo "Running Rust tests..."
	cd $(ENGINE_DIR) && cargo test

test-verbose: ## Run tests with output
	@echo "Running Rust tests (verbose)..."
	cd $(ENGINE_DIR) && cargo test -- --nocapture

clean: ## Clean build artifacts
	@echo "Cleaning..."
	cd $(ENGINE_DIR) && cargo clean
	rm -rf $(BUILD_DIR)
	rm -rf $(RELEASE_DIR)
	@echo "Clean complete"

install: ## Install locally (macOS only)
	@if [ "$(OS)" != "Darwin" ]; then \
		echo "Error: Installation only supported on macOS"; \
		exit 1; \
	fi
	./scripts/install_local.sh

release: ## Build release packages
	./scripts/build_release.sh

format: ## Format code
	cd $(ENGINE_DIR) && cargo fmt

lint: ## Run linters
	cd $(ENGINE_DIR) && cargo clippy -- -D warnings

check: ## Check code without building
	cd $(ENGINE_DIR) && cargo check

dev: ## Run in development mode
	./scripts/dev_run.sh

setup: ## Set up development environment
	@echo "Setting up development environment..."
	@./scripts/setup_dev.sh

# Model management
download-models: ## Download required models
	@echo "Downloading models..."
	@./scripts/download_models.sh

# Documentation
docs: ## Generate documentation
	cd $(ENGINE_DIR) && cargo doc --no-deps --open
