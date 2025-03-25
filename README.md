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
- `{dir}`: working directory as in `pwd`
- `{git}`: repository and branch names, and status icon
- `{time24}`: current time in **hh:mm:ss** 24-hour format

## Features

- [x] Saved history, with arrow navigation
- [x] Handling "quotes"
- [x] Prompt configuration
