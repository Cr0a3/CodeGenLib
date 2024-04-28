use lazy_static::lazy_static;
use iced_x86::Register::*;

use super::Abi;

lazy_static! {
    pub static ref ABI: Abi = Abi {
        reg_args: 4, 
        regs_64: vec![RCX, RDX, R8, R9], 
        regs_32: vec![ECX, EDX, R8D, R9D], 
    };
}