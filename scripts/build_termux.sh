#!/bin/bash
# Aryabhata Node — Termux Build Script
# Android AArch64

echo "Aryabhata Node — Termux Build"
echo "Target: Android AArch64"

pkg update -y
pkg install -y rust clang git

cd rust
cargo build --release

BINARY="target/release/aryabhata-node"

if [ -f "$BINARY" ]; then
    SIZE=$(du -sh "$BINARY" | cut -f1)
    echo "Build successful"
    echo "Binary size: $SIZE"
    echo "Run: ./$BINARY --mode=pruned --mining=low-power"
else
    echo "Build failed"
    exit 1
fi
