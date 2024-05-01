# Rust simple HTTP-server
My first tiny-project on Rust for laboratory work in NSU

This project builds an executable which will perform access to files in some dir with (or without GET-parameters pre-rendering)

## Installation
1. [Install Rust](https://www.rust-lang.org/tools/install)
2. Run `cargo install --git https://github.com/KozlovKV/rust-simple-http` by base cargo binary path

## Usage
Executute binary with argument specifying directory with `index.html` and other dirs/files.

For text files you can write `{{ key }}` pattern then here will be placed `value` from request `/render/<path/to/file>?key=value`

You also can obtain or download static files by request `/static/<path/to/file>`