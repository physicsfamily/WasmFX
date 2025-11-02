#!/bin/bash

# WasmFX Build Script
# This script builds the Rust WASM library and sets up the Angular demo

set -e

echo "🦀 WasmFX Build Script"
echo "====================="
echo ""

# Check if wasm-pack is installed
if ! command -v wasm-pack &> /dev/null; then
    echo "❌ Error: wasm-pack is not installed."
    echo "Install it with: cargo install wasm-pack"
    exit 1
fi

# Check if npm is installed
if ! command -v npm &> /dev/null; then
    echo "❌ Error: npm is not installed."
    echo "Install Node.js from: https://nodejs.org/"
    exit 1
fi

echo "Step 1/3: Building Rust WASM library..."
cd wasm_lib
wasm-pack build --target bundler
echo "✅ WASM library built successfully!"
echo ""

echo "Step 2/3: Installing Angular dependencies..."
cd ../demo_site
npm install
echo "✅ Dependencies installed!"
echo ""

echo "Step 3/3: Ready to run!"
echo ""
echo "🚀 To start the development server, run:"
echo "   cd demo_site && npm start"
echo ""
echo "Then open http://localhost:4200 in your browser."
echo ""
echo "✨ Build complete!"
