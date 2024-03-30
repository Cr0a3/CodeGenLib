use std::{fs::File, path::Path};
use faerie::{ArtifactBuilder, ArtifactError, Decl, Link};
use target_lexicon::{Architecture, BinaryFormat, Environment, OperatingSystem, Triple, Vendor};
use super::{function::*, staticv::*};
use super::mem::AdressManager;
use crate::OptimizeTrait;

/// The builder is a wrapper around the entire code generation
/// via the classes: <br>
/// `Function`, `StaticValue`, `ExternFunction`, `AdressManager`
/// 
/// It also create the object file via the `faerie` crate
#[derive(Clone)]
pub struct Builder<'a> {
    functions: Vec<Function<'a>>,
    statics: Vec<StaticValue>,
    externs: Vec<ExternFunction>,
    mem: AdressManager
}

impl<'a> Builder<'a> {
    /// Creates a new instance of Builder
    pub fn new() -> Self {
        let memmng = AdressManager::new((0x00, 0x00));  // ToDo: Custom memory ranges
        Self {
            functions: vec![],
            statics: vec![],
            externs: vec![],
            mem: memmng,
        }
    }

    /// Adds a new global function with the name `name`
    /// To the builder
    pub fn add_function(&mut self, name: &str) -> &'a mut Function {
        let func = Function::new(name, &mut self.mem);
        self.functions.push( func );
        let list = self.functions.clone();
        self.functions.get_mut(list.len() -1)
            .expect("error while getting last function (CodeGenLib/x86/builder.rs/39")
    }

    /// Adds referenc to static value to the builder.
    /// 
    /// `name`   - name of the static value
    /// <br>
    /// `global` - import/export from/to other file 
    pub fn add_static(&mut self, name: &str, global: bool) -> StaticValue {
        let stat = StaticValue::new(name, global);
        self.statics.push( stat );
        let list = self.statics.clone();
        self.statics.get_mut(list.len() -1)
            .expect("error while getting last staic value (CodeGenLib/x86/builder.rs/47").to_owned()
    }
    
    /// Adds function import from another file
    pub fn add_extern_fn(&mut self, name: &str) -> &mut ExternFunction {
        let func = ExternFunction { name: name.into() };
        self.externs.push( func );
        let list = self.externs.clone();
        self.externs.get_mut(list.len() -1)
        .expect("error while getting last function (CodeGenLib/x86/builder.rs/53")
    }

    /// Builds all functions, symbols, etc into one
    /// object file with the name `name`
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

        for func in self.functions.iter_mut() {
            func.optimize();
        }

        obj.write(file)?;

        Ok(())
    }
}