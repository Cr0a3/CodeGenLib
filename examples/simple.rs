use faerie::ArtifactError;
use CodeGenLib::x86::{builder::Builder, asm::REGISTER};

pub fn main() -> Result<(), ArtifactError>{
    let mut builder = Builder::new();

    let func = builder.add_function("ret_five");
    func.mov(REGISTER::EAX, 5);
    func.ret();

    builder.build("test.o")?;

    Ok(())
}