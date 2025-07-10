#!/bin/bash
set -e

echo "🔍 Running pre-commit checks..."

echo "📏 Checking code formatting..."
cargo fmt --all -- --check

echo "📎 Running clippy..."
cargo clippy --all-targets --all-features -- -D warnings

echo "🧪 Running tests..."
cargo test --all-features

echo "🏗️  Building release..."
cargo build --release

echo "✅ All pre-commit checks passed!"
