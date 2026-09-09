#!/bin/bash
set -e
echo "Building all packages..."

cd packages/forge-core-js && npm run build && cd ../..
cd apps/forge-web && npm run build && cd ../..

if command -v cargo &> /dev/null; then
  echo "Building Rust workspace..."
  cargo build --workspace
else
  echo "Cargo not found, skipping Rust build"
fi

echo "Build complete!"
