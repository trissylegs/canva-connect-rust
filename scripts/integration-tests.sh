#!/bin/bash
set -e

echo "🧪 Running Canva Connect Integration Tests"
echo "=========================================="

# Check if .env file exists
if [ ! -f ".env" ]; then
    echo "❌ .env file not found!"
    echo "   Please create a .env file with CANVA_ACCESS_TOKEN"
    echo "   You can copy from .env.example:"
    echo "   cp .env.example .env"
    exit 1
fi

# Source environment variables from .env file
echo "📄 Loading environment variables from .env..."
set -a  # automatically export all variables
source .env
set +a  # stop automatically exporting

# Check if CANVA_ACCESS_TOKEN is set
if [ -z "$CANVA_ACCESS_TOKEN" ]; then
    echo "❌ CANVA_ACCESS_TOKEN not found in .env file!"
    echo "   Please add your Canva Connect API access token to .env:"
    echo "   CANVA_ACCESS_TOKEN=your-token-here"
    exit 1
fi

# Enable integration tests
export CANVA_INTEGRATION_TESTS=1

echo "✅ Environment configured"
echo "   - Access token: ${CANVA_ACCESS_TOKEN:0:10}..." # Show first 10 chars
echo "   - Integration tests: enabled"
echo ""

# Check for specific test filter
if [ $# -eq 0 ]; then
    echo "🚀 Running all integration tests..."
    cargo test --test integration
else
    echo "🚀 Running integration test: $1"
    cargo test "$1" --test integration
fi

echo ""
echo "✅ Integration tests completed!"
echo ""
echo "💡 Tips:"
echo "   - Run specific test: ./scripts/integration-tests.sh test_list_designs"
echo "   - Check .env file for your access token configuration"
echo "   - Tests automatically clean up any created resources"
