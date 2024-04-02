use std::{fs::File, path::Path};
use object::write::{Object, Relocation, StandardSection, Symbol, SymbolScope, SymbolSection};
use object::{
    Architecture, BinaryFormat, Endianness, RelocationEncoding, RelocationFlags, RelocationKind, SymbolFlags, SymbolKind
};
use super::{function::*, staticv::*};
use super::mem::AdressManager;
use crate::OptimizeTrait;

/// The builder is a wrapper around the entire code generation
/// via the classes: <br>
/// `Function`, `StaticValue`, `ExternFunction`, `AdressManager`
/// 
/// It also create the object file via the `faerie` crate
#[derive(Clone)]
pub struct Builder {
    functions: Vec<Function>,
    statics: Vec<StaticValue>,
    externs: Vec<ExternFunction>,
    mem: AdressManager
}

impl Builder {
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
    pub fn add_function(& mut self, name: &str) -> &mut Function {
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
    pub fn add_static(& mut self, name: &str, global: bool) -> &mut StaticValue {
        let stat = StaticValue::new(name, global);
        self.statics.push( stat );
        let list = self.statics.clone();
        self.statics.get_mut(list.len() -1)
            .expect("error while getting last staic value (CodeGenLib/x86/builder.rs/47")
    }
    
    /// Adds function import from another file
    pub fn add_extern_fn(& mut self, name: &str) -> &mut ExternFunction {
        let func = ExternFunction { name: name.into() };
        self.externs.push( func );
        let list = self.externs.clone();
        self.externs.get_mut(list.len() -1)
        .expect("error while getting last function (CodeGenLib/x86/builder.rs/53")
    }

    /// Builds all functions, symbols, etc into one
    /// object file with the name `name`
    pub fn build(& mut self, name: &str, format: BinaryFormat) -> Result<(), Box<dyn std::error::Error> > {
        let mut obj = Object::new(format, Architecture::X86_64, Endianness::Little);

        obj.add_file_symbol(b"test.o".into());

        // optimize
        for func in self.functions.iter_mut() {
            func.optimize();
        }
        
        for func in self.functions.iter() {
            let (section, offset)  = 
                obj.add_subsection(StandardSection::Text, func.name.as_bytes(), &func.clone().get_gen(), 16);
            
            obj.add_symbol(Symbol {
                name: func.name.clone().into(),
                value: offset,
                size: func.clone().get_gen().len() as u64,
                kind: SymbolKind::Text,
                scope: SymbolScope::Linkage,
                weak: false,
                section: SymbolSection::Section(section),
                flags: SymbolFlags::None,     
            });

            for sym in func.esymbols.iter() { 
                let symid = obj.add_symbol(Symbol {
                    name: sym.dest.as_bytes().into(),
                    value: 0,
                    size: 0,
                    kind: SymbolKind::Text,
                    scope: SymbolScope::Dynamic,
                    weak: false,
                    section: SymbolSection::Undefined,
                    flags: SymbolFlags::None,
                });

                obj.add_relocation(
                    section,
                    Relocation {
                        offset: offset + sym.at as u64,
                        symbol: symid,
                        addend: -4,
                        flags: RelocationFlags::Generic {
                            kind: RelocationKind::PltRelative,
                            encoding: RelocationEncoding::Generic,
                            size: 32,
                        },
                    }
                )?;
            }
        }

        for efunc in self.externs.iter() {
            obj.add_symbol(Symbol {
                name: efunc.name.as_bytes().into(),
                value: 0,
                size: 0,
                kind: SymbolKind::Text,
                scope: SymbolScope::Dynamic,
                weak: false,
                section: SymbolSection::Undefined,
                flags: SymbolFlags::None, 
            });
        }

        for stat in self.statics.iter() {
            let (section, offset) =
                obj.add_subsection(StandardSection::ReadOnlyData, name.as_bytes(), &stat.value, 16);

            obj.add_symbol(Symbol {
                name: name.into(),
                value: offset,
                size: stat.value.len() as u64,
                kind: SymbolKind::Data,
                scope: SymbolScope::Compilation,
                weak: false,
                section: SymbolSection::Section(section),
                flags: SymbolFlags::None,
            });
        }
        
        let file = File::create(Path::new(name))?;
        obj.write_stream(file)?;

        Ok(())
    }
}