# CodeGenLib

A libary to generate x86-64Bit machine code

## Example
```rust
use faerie::ArtifactError;
use CodeGenLib::Builder;

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