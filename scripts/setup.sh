#!/bin/bash
set -e

echo "Setting up Forge..."

# Check Node
if ! command -v node &> /dev/null; then
  echo "Node.js not found, please install Node.js 18+"
  exit 1
fi

echo "Node version: $(node --version)"

# Install JS Core
echo "Installing JS Core..."
cd packages/forge-core-js
npm install
npm run build
cd ../..

# Install Web
echo "Installing Web client..."
cd apps/forge-web
npm install
cd ../..

echo "Forge setup complete!"
echo "Run: forge demo or forge server"
