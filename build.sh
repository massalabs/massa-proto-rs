#!/bin/bash

echo "Building Massa proto bindings..."

# Generate ABI documentation
echo "Generating RUST bindings..."
cd ./rust && cargo build --release
echo "RUST bindings generated successfully."

echo "Massa proto bindings build finished!"
