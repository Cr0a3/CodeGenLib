use std::collections::HashMap;
use formatic::{Arch, BinFormat, Endian, ObjectBuilder};
use crate::{asm::AsmInstructionEnum, resolve, Optimize};

/// The builder is a wrapper around the entire code generation
/// 
/// It also create the object file via the `formatic` crate
pub struct Builder {
    funcs: HashMap<String, Vec<AsmInstructionEnum>>,
}

impl Builder {
    /// Creates a new instance of Builders
    pub fn new() -> Self {
        Self {
            funcs: HashMap::new(),
        }
    }

    pub fn define(&mut self, name: &str, code: Vec<AsmInstructionEnum>) {
        let code = Optimize(code);
        self.funcs.insert(name.into(), code);
    }

    pub fn write(&mut self, outpath: &str, binformat: BinFormat) -> Result<(), Box<dyn std::error::Error>> {

        let mut obj = ObjectBuilder::new(outpath);

        let mut funcs: HashMap<String, Vec<u8>> = HashMap::new();

        for func in self.funcs.iter() {
            let ir = func.1;
            funcs.insert(func.0.to_owned(), resolve(ir));
        }

        obj.write(	binformat, Arch::X86_64, Endian::Litte)?;
        Ok(())
    }
}