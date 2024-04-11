use std::error::Error;
use CodeGenLib::{BinFormat, Builder, IR::*};

#[rustfmt::skip]
pub fn main() -> Result<(), Box<dyn Error>> {
    let mut builder = Builder::new();

    builder.define("call", true, vec![
        Call("callme"), 
        MovVal(Register::EAX, 5),
        Ret,
    ]);

    builder.write("tmp/test.o", BinFormat::Coff)?;

    Ok(())
}
