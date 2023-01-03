# MallardScript

![Version](https://img.shields.io/crates/v/mallardscript?style=flat-square)
![Build](https://img.shields.io/travis/Nate-Wilkins/mallardscript/main?style=flat-square)
![Downloads](https://img.shields.io/crates/d/mallardscript?color=%230E0&style=flat-square)
![Open Issues](https://img.shields.io/github/issues-raw/Nate-Wilkins/mallardscript?style=flat-square)
![License](https://img.shields.io/github/license/Nate-Wilkins/mallardscript?color=%2308F&style=flat-square)

> Hak5 DuckyScript extended language compiler.

## Installation

```
cargo install mallardscript
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

- Badges
- Configuration file like `.mallardscriptrc`
