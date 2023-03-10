# MallardScript

[![Version](https://img.shields.io/crates/v/mallardscript?style=flat-square)](https://crates.io/crates/mallardscript)
[![Build](https://img.shields.io/travis/Nate-Wilkins/mallardscript/main?style=flat-square)](https://app.travis-ci.com/github/Nate-Wilkins/mallardscript)
[![Downloads](https://img.shields.io/crates/d/mallardscript?color=%230E0&style=flat-square)](https://crates.io/crates/mallardscript)
[![Open Issues](https://img.shields.io/github/issues-raw/Nate-Wilkins/mallardscript?style=flat-square)](https://github.com/Nate-Wilkins/mallardscript/issues)
[![License](https://img.shields.io/github/license/Nate-Wilkins/mallardscript?color=%2308F&style=flat-square)](https://github.com/Nate-Wilkins/mallardscript/blob/main/LICENSE)

> Hak5 DuckyScript extended language compiler.

## Installation

```
cargo install mallardscript
```

## Usage

Compiles [MallardScript](https://github.com/Nate-Wilkins/pest_duckyscript) to DuckyScript!

```
mallardscript build --input src/index.ducky --output output/index.ducky
```

## Configuration

### Shell Completions

You can put this in your `.zshrc` file (just make sure `$HOME/.zsh_functions/` is in your
`fpath`):

```
if [[ ! -f "$HOME/.zsh_functions/_mallardscript" ]]; then
  mallardscript completions --type zsh > "$HOME/.zsh_functions/_mallardscript"
fi
```

Or you can generate yours with:

```
mallardscript completions --type $SHELL               # Where $SHELL is zsh,bash,fish,elvish,powershell
```

## Development

Written in rust. Workflows are defined in `.envrc.sh`.

## Roadmap

- Encode directly (or by library ref) in this project, so users don't have to compile twice.
- [Package System Binaries](https://rust-cli.github.io/book/tutorial/packaging.html)
- Configuration file like `.mallardscriptrc`.
- Source errors would be really nice to have.
  Can be implemented with:
  - [miette](https://crates.io/crates/miette).
  - [pest miette](https://github.com/pest-parser/pest/issues/582).
  - [ariadne](https://github.com/zesterer/ariadne)?
