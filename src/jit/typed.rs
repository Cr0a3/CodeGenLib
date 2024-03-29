//! <h4>Jit execution</h4>
//! With this module you can jit execute a function

use crate::asm::ASMCall;
use crate::x86::function::Function;
use crate::{Result, CodeGenLibError};
use super::macros::jit_func;

pub trait JitRuntime {
    /// Returns the function
    fn typed<T, X>(&mut self) -> Result<extern "C" fn() -> X>;
}

impl<'a> JitRuntime for Function<'a> {
    /// Tr
    fn typed<Params, Ret>(&mut self) -> Result<extern "C"  fn() -> Ret> {
        let func_ptr: extern "C" fn() -> Ret = unsafe {
            std::mem::transmute(self.gen.as_ptr())
        };

        if self.gen[0] == 0xF3 && self.gen[1] == 0x0F && self.gen[2] == 0x1E && self.gen[3] ==  0xFA {
            for i in 0..3 {
                self.gen[i] = 0;
            }
        }

        if self.esymbols.len() != 0 {   // ExternSymbols isn't empty
            return Err(CodeGenLibError::JitFunctionsNoExtern)
        }

        Ok(func_ptr)
    }
}