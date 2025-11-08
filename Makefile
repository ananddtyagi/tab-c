.PHONY: engine run-engine format lint

ENGINE_DIR := engine
SOCKET_PATH := $(HOME)/Library/Application\ Support/MacAutoComplete/engine.sock

engine:
cd $(ENGINE_DIR) && cargo build

run-engine:
cd $(ENGINE_DIR) && cargo run -- --socket "$(SOCKET_PATH)"

format:
cd $(ENGINE_DIR) && cargo fmt

lint:
cd $(ENGINE_DIR) && cargo clippy --all-targets --all-features -- -D warnings
