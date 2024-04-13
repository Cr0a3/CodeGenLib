use std::error::Error;
use CodeGenLib::{asm::arg32, Builder, IR::*};

#[rustfmt::skip]
pub fn main() -> Result<(), Box<dyn Error>> {
    let mut builder = Builder::new();

    builder.define("add", true, vec![
        MovReg(Register::EBX, arg32(1)),
        MovReg(Register::EAX, arg32(2)),
        AddReg(Register::EAX, Register::EBX),
    ])?;

    builder.write("tmp/add.o")?;

    Ok(())
}
