use std::mem::transmute;

macro_rules! jit_func {
    ($args: expr, $ret: expr, $machine_code: expr) => {
        unsafe {
            extern "C" fn($args) -> $ret = transmute($machine_code.as_ptr())
        }
    };
}

pub (crate) use jit_func;