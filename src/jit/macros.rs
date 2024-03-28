use std::mem::transmute;

pub macro_rules! jit_func {
    ($args: expr, $ret: expr, $machine_code: expr) => {
        unsafe {
            extern "C" fn($args) -> $ret = std::mem::transmute($machine_code.as_ptr())
        }
    };
}