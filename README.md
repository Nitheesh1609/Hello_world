# Simple Kernel Module in Rust
Crate with an example of a kernel module in Rust

## What does this crate actually do?
This repo is a Makefile with a Rust project that, using Kbuild, the kernel's build system, will generate 
a kernel module file named `hello.ko`.

This repo takes inspiration from user jbaublitz's repository https://github.com/jbaublitz/knock-out, who has 
also given a talk on the process of creating this kernel module at 
[DevConf.cz 2020](https://www.youtube.com/watch?v=oacmnKlWZT8&t=32s).

## Instructions for loading the module
```
# Make sure Rust is installed via rustup
# See https://rustup.rs/ for instructions

# This project requires a nightly Rust build
rustup default nightly

# Run the bash script to make, insert and remove the module and print the relevant dmesg output
./test.sh

```
