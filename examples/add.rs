use std::error::Error;
use CodeGenLib::{asm::arg, BinFormat, Builder, IR::*};

#[rustfmt::skip]
pub fn main() -> Result<(), Box<dyn Error>> {
    let mut builder = Builder::new();

    builder.define("add", true, vec![
        AddVal(Register::RSP, 0xfffffff0),
        Load(Register::EBX, arg(1)),
        Load(Register::EAX, arg(2)),
        AddReg(Register::EAX, Register::EBX),
    ])?;

    builder.write("tmp/add.o", BinFormat::host())?;

    Ok(())
}
