use std::{fs::File, path::Path};

use crate::x86::function::Function;
use faerie::{ArtifactBuilder, ArtifactError, Decl};
use target_lexicon::{Architecture, BinaryFormat, Environment, OperatingSystem, Triple, Vendor};

pub struct Builder {
    functions: Vec<Function>,
}

impl Builder {
    pub fn new() -> Self {
        Self {
            functions: vec![],
        }
    }

    pub fn add_function(&mut self, name: &str) -> &mut Function {
        let func = Function::new(name);
        self.functions.push( func );
        self.get_last_fn()
    }

    fn get_last_fn(&mut self) -> &mut Function {
        let list = self.functions.clone();
        self.functions.get_mut(list.len() -1)
        .expect("error while getting last function (CodeGenLib/x86/builder.rs/21")
    }

    pub fn build(&mut self, name: &str) -> Result<(), ArtifactError> {
        let file = File::create(Path::new(name))?;
        let mut obj = ArtifactBuilder::new(Triple {
            architecture: Architecture::host(),
            vendor: Vendor::host(),
            operating_system: OperatingSystem::Linux,
            environment: Environment::host(),
            binary_format: BinaryFormat::host(),
        })
            .name(name.to_owned())
            .finish();

        // add declerations
        let mut decls: Vec<(String, Decl)> = vec![];

        for func in self.functions.iter() {
            decls.push( (func.name.to_owned(), Decl::function().global().into() ) );
        }

        obj.declarations( decls.iter().cloned() )?;

        // add functions
        for func in self.functions.iter() {
            let gen = func.get_gen();

            obj.define(func.name.to_string(), gen)?;
        }

        obj.write(file)?;

        Ok(())
    }
}