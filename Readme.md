<img src="https://github.com/Toni-Graphics/CodeGenLib/assets/127748753/a8e95f8b-2382-481c-b11a-29065e1f0e73">

A libary to generate x86-64Bit machine code

> **Warning:** this libary is currently undergoing big changes so don't use in production

## Example
```rust
use CodeGenLib::prelude::*;

#[rustfmt::skip]
pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut builder = IrBuilder::new(Abi::host());

    let add = builder.add("add");
    add.args(vec![
        ("x", Type::u64(0) ),
        ("y", Type::u64(0) ),
    ]);

    add.vars(vec![
        ("z", Type::u64(0) ),
    ]);

    add.build_add("x", "y", "z")?;
    add.build_return_var("z")?;

    add.set_public();

    builder.builder()?.write("tmp/ir.o")?;

    Ok(())
}
```

## Documentation
Check out our documentation on https://docs.rs/CodeGenLib/latest/CodeGenLib/

## Copyright
Copyright (C) 2024 Cr0a3
