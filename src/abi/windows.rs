use iced_x86::Register::*;

use super::Abi;

pub trait WindowsAbi {
    fn windows() -> Self;
}

impl WindowsAbi for Abi {
    fn windows() -> Self {
        Abi {
            reg_args: 4, 
            regs_64: vec![RCX, RDX, R8, R9], 
            regs_32: vec![ECX, EDX, R8D, R9D], 

            stack_base: 8,

            bin: formatic::BinFormat::Coff,
        }
    }
}