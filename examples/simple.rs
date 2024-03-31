use faerie::ArtifactError;
use CodeGenLib::Builder;

pub fn main() -> Result<(), ArtifactError>{
    let mut builder = Builder::new();

    let func = builder.add_function("call");
    func.call("callme");
    func.ret_int(5);

    builder.build("test.o")?;

    Ok(())
}