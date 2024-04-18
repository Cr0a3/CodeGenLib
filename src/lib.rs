//! CodeGenLib is a rust libary to generate x86-64Bit machine code (like llvm)
//!
//! <h4>Example</h4>
//!
//! ```
//! uuse std::error::Error;
//! use CodeGenLib::{Builder, IR::*};
//!
//! #[rustfmt::skip]
//! pub fn main() -> Result<(), Box<dyn Error>> {
//!     let mut builder = Builder::new();
//!
//!     builder.define("call", true, vec![
//!         Call("callme"),
//!         MovVal(Register::EAX, 5),
//!     ])?;
//!
//!     builder.write("tmp/test.o")?;
//!
//!     Ok(())
//! }
//!
//! ```
//!
//! The examples would make a elf file with a function named call wich just calls
//! a function named callme and then returns a 1  

#![allow(non_snake_case)]

pub mod error;
pub mod ir;
#[cfg(feature = "jit")]
pub mod jit;
pub mod opt;
pub mod x86;

pub use ir::resolve;
pub use x86::builder::Builder;
pub use x86::*;

pub mod IR {
    pub use crate::x86::asm::AsmInstructionEnum::*;
    pub use iced_x86::Register;
}

pub use opt::Optimize;

//#[cfg(feature = "jit")]
//pub use jit::typed::JitRuntime as Jit;

/// BinaryFormat re-exported
pub use formatic::BinFormat;
