#!/bin/bash
set -e

mkdir -p public/wasm

zig build

cp zig-out/bin/radix_sort.wasm public/wasm/radix_sort.wasm

echo "✅ WASM compiled successfully!"