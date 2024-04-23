use CodeGenLib::ir::IrBuilder;

#[rustfmt::skip]
pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut builder = IrBuilder::new();

    let add = builder.add("add");
    add.args(vec![
        ("x", 8),
        ("y", 8),
    ]);

    add.vars(vec![
        ("z", 8),
    ]);

    add.build_add("x", "y", "z")?;
    add.build_return_var("z")?;

    add.set_public();

    builder.builder()?.write("tmp/ir.o")?;

    Ok(())
}
