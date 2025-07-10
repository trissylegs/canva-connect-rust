#!/bin/bash
set -e

echo "🚀 Setting up Canva Connect Rust development environment..."

# Check if we're in a git repository
if [ ! -d ".git" ]; then
    echo "❌ Error: Not in a git repository root"
    exit 1
fi

# Check if scripts directory exists
if [ ! -d "scripts" ]; then
    echo "❌ Error: scripts/ directory not found"
    exit 1
fi

echo "🔧 Configuring git settings..."

# Set up git config for better development experience
git config --local core.autocrlf false
git config --local push.default simple
git config --local pull.rebase true

echo "🪝 Setting up pre-commit hook..."

# Make sure pre-commit script is executable
chmod +x scripts/pre-commit.sh

# Create symlink to pre-commit hook
if [ -f ".git/hooks/pre-commit" ]; then
    echo "⚠️  Pre-commit hook already exists, backing up..."
    mv .git/hooks/pre-commit .git/hooks/pre-commit.backup
fi

ln -sf "../../scripts/pre-commit.sh" .git/hooks/pre-commit
echo "✅ Pre-commit hook installed"

echo "🔍 Checking development environment..."

# Check if Rust is installed
if ! command -v cargo &> /dev/null; then
    echo "❌ Error: Rust/Cargo not found. Install from https://rustup.rs/"
    exit 1
fi
echo "✅ Rust/Cargo found: $(cargo --version)"

# Check if rustfmt is available
if ! cargo fmt --version &> /dev/null; then
    echo "⚠️  Installing rustfmt..."
    rustup component add rustfmt
fi
echo "✅ rustfmt available: $(cargo fmt --version)"

# Check if clippy is available
if ! cargo clippy --version &> /dev/null; then
    echo "⚠️  Installing clippy..."
    rustup component add clippy
fi
echo "✅ clippy available: $(cargo clippy --version)"

echo "🧪 Running initial checks to verify setup..."

# Run a quick check to make sure everything works
./scripts/check.sh

echo ""
echo "🎉 Development environment setup complete!"
echo ""
echo "📋 Next steps:"
echo "   1. Copy environment template: cp .env.example .env"
echo "   2. Add your Canva API credentials to .env"
echo "   3. Run examples: cargo run --example basic_usage"
echo ""
echo "💡 Useful commands:"
echo "   ./scripts/check.sh  - Run all CI checks"
echo "   ./scripts/fix.sh    - Auto-fix issues"
echo "   cargo test          - Run tests"
echo ""
echo "🪝 Pre-commit hook is now active and will run on every commit!"
