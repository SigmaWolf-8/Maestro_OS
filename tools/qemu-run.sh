#!/bin/bash
# QEMU launcher for Maestro OS
# Usage: ./tools/qemu-run.sh [--debug]

set -e

BUILD_DIR="target/x86_64-maestro/release"
OVMF_PATH="/usr/share/OVMF/OVMF_CODE.fd"

if [ ! -f "/bootimage-salvi-boot.bin" ]; then
    echo "Error: Boot image not found. Run "cargo bootimage --release" first."
    exit 1
fi

QEMU_ARGS=(
    -machine q35
    -m 512M
    -smp 4
    -drive "format=raw,file=/bootimage-salvi-boot.bin"
    -bios ""
    -serial stdio
    -display sdl
)

if [ "" = "--debug" ]; then
    QEMU_ARGS+=(-s -S)
    echo "Waiting for GDB on :1234..."
fi

qemu-system-x86_64 ""
