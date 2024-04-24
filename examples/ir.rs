use CodeGenLib::prelude::*;

#[rustfmt::skip]
pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut builder = IrBuilder::new();

    let add = builder.add("add");
    add.args(vec![
        ("x", Type::uInt64(0) ),
        ("y", Type::uInt64(0) ),
    ]);

    add.vars(vec![
        ("z", Type::uInt64(0) ),
    ]);

    add.build_add("x", "y", "z")?;
    add.build_return_var("z")?;

    add.set_public();

    builder.builder()?.write("tmp/ir.o")?;

    Ok(())
}
