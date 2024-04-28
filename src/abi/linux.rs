use iced_x86::Register::*;

use super::Abi;

pub trait LinuxAbi {
    fn linux() -> Self;
}

impl LinuxAbi for Abi {
    fn linux() -> Self {
        Abi {
            reg_args: 6, 
            regs_64: vec![RDI , RSI , RDX , RCX , R8 , R9], 
            regs_32: vec![EDI, ESI, EDX, ECX, R8D, R9D], 

            stack_base: 8,

            bin: formatic::BinFormat::Elf,
        }
    }
}