use crate::x86::asm::{REGISTER, to_bytes_16, to_bytes_32, to_bytes_64};
use crate::arch::AsmCall::AsmCall;

pub trait I8086 {
    /// Add with carry value to 8Bit register
    pub fn adc_8(register: REGISTER, value: u8) -> Vec<u8>,
    
    /// Add with carry value to 16Bit register
    pub fn adc_16(register: REGISTER, value: u16) -> Vec<u8>,

    /// Moves value into one of the 8bit register
    pub fn mov_8(register: REGISTER, value: u8) -> Vec<u8>,

    /// Moves value into one of the 16bit register
    pub fn mov_16(register: REGISTER, value: u16) -> Vec<u8>,

    /// Does nothing
    pub fn nop() -> Vec<u8>,

    /// Return
    pub fn ret() -> Vec<u8>,

    /// Calls an interupt
    pub fn int(nr: u8) -> Vec<u8>,

    /// deactivates interrupts
    pub fn cli() -> Vec<u8>,

    /// activates interrupts
    pub fn sti() -> Vec<u8>,

    /// Pushes flags to stack
    pub fn pusfq() -> Vec<u8>,

    /// Pops flags from stack
    pub fn popfq() -> Vec<u8>,

    /// Waits until BUSY# pin is inactive
    pub fn wait() -> Vec<u8>,

    /// Set carry flag
    pub fn stc() -> Vec<u8>,

    /// Clear carry flag
    pub fn clc() -> Vec<u8>,

    /// Set direction flag
    pub fn std() -> Vec<u8>,

    /// Clear direction flag
    pub fn cld() -> Vec<u8>,

    /// Convert byte to word
    pub fn cbw() -> Vec<u8>,
}

impl I8086 for AsmCall {
    /// Add with carry value to 16Bit register
    pub fn adc_16(register: REGISTER, value: u16) -> Vec<u8> {
        match register {
            REGISTER::AX => { 
                let (x1, x2) = to_bytes_16(value);
                vec![0x66, 0x15, x1, x2]
            },
            REGISTER::BX => { 
                let (x1, x2) = to_bytes_16(value);
                vec![0x66, 0x81, 0xD3, x1, x2]
            },
            REGISTER::CX => { 
                let (x1, x2) = to_bytes_16(value);
                vec![0x66, 0x81, 0xD1, x1, x2]
            },
            REGISTER::DX => { 
                let (x1, x2) = to_bytes_16(value);
                vec![0x66, 0x81, 0xD2, x1, x2]
            },
            _ => vec![0]
        }
    }

    /// Add with carry value to 8Bit register
    pub fn adc_8(register: REGISTER, value: u8) -> Vec<u8> {
        match register {
            REGISTER::AH => { vec![0x80, 0xD4, value.to_le_bytes()[0]] },
            REGISTER::AL => { vec![0x14, 0xFF, value.to_le_bytes()[0]] },
            REGISTER::BH => { vec![0x80, 0xD7, value.to_le_bytes()[0]] },
            REGISTER::BL => { vec![0x80, 0xD3, value.to_le_bytes()[0]] },
            REGISTER::CH => { vec![0x80, 0xD5, value.to_le_bytes()[0]] },
            REGISTER::CL => { vec![0x80, 0xD1, value.to_le_bytes()[0]] },
            REGISTER::DH => { vec![0x80, 0xD6, value.to_le_bytes()[0]] },
            REGISTER::DL => { vec![0x80, 0xD2, value.to_le_bytes()[0]] },
            _ => vec![0]
        }
    }

    /// Moves value into one of the 16bit register
    pub fn mov_16(register: REGISTER, value: u16) -> Vec<u8> {
        match register {
            REGISTER::AX =>{
                let (x1, x2) = to_bytes_16(value);
                vec![0x66, 0xb8, x1, x2]
            },
            REGISTER::BX =>{
                let (x1, x2) = to_bytes_16(value);
                vec![0x66, 0xbb, x1, x2]
            },
            REGISTER::DX =>{
                let (x1, x2) = to_bytes_16(value);
                vec![0x66, 0xba, x1, x2]
            },
            _ => vec![0]
        }
    }

    /// Moves value into one of the 8bit register
    pub fn mov_8(register: REGISTER, value: u8) -> Vec<u8> {
        match register {
            REGISTER::AH => { vec![0xb4, value] },
            REGISTER::AL => { vec![0xb0, value] },
            REGISTER::BH => { vec![0xb7, value] },
            REGISTER::BL => { vec![0xb3, value] },
            REGISTER::CH => { vec![0xb5, value] },
            REGISTER::CL => { vec![0xb1, value] },
            REGISTER::DH => { vec![0xb6, value] },
            REGISTER::DL => { vec![0xb2, value] },
            _ => vec![0]
        }
    }

    /// Does nothing
    pub fn nop() -> Vec<u8> {
        vec![0x90]
    }

    /// Return
    pub fn ret() -> Vec<u8> {
        vec![0xC3]
    }

    /// Calls an interupt
    pub fn int(nr: u8) -> Vec<u8> {
        vec![0xCd, nr]
    }
    /// deactivates interrupts
    pub fn cli() -> Vec<u8> {
        vec![0xFA]
    }

    /// activates interrupts
    pub fn sti() -> Vec<u8> {
        vec![0xFB]
    }

    /// Pushes flags to stack
    pub fn pusfq() -> Vec<u8> {
        vec![0x9C]
    }

    /// Pops flags from stack
    pub fn popfq() -> Vec<u8> {
        vec![0x9D]
    }

    /// Waits until BUSY# pin is inactive
    pub fn wait() -> Vec<u8> {
        vec![0x9B]
    }

    /// Set carry flag
    pub fn stc() -> Vec<u8> {
        vec![0xF9]
    }

    /// Set direction flag
    pub fn std() -> Vec<u8> {
        vec![0xFD]
    }

    /// Clear carry flag
    pub fn clc() -> Vec<u8> {
        vec![0xF8]
    }

    /// Clear direction flag
    pub fn cld() -> Vec<u8> {
        vec![0xFC]
    }

    /// Convert byte to word
    pub fn cbw() -> Vec<u8> {
        vec![0x66, 0x98]
    }

}