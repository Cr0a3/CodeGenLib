use std::error::Error;
use CodeGenLib::{Builder, BinaryFormat};

pub fn main() -> Result<(), Box<dyn Error>>{
    let mut builder = Builder::new();

    let func = builder.add_function("call");
    func.call("callme");
    func.ret_int(5);

    builder.build("test.o", BinaryFormat::Coff)?;

    Ok(())
}