<img src="https://github.com/Toni-Graphics/CodeGenLib/assets/127748753/83223f5c-72f0-4b20-8380-dd9ec075551b">

A libary to generate x86-64Bit machine code

> **Error:** Jit dosn't work
> **Warning:** this libary is currently undergoing big changes so don't use in production

## Example
```rust
use CodeGenLib::{Builder, BinaryFormat};

pub fn main() -> Result<(), Box<dyn std::error::Error>>{
    let mut builder = Builder::new();

    let func = builder.function(
        "call",
        vec![
            Call("callme"),
            Return(5),
        ]);

    builder.build("test.o", BinaryFormat::Elf)?;

    Ok(())
}
```

## Documentation
Check out our documentation on https://docs.rs/CodeGenLib/latest/CodeGenLib/

## Copyright
Copyright (C) 2024 Cr0a3

(!) Uses the faerie crate (https://crates.io/crates/faerie)