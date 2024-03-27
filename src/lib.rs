//! CodeGenLib is a rust libary to generate x86-64Bit machine code (like llvm)
//! 
//! <h4>Example</h4>
//! 
//! ```
//! use CodeGenLib::{Builder, ArtifactError}
//! 
//! pub fn main() -> Result<(), ArtifactError> {
//!    let mut builder = Builder::new();
//!
//!    builder.add_function("call")
//!     .call("callme")
//!     .ret_int(1);
//!    
//!    builder.build("test.o")?;
//!    
//!    Ok(())
//! }
//! ```
//! 
//! The examples would make a elf file with a function named call wich just calls 
//! a function named callme and then returns a 1  

pub mod x86;
#[cfg(test)]
mod test;

#[cfg(feature = "jit")]
mod jit;

pub use x86::*;
pub use x86::function::Function as Function;
pub use x86::builder::Builder as Builder;

#[cfg(feature = "jit")]
pub use jit::typed::JIT_Runtime as jit;

/// ArtifactError exportet from the faerie crate
pub use faerie::ArtifactError as ArtifactError;