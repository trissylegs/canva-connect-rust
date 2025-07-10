#!/bin/bash
set -e

echo "📖 Generating Rust documentation..."

# Generate documentation with all features
echo "🔧 Building docs with all features..."
cargo doc --all-features --no-deps --document-private-items

# Open documentation in browser (optional)
if [[ "$1" == "--open" ]]; then
    echo "🌐 Opening documentation in browser..."
    cargo doc --all-features --no-deps --document-private-items --open
else
    echo "💡 Run 'scripts/docs.sh --open' to open docs in browser"
fi

# Show where docs are generated
echo "📁 Documentation generated in: target/doc/"
echo "🔗 Main entry point: target/doc/canva_connect/index.html"

echo "✅ Documentation generation complete!"
