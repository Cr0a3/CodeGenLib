use iced_x86::Register::*;

use super::Abi;

pub trait LinuxAbi {
    /// Returns new ABI struct with Linux ABI values
    fn linux() -> Self;
}

impl LinuxAbi for Abi {
    fn linux() -> Self {
        Abi {
            reg_args: 6, 
            regs_64: vec![RDI , RSI , RDX , RCX , R8 , R9], 
            regs_32: vec![EDI, ESI, EDX, ECX, R8D, R9D], 

            return_reg: RAX,

            stack_base: 8,
        }
    }
}