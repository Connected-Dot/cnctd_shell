# cnctd_shell

Simple shell command execution wrapper for Rust applications.

## Installation

```toml
[dependencies]
cnctd_shell = { git = "https://github.com/Connected-Dot/cnctd_crates.git" }
```

## Usage

```rust
use cnctd_shell::{run, run_silent};

// Run command and capture output
let output = run("ls -la")?;

// Run silently (no stdout)
let output = run_silent("git status")?;
```

## Documentation

See [CLAUDE.md](./CLAUDE.md) for detailed documentation.
