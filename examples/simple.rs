use std::error::Error;
use CodeGenLib::{Builder, BinFormat};

pub fn main() -> Result<(), Box<dyn Error>>{
    let mut builder = Builder::new();

    builder.write("test.o", BinFormat::host())?;

    Ok(())
}