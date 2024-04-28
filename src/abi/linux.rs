use lazy_static::lazy_static;
use iced_x86::Register::*;

use super::Abi;

lazy_static! {
    pub static ref ABI: Abi = Abi {
        reg_args: 6, 
        regs_64: vec![RDI , RSI , RDX , RCX , R8 , R9], 
        regs_32: vec![EDI, ESI, EDX, ECX, R8D, R9D], 
    };
}