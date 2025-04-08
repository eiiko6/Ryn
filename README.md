# Ryn

A minimalist shell written in Rust.
For now it only supports interactive use.

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
- `{git}`: repository and branch names, and status icon
- `{time24}`: current time in **hh:mm:ss** 24-hour format

- `{variable ifnotgit}`: uses the variable if not in a gir repo, empty otherwise
  > example: `{dir ifnotgit}` will be `{dir}` if not in a git repo, and empty otherwise

## Features

- [x] Saved history, with arrow navigation
- [x] Handling "quotes"
- [x] Prompt configuration
