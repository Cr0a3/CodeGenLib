use crate::x86::function::Function;

pub trait JIT_Runtime {
    fn typed<T, X>() -> (fn(T) -> X);
}

impl JIT_Runtime for Function {
    pub fn typed<T, X>() -> (fn(T) -> X) {
        let func_ptr: extern "C" fn(T) -> X = unsafe {
                std::mem::transmute(self.generated.as_ptr()) 
            };

        (func_ptr)
    }
}