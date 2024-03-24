use std::{fs::File, path::Path};
use faerie::{ArtifactBuilder, ArtifactError, Decl, Link};
use target_lexicon::{Architecture, BinaryFormat, Environment, OperatingSystem, Triple, Vendor};
use crate::x86::{function::*, staticv::*};

pub struct Builder {
    functions: Vec<Function>,
    statics: Vec<StaticValue>,
    externs: Vec<ExternFunction>,
}

impl Builder {
    pub fn new() -> Self {
        Self {
            functions: vec![],
            statics: vec![],
            externs: vec![],
        }
    }

    pub fn add_function(&mut self, name: &str) -> &mut Function {
        let func = Function::new(name);
        self.functions.push( func );
        self.get_last_fn()
    }

    pub fn add_static(&mut self, name: &str, global: bool) -> &mut StaticValue {
        let stat = StaticValue::new(name, global);
        self.statics.push( stat );
        self.get_last_st()
    }
    
    pub fn add_extern_fn(&mut self, name: &str) -> &mut ExternFunction {
        let func = ExternFunction { name: name.into() };
        self.externs.push( func );
        self.get_last_exfn()
    }

    fn get_last_fn(&mut self) -> &mut Function {
        let list = self.functions.clone();
        self.functions.get_mut(list.len() -1)
        .expect("error while getting last function (CodeGenLib/x86/builder.rs/41")
    }

    fn get_last_st(&mut self) -> &mut StaticValue {
        let list = self.statics.clone();
        self.statics.get_mut(list.len() -1)
        .expect("error while getting last staic value (CodeGenLib/x86/builder.rs/47")
    }

    fn get_last_exfn(&mut self) -> &mut ExternFunction {
        let list = self.externs.clone();
        self.externs.get_mut(list.len() -1)
        .expect("error while getting last function (CodeGenLib/x86/builder.rs/53")
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

            for sym in func.esymbols.iter() {
                decls.push( (sym.dest.clone(), Decl::function_import().into() )); 
            }
        }

        for efunc in self.externs.iter() {
            decls.push( (efunc.name.to_owned(), Decl::function_import().into() ) );
        }

        for stat in self.statics.iter() {
            let mut decl = Decl::cstring();

            if stat.global { 
                decl = decl.global() 
            }

            decls.push( (stat.name.to_owned(), decl.into() ) );
        }

        obj.declarations( decls.iter().cloned() )?;

        // add functions
        for func in self.functions.iter() {
            let gen = func.get_gen();

            obj.define(func.name.to_string(), gen)?;

            // adding symbols
            for sym in func.esymbols.iter() {
                obj.link(Link { from: &sym.start, to: &sym.dest, at: sym.at as u64 + 1})?;
            }

        }

        obj.write(file)?;

        Ok(())
    }
}