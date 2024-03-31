<img src="https://github.com/Toni-Graphics/CodeGenLib/assets/127748753/83223f5c-72f0-4b20-8380-dd9ec075551b">

A libary to generate x86-64Bit machine code

> **Error:** Jit dosn't work

## Example
```rust
use CodeGenLib::{Builder, ArtifactError};

pub fn main() -> Result<(), ArtifactError>{
    let mut builder = Builder::new();

    builder.add_function("call")
        .call("callme")
        .ret_int(5);

    builder.build("test.o")?;

    Ok(())
}
```

## Documentation
Check out our documentation on https://docs.rs/CodeGenLib

## Copyright
Copyright (C) 2024 Cr0a3

(!) Uses the faerie crate (https://crates.io/crates/faerie)