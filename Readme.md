<img src="https://github.com/Toni-Graphics/CodeGenLib/assets/127748753/83223f5c-72f0-4b20-8380-dd9ec075551b">

A libary to generate x86-64Bit machine code

> **Warning:** this libary is currently undergoing big changes so don't use in production

## Example
```rust
use std::error::Error;
use CodeGenLib::{Builder, IR::*};

#[rustfmt::skip]
pub fn main() -> Result<(), Box<dyn Error>> {
    let mut builder = Builder::new();

    builder.define("call", true, vec![
        Call("callme"),
        MovVal(Register::EAX, 5),
    ])?;

    builder.write("tmp/test.o")?;

    Ok(())
}

```

## Documentation
Check out our documentation on https://docs.rs/CodeGenLib/latest/CodeGenLib/

## Copyright
Copyright (C) 2024 Cr0a3

(!) Uses the faerie crate (https://crates.io/crates/faerie)