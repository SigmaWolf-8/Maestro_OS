#!/bin/bash
# Integration test runner — boots Maestro OS in QEMU and runs test suite
# Usage: ./tools/test-runner/run-integration.sh

set -e

echo "Building test kernel..."
cargo test -p salvi-kernel --no-run

echo "Running integration tests in QEMU..."
cargo test -p salvi-kernel -- --test-threads=1

echo "Integration tests complete."
