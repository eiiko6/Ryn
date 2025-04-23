# Ryn

A cross-platform minimalist shell written in Rust.
For now it only supports interactive use.

## Features

- [x] [rustyline](https://crates.io/crates/rustyline/) features and actions
- [x] Handling quotes (`""` and `''` are handled the same for now)
- [x] Prompt configuration
- [x] Command sequences (`a ; b`)
- [x] Logical operators (`a && b`, `a || b`)
- [x] Pipes (`a | b | c`)
- [x] Changing the cursor
- [x] Aliases
- [x] Tab path completion
- [x] Hints
- [ ] Redirections (`>`, `<`)
- [ ] Background jobs (`&`)
- [ ] `Ctrl + Z` handling (currently handled by the parent)
- [ ] Full job control

### Not planned

- [ ] Scripting
- [ ] Environment variables

## Installation

### Prerequisites

Use a [Nerd Font](https://www.nerdfonts.com/) to be able to see prompt icons.

### Installing

```bash
git clone https://github.com/eiiko6/ryn.git
cd ryn
cargo install --path .
```

> **Note:** Add `~/.cargo/bin` to your path

## Configuration

You can create a configuration file in `~/.config/ryn/config`

### Prompt

```conf
prompt = "{user}@{host}> "
```

Available variables:

- `{user}`: username as in `whoami`
- `{host}`: hostname as in `/etc/hostname`
- `{dir}`: full working directory from /
- `{compactdir}` working directory from /, in a compact **c/c/c** format
- `{git}`: repository and branch names, and status icon
- `{time24}`: current time in **hh:mm:ss** 24-hour format
- `{timetaken}`: time taken by the last command

- `{variable ifnotgit}`: uses the variable if not in a gir repo, empty otherwise
  > example: `{dir ifnotgit}` will be `{dir}` if not in a git repo, and empty otherwise

### Cursor

```conf
cursor = blinkingbar
```

Available cursors:

- blinkingblock
- steadyblock
- blinkingunderline
- steadyunderline
- blinkingbar
- steadybar

### Aliases

```conf
alias foo = "echo bar"
```

With this, using `foo` will echo `bar`.
