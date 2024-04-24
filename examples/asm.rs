use std::error::Error;
use CodeGenLib::{Builder, IR::*};

#[rustfmt::skip]
pub fn main() -> Result<(), Box<dyn Error>> {
    let mut builder = Builder::new();

    builder.define("call", true, vec![
        Call("callme".into()),
        MovVal(Register::EAX, 5),
    ])?;

    builder.write("tmp/asm.o")?;

    Ok(())
}
