use std::io::Write;

use CodeGenLib::asm::{ASMCall, REGISTER};

pub fn main() -> std::io::Result<()> {
    let mut call = ASMCall::new();
    call.mov_16(REGISTER::AX, 16);

    let mut file = std::fs::OpenOptions::new().create(true).write(true).open("out.bin")?;

    let generated = call.generated;
    file.write(&generated)?;

    file.flush()?;

    Ok(())
}