#![allow(non_snake_case)]

use CodeGenLib::prelude::*;

#[rustfmt::skip]
pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut builder = IrBuilder::new(Target::host());

    let main = builder.add("main");

    main.efuncs(vec![
        ("printf", vec![Type::Unlim(vec![])])
    ]);

    main.build_call(
        "printf", 
        vec![
            Type::Str(b"Hello World %s".into()),
            Type::Str(b"!!".into()),
            ]
    )?;
    
    main.set_public();

    builder.write("tmp/HelloWorld.o")?;

    Ok(())
}
