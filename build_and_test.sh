#!/bin/bash

# Script to build and test the WASM library


echo "🚀 Building WASM package..."
wasm-pack build --target web

echo "🌐 Running WASM tests in headless Chrome..."
wasm-pack test --headless --chrome

echo "✅ All tests completed!"
