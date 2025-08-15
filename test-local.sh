#!/bin/bash

# Local test runner for comprehensive testing
# Run this before committing to ensure CI will pass

set -e

echo "🧪 Running local tests..."
echo

# Format check
echo "📋 Checking formatting..."
cargo fmt -- --check

# Linting
echo "🔍 Running clippy..."
cargo clippy --all-targets --all-features -- -D warnings

# Tests
echo "🧪 Running tests..."
cargo test --all-features

# Doc tests
echo "📚 Running doc tests..."
cargo test --doc --all-features

# Documentation build
echo "📖 Building documentation..."
cargo doc --no-deps --all-features

echo
echo "✅ All tests passed!"
