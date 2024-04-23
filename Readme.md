<img src="https://github.com/Toni-Graphics/CodeGenLib/assets/127748753/83223f5c-72f0-4b20-8380-dd9ec075551b">

A libary to generate x86-64Bit machine code

> **Warning:** this libary is currently undergoing big changes so don't use in production

## Example
```rust
use CodeGenLib::ir::IrBuilder;

pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut builder = IrBuilder::new();

    let add = builder.add("add");
    add.args(vec![
        ("x", 4),
        ("y", 4),
    ]);

    add.vars(vec![
        ("z", 4),
    ]);

    add.build_arg_add("x", "y", "z")?;
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
