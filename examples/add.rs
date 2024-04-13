use std::error::Error;
use CodeGenLib::{asm::{var, arg32}, BinFormat, Builder, IR::*};

#[rustfmt::skip]
pub fn main() -> Result<(), Box<dyn Error>> {
    let mut builder = Builder::new();

    builder.define("add", true, vec![
        Store(arg32(1), var(1, 0)),
        Store(arg32(2), var(2, 4)),
        Load(Register::EDX, var(1, 0)),
        Load(Register::EAX, var(2, 4)),
        AddReg(Register::EAX, Register::EDX),
    ])?;

    builder.write("tmp/add.o", BinFormat::host())?;

    Ok(())
}
