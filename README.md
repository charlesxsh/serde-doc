
[![Crates.io](https://img.shields.io/crates/v/serde-doc)](https://crates.io/crates/serde-doc)

serde-doc is an tool that generates documentations for the serde structures in your source code  without modifying any source code.

## Features
- Markdown (WIP)
Direct human readable markdown document for the serde structures

- JSON Schema (WIP)
JSON Schema gives you more flexibilities to use other document generators.


## Usage
```
A cargo extension CLI for generating documentation for serde structs

Usage: cargo serde-doc [OPTIONS] <COMMAND> [ARGS]

Commands:
  list  List available serde structs
  gen   Generate files using a generator
  help  Print this message or the help of the given subcommand(s)

Options:
  -m, --manifest-path <MANIFEST_PATH>  Path to the Cargo.toml file or directory containing it [default: .]
  -h, --help                           Print help
  -V, --version                        Print version
```

## Dev
```
cargo install --path .
```