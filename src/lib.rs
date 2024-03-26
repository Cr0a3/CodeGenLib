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

pub use x86::*;

/// ArtifactError exportet from the faerie crate
pub use faerie::ArtifactError as ArtifactError;