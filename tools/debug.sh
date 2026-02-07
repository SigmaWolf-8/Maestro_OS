#!/bin/bash
# GDB attach script for Maestro OS kernel debugging
# Usage: ./tools/debug.sh

set -e

echo "Attaching GDB to QEMU on localhost:1234..."
gdb -ex "target remote :1234" \
    -ex "symbol-file target/x86_64-maestro/release/salvi-kernel" \
    -ex "break _start" \
    -ex "continue"
