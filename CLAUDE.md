# CLAUDE.md - cnctd_shell

> Brief reference for the shell command execution library.

## Purpose

Simple shell command execution wrapper for running CLI commands from Rust.

## Key Exports

```rust
pub fn run(command: &str) -> Result<String>;
pub fn run_silent(command: &str) -> Result<String>;
```

## Ecosystem Role

- **Used by**: cnctd_docker, cnctd_cli, various build tools

## Version

**0.1.7**

---

*Part of the cnctd monorepo. See `../../../CLAUDE.md` for ecosystem context.*
