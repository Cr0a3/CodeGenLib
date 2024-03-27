use crate::x86::function::Function;
use crate::JitFunctionError;

//! <h4>Jit execution</h4>
//! With this module you can jit execute a function

pub trait JIT_Runtime {
    /// Returns the function
    fn typed<T, X>() -> Result(fn(T) -> X, JitFunctionError);
}

impl JIT_Runtime for Function {
    fn typed<T, X>() -> (fn(T) -> X) {
        let func_ptr: extern "C" fn(T) -> X = unsafe {
                std::mem::transmute(self.generated.as_ptr()) 
            };

        (func_ptr)
    }
}