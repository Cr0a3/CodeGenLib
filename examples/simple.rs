use std::error::Error;
use CodeGenLib::{asm::arg, BinFormat, Builder, IR::*};

#[rustfmt::skip]
pub fn main() -> Result<(), Box<dyn Error>> {
    let mut builder = Builder::new();

    builder.define("call", true, vec![
        Call("callme"),
        MovVal(Register::EAX, 5),
    ])?;

    builder.write("tmp/test.o", BinFormat::host())?;

    Ok(())
}
