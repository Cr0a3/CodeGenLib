use crate::x86::asm::{REGISTER, to_bytes_16, to_bytes_32, to_bytes_64};
use crate::arch::AsmCall::AsmCall;

pub trait I80386 {
    /// Moves value into one of the 32bit register
    pub fn mov_32(register: REGISTER, value: u32) -> Vec<u8>

    /// Add with carry value to 32Bit register
    pub fn adc_32(register: REGISTER, value: u32) -> Vec<u8>,

    /// Pushes all 32bit registers to the stack
    pub fn pushad() -> Vec<u8>,

    /// Pops all 32bit registers from the stack
    pub fn pophad() -> Vec<u8>,

    /// Pushes all 32bit registers to the stack
    pub fn pushfd() -> Vec<u8>,

    /// Pops all 32bit registers from the stack
    pub fn pophfd() -> Vec<u8>,
}

impl I80386 for AsmCall {
    /// Moves value into one of the 32bit register
    pub fn mov_32(register: REGISTER, value: u32) -> Vec<u8> {
        match register {
            REGISTER::EAX =>  {
                let (x1, x2, x3, x4) = to_bytes_32(value); 
                vec![0xb8, x1, x2, x3, x4]
            },
            REGISTER::EBX => {
                let (x1, x2, x3, x4) = to_bytes_32(value);
                vec![0xbb, x1, x2, x3, x4]
            },
            REGISTER::ECX => {
                let (x1, x2, x3, x4) = to_bytes_32(value);
                vec![0xb9, x1, x2, x3, x4]
            },
            REGISTER::EDX => {
                let (x1, x2, x3, x4) = to_bytes_32(value);
                vec![0xba, x1, x2, x3, x4]
            },
            _ => vec![0x00]
        }
    }

    /// Add with carry value to 32Bit register
    pub fn adc_32(register: REGISTER, value: u32) -> Vec<u8> {
        match register {
            REGISTER::EAX => { 
                let (x1, x2, x3, x4) = to_bytes_32(value);
                vec![0x15, x1, x2, x3, x4]
            },
            REGISTER::EBX => { 
                let (x1, x2, x3, x4) = to_bytes_32(value);
                vec![0x81, 0xD3, x1, x2, x3, x4]
            },
            REGISTER::ECX => { 
                let (x1, x2, x3, x4) = to_bytes_32(value);
                vec![0x81, 0xD1, x1, x2, x3, x4]
            },
            REGISTER::EDX => { 
                let (x1, x2, x3, x4) = to_bytes_32(value);
                vec![0x81, 0xD2, x1, x2, x3, x4]
            },
            _ => vec![0x00],
        }
    }
}