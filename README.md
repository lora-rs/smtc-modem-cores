# smtc-modem-cores

Provides Rust bindings to [Semtech's SX12xx drivers written in C](https://github.com/Lora-net/SWL2001).

## Setup & build dependencies

Be sure to initialize the git submodule and to install the build dependencies:
```bash
git submodule update --init --recursive
sudo apt-get update && sudo apt-get install -y --no-install-recommends cmake llvm-dev clang libclang-dev
```

## Project layout

### smtc-modem-cores-sys

This submodule generates the Rust bindings to the C library:
- `SWL2001`, the underlying C code by Semtech is included as a git submodule
- `CMakeLists.txt` is manually maintained here and invoked by `smtc-modem-cores-sys/build.rs`; it cherry-picks the 
necessary files for building the modem cores.

### smtc-modem-cores

Makes the bindings generated by `smtc-modem-cores-sys` and creates a relatively safe Rust API for
users.