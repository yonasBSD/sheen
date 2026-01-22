# sheen

A minimal, colorful logging library for Rust.

## Quick Start

```rust
fn main() {
    sheen::init();
    sheen::info!("Server started", port = 3000);
}
```

Output:

```
14:32:15 INFO  Server started port=3000
```

## Features

- Colorful, human-readable output
- Structured key=value logging
- Builder pattern configuration
- Zero config defaults

## Installation

```toml
[dependencies]
sheen = "0.1"
```

## Usage
