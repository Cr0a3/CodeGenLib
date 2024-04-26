#![allow(non_snake_case)]

use CodeGenLib::prelude::*;

#[rustfmt::skip]
pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut builder = IrBuilder::new();

    let main = builder.add("main");

    main.efuncs(vec![
        ("printf", vec![Type::Str(vec![])])
    ]);

    main.build_call(
        "printf", 
        vec![Type::Str(b"Hello World!".into())]
    )?;
    
    main.set_public();

    builder.builder()?.write("tmp/HelloWorld.o")?;

    Ok(())
}
