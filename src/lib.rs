//! CodeGenLib is a rust libary to generate x86-64Bit machine code (like LLVM)
//!
//! <h4>Example</h4>
//!
//! ```
//!use CodeGenLib::prelude::*;
//!
//!#[rustfmt::skip]
//!pub fn main() -> Result<(), Box<dyn std::error::Error>> {
//!    let mut builder = IrBuilder::new(Target::host());
//!
//!    let add = builder.add("add");
//!    add.args(vec![
//!        ("x", Type::u64(0) ),
//!        ("y", Type::u64(0) ),
//!    ]);
//!
//!    add.vars(vec![
//!        ("z", Type::u64(0) ),
//!    ]);
//!
//!    add.build_add("x", "y", "z")?;
//!    add.build_return_var("z")?;
//!
//!    add.set_public();
//!
//!    builder.write("tmp/ir.o")?;
//!
//!    Ok(())
//!}
//! ```
//!
//! The examples would make a elf file with a function named call wich just calls
//! a function named callme and then returns a 1  

#![allow(non_snake_case)]

pub mod error;
pub mod ir;
pub mod opt;
pub mod x86;
pub mod target;
pub mod exec;

pub use ir::resolve::resolve;
pub use x86::builder::Builder;
pub use x86::*;

/// Most used structs for ir work exported
pub mod IR {
    pub use crate::ir::AsmInstructionEnum::*;
    pub use crate::Builder;
    pub use iced_x86::Register;
}

pub use opt::Optimize;

/// Most used structs for high level interface exported
pub mod prelude {
    pub use crate::ir::IrBuilder;
    pub use crate::ir::Type;

    pub use crate::target::Target;

    pub use crate::exec::Intepr;
}

/// BinaryFormat re-exported
pub use formatic::BinFormat;
