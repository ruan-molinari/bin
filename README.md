# Bin
A simple command line utility to easily install executable binaries into unix systems.

# Usage
To install a binary run the following command:
`bin install -b <binary>`

This fill automatically run `chmod +x` to give authorize the binary.

**Bin** uses shell variables to select which folder to install thi binaries.
The default install path is `$HOME/.bin`, but you can change this by creating a variable named `PERSONAL_BIN`.

To install it in a different path use the argument `-t` (target):
`bin install -b <binary> -t <target-dir>`


