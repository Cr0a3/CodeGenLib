use crate::{ir::AsmInstructionEnum, ir::resolve::resolve, Optimize};
use formatic::{Arch, BinFormat, Decl, Endian, Link, ObjectBuilder, Scope};
use std::collections::HashMap;

/// The builder is a wrapper around the entire code generation
///
/// It also create the object file via the `formatic` crate
#[derive(Debug, Clone)]
pub struct Builder {
    pub funcs: HashMap<String, (bool, Vec<AsmInstructionEnum>)>,
    pub labels: HashMap<String, (bool, Vec<u8>)>,
    pub func_names: Vec<String>,
    pub label_names: Vec<String>,
}

impl Builder {
    /// Creates a new instance of Builders
    pub fn new() -> Self {
        Self {
            funcs: HashMap::new(),
            labels: HashMap::new(),
            func_names: vec![],
            label_names: vec![],
        }
    }

    pub fn define(
        &mut self,
        name: &str,
        public: bool,
        code: Vec<AsmInstructionEnum>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let opt_code = Optimize(code)?;
        self.funcs.insert(name.into(), (public, opt_code));
        self.func_names.push(name.into());

        Ok(())
    }

    pub fn define_label(
        &mut self,
        name: &str,
        public: bool,
        data: Vec<u8>,
    ) {
        self.labels.insert(name.into(), (public, data));
        self.label_names.push(name.into());
    }

    pub fn write(&mut self, outpath: &str, bin: BinFormat) -> Result<(), Box<dyn std::error::Error>> {
        let mut obj = ObjectBuilder::new(outpath);

        let mut resolved_funcs: HashMap<String, Vec<u8>> = HashMap::new();

        // Resolve machine code
        for func in self.funcs.iter() {
            let ir = &func.1 .1;

            let resolved = resolve(self.func_names.clone(), self.label_names.clone(), &ir)?;

            resolved_funcs.insert(func.0.to_owned(), resolved.0);

            // add decls
            let decls = resolved.2;
            for decl in decls {
                obj.add_decl(&decl.0, decl.1);
            }

            // add links
            let links = resolved.1;
            for link in links {
                if self.label_names.contains(&link.to) {
                    obj.link(Link {
                        from: func.0.to_string(),
                        to: format!(".L{}", link.to),
                        at: link.at,
                    });

                } else {
                    obj.link(Link {
                        from: func.0.to_string(),
                        to: link.to,
                        at: link.at,
                    });
                }
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

        // Defining labels
        for label in self.labels.iter() {

            let name = format!(".L{}", label.0);

            obj.add_decl(&name, Decl::Data({
                match label.1.0 {
                    true => Scope::Export,
                    false => Scope::Private,
                }
            }));

            obj.define(&name, label.1.1.to_owned());
        }

        obj.write(bin, Arch::X86_64, Endian::Litte)
    }

    /// Syncronices the symbols/links/etc. from the other builder into the current if they doesn't exits
    pub fn sync(&mut self, other: &Builder) {
        for label in other.labels.keys() {
            let data = other.labels.get(label).unwrap().to_owned();

            if ! self.labels.contains_key(label) {
                self.labels.insert(label.to_owned(), data);
                self.label_names.push(label.to_owned());
            }
        }

        for func in other.funcs.keys() {
            let data = other.funcs.get(func).unwrap().to_owned();

            if ! self.funcs.contains_key(func) {
                self.funcs.insert(func.to_owned(), data);
                self.func_names.push(func.to_owned());
            }
        }
    }
}
