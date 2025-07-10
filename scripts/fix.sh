#!/bin/bash
set -e

echo "🔧 Auto-fixing code issues..."

echo "📏 Formatting code..."
cargo fmt --all

echo "📎 Running clippy with auto-fix..."
cargo clippy --all-targets --all-features --fix --allow-staged --allow-dirty

echo "✅ Auto-fixes applied! Run 'scripts/check.sh' to verify."
