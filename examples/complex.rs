#![allow(non_snake_case)]

use CodeGenLib::prelude::*;

#[rustfmt::skip]
pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut builder = IrBuilder::new(Target::host());

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

    let main = builder.add("main");

    main.efuncs(vec![
        ("printf", vec![Type::Unlim(vec![])]),
        ("add", vec![Type::u64(0), Type::u64(0)]),
    ]);

    main.vars(vec![
        ("a", Type::u64(0)),
        ("b", Type::u64(0)),
        ("c", Type::u64(0)),
    ]);

    main.build_set("a", Type::u64(2))?;
    main.build_set("b", Type::u64(2))?;

    main.build_call("add", vec![Type::InVar("a".into()), Type::InVar("b".into())])?;

    main.build_call(
        "printf", 
        vec![
            Type::Str(b"%d + %d = %d".into()),
            Type::InVar("a".into()),
            Type::InVar("b".into()),
            Type::InVar("c".into()),
            ]
    )?;
    
    main.set_public();

    builder.write("tmp/complex.o")?;

    Ok(())
}
