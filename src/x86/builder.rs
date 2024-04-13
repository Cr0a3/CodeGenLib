use crate::{asm::AsmInstructionEnum, ir::SafeCode, resolve, Optimize};
use formatic::{Arch, BinFormat, Decl, Endian, Link, ObjectBuilder, Scope};
use std::collections::HashMap;

/// The builder is a wrapper around the entire code generation
///
/// It also create the object file via the `formatic` crate
pub struct Builder {
    funcs: HashMap<String, (bool, Vec<AsmInstructionEnum>)>,
    func_names: Vec<String>,
}

impl Builder {
    /// Creates a new instance of Builders
    pub fn new() -> Self {
        Self {
            funcs: HashMap::new(),
            func_names: vec![],
        }
    }

    pub fn define(
        &mut self,
        name: &str,
        public: bool,
        code: Vec<AsmInstructionEnum>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let code = SafeCode(code)?;
        let code = Optimize(code);
        self.funcs.insert(name.into(), (public, code));
        self.func_names.push(name.into());

        Ok(())
    }

    pub fn write(
        &mut self,
        outpath: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut obj = ObjectBuilder::new(outpath);

        let mut resolved_funcs: HashMap<String, Vec<u8>> = HashMap::new();

        // Resolve machine code
        for func in self.funcs.iter() {
            let ir = &func.1 .1;

            let resolved = resolve(self.func_names.clone(), &ir)?;

            resolved_funcs.insert(func.0.to_owned(), resolved.0);

            // add decls
            let decls = resolved.2;
            for decl in decls {
                obj.add_decl(&decl.0, decl.1);
            }

            // add links
            let links = resolved.1;
            for link in links {
                obj.link(Link {
                    from: func.0.to_string(),
                    to: link.to,
                    at: link.at,
                });
            }
        }

        // Defining functions
        for func in resolved_funcs {
            let decl = Decl::Function({
                let public = self.funcs.get(&func.0).unwrap().0;
                match public {
                    true => Scope::Export,
                    false => Scope::Private,
                }
            });

            obj.add_decl(&func.0, decl);

            obj.define(&func.0, func.1);
        }

        obj.write(BinFormat::host(), Arch::X86_64, Endian::Litte)?;
        Ok(())
    }
}
