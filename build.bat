@echo off
echo Building Massa proto bindings...

rem Generate ABI documentation
echo Generating RUST bindings...
cd ./rust
cargo build --release
echo RUST bindings generated successfully.

echo Massa proto bindings build finished!
