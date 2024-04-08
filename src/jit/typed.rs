//! <h4>Jit execution</h4>
//! With this module you can jit execute a function

// use crate::arch::AsmCall::AsmCall as asm;
// use crate::arch::ext::AMD64::*;
/*use crate::x86::function::Function;
use crate::{Result, CodeGenLibError};

pub trait JitRuntime {
    /// Returns the function
    unsafe fn typed<T, X>(&mut self) -> Result<extern "C" fn() -> X>;
}

impl JitRuntime for Function {
    /// Tr
    unsafe fn typed<Params, Ret>(&mut self) -> Result<extern "C"  fn() -> Ret> {
        let func_ptr: extern "C" fn() -> Ret = unsafe {
            std::mem::transmute(self.gen.as_ptr())
        };

        if self.esymbols.len() != 0 {   // ExternSymbols isn't empty
            return Err(CodeGenLibError::JitFunctionsNoExtern)
        }

        Ok(func_ptr)
    }
}*/
