use faerie::ArtifactError;
use CodeGenLib::x86::builder::Builder;

pub fn main() -> Result<(), ArtifactError>{
    let mut builder = Builder::new();

    builder.add_function("call")
        .call("callme")
        .ret_int(5);

    builder.build("test.o")?;

    Ok(())
}