#!/bin/bash
set -e

echo "🔍 Running CI checks locally..."

echo "📏 Checking code formatting..."
cargo fmt --all -- --check

echo "📎 Running clippy..."
cargo clippy --all-targets --all-features -- -D warnings

echo "🧪 Running tests..."
cargo test --all-features

echo "🏗️  Building release..."
cargo build --release

echo "✅ All checks passed! Ready to commit."
