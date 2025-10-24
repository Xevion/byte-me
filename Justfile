# Default recipe - show available commands
default:
    @just --list

dev:
	cargo tauri dev

dev-build:
	cargo tauri build --debug

generate-types:
    cargo test --manifest-path src-tauri/Cargo.toml -- --test export_bindings

check-frontend:
    pnpm exec tsc --noEmit

check-backend:
    cargo clippy --manifest-path src-tauri/Cargo.toml

check: check-frontend check-backend

build-frontend: generate-types
    pnpm exec tsc
    pnpm exec vite build

build-backend:
    cargo build --manifest-path src-tauri/Cargo.toml

build: build-frontend build-backend

test-frontend:
    pnpm exec vitest run

test-backend:
    cargo nextest run --manifest-path src-tauri/Cargo.toml

test: test-frontend test-backend
