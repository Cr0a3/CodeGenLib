use crate::arch::AsmCall::AsmCall;
use crate::x86::asm::{to_bytes_16, REGISTER};

pub trait I8086 {
    /// Add with carry value to 8Bit register
    fn adc_8(register: REGISTER, value: u8) -> Vec<u8>;

    /// Add with carry value to 16Bit register
    fn adc_16(register: REGISTER, value: u16) -> Vec<u8>;

    /// Moves value into one of the 8bit register
    fn mov_8(register: REGISTER, value: u8) -> Vec<u8>;

    /// Moves value into one of the 16bit register
    fn mov_16(register: REGISTER, value: u16) -> Vec<u8>;

    /// Does nothing
    fn nop() -> Vec<u8>;

    /// Return
    fn ret() -> Vec<u8>;

    /// Calls an interupt
    fn int(nr: u8) -> Vec<u8>;

    /// deactivates interrupts
    fn cli() -> Vec<u8>;

    /// activates interrupts
    fn sti() -> Vec<u8>;

    /// Pushes flags to stack
    fn pusfq() -> Vec<u8>;

    /// Pops flags from stack
    fn popfq() -> Vec<u8>;

    /// Waits until BUSY# pin is inactive
    fn wait() -> Vec<u8>;

    /// Set carry flag
    fn stc() -> Vec<u8>;

    /// Clear carry flag
    fn clc() -> Vec<u8>;

    /// Set direction flag
    fn std() -> Vec<u8>;

    /// Clear direction flag
    fn cld() -> Vec<u8>;

    /// Convert byte to word
    fn cbw() -> Vec<u8>;
}

impl I8086 for AsmCall {
    /// Add with carry value to 16Bit register
    fn adc_16(register: REGISTER, value: u16) -> Vec<u8> {
        match register {
            REGISTER::AX => {
                let (x1, x2) = to_bytes_16(value);
                vec![0x66, 0x15, x1, x2]
            }
            REGISTER::BX => {
                let (x1, x2) = to_bytes_16(value);
                vec![0x66, 0x81, 0xD3, x1, x2]
            }
            REGISTER::CX => {
                let (x1, x2) = to_bytes_16(value);
                vec![0x66, 0x81, 0xD1, x1, x2]
            }
            REGISTER::DX => {
                let (x1, x2) = to_bytes_16(value);
                vec![0x66, 0x81, 0xD2, x1, x2]
            }
            _ => vec![0],
        }
    }

    /// Add with carry value to 8Bit register
    fn adc_8(register: REGISTER, value: u8) -> Vec<u8> {
        match register {
            REGISTER::AH => {
                vec![0x80, 0xD4, value.to_le_bytes()[0]]
            }
            REGISTER::AL => {
                vec![0x14, 0xFF, value.to_le_bytes()[0]]
            }
            REGISTER::BH => {
                vec![0x80, 0xD7, value.to_le_bytes()[0]]
            }
            REGISTER::BL => {
                vec![0x80, 0xD3, value.to_le_bytes()[0]]
            }
            REGISTER::CH => {
                vec![0x80, 0xD5, value.to_le_bytes()[0]]
            }
            REGISTER::CL => {
                vec![0x80, 0xD1, value.to_le_bytes()[0]]
            }
            REGISTER::DH => {
                vec![0x80, 0xD6, value.to_le_bytes()[0]]
            }
            REGISTER::DL => {
                vec![0x80, 0xD2, value.to_le_bytes()[0]]
            }
            _ => vec![0],
        }
    }

    /// Moves value into one of the 16bit register
    fn mov_16(register: REGISTER, value: u16) -> Vec<u8> {
        match register {
            REGISTER::AX => {
                let (x1, x2) = to_bytes_16(value);
                vec![0x66, 0xb8, x1, x2]
            }
            REGISTER::BX => {
                let (x1, x2) = to_bytes_16(value);
                vec![0x66, 0xbb, x1, x2]
            }
            REGISTER::DX => {
                let (x1, x2) = to_bytes_16(value);
                vec![0x66, 0xba, x1, x2]
            }
            _ => vec![0],
        }
    }

    /// Moves value into one of the 8bit register
    fn mov_8(register: REGISTER, value: u8) -> Vec<u8> {
        match register {
            REGISTER::AH => {
                vec![0xb4, value]
            }
            REGISTER::AL => {
                vec![0xb0, value]
            }
            REGISTER::BH => {
                vec![0xb7, value]
            }
            REGISTER::BL => {
                vec![0xb3, value]
            }
            REGISTER::CH => {
                vec![0xb5, value]
            }
            REGISTER::CL => {
                vec![0xb1, value]
            }
            REGISTER::DH => {
                vec![0xb6, value]
            }
            REGISTER::DL => {
                vec![0xb2, value]
            }
            _ => vec![0],
        }
    }

    /// Does nothing
    fn nop() -> Vec<u8> {
        vec![0x90]
    }

    /// Return
    fn ret() -> Vec<u8> {
        vec![0xC3]
    }

    /// Calls an interupt
    fn int(nr: u8) -> Vec<u8> {
        vec![0xCd, nr]
    }
    /// deactivates interrupts
    fn cli() -> Vec<u8> {
        vec![0xFA]
    }

    /// activates interrupts
    fn sti() -> Vec<u8> {
        vec![0xFB]
    }

    /// Pushes flags to stack
    fn pusfq() -> Vec<u8> {
        vec![0x9C]
    }

    /// Pops flags from stack
    fn popfq() -> Vec<u8> {
        vec![0x9D]
    }

    /// Waits until BUSY# pin is inactive
    fn wait() -> Vec<u8> {
        vec![0x9B]
    }

    /// Set carry flag
    fn stc() -> Vec<u8> {
        vec![0xF9]
    }

    /// Set direction flag
    fn std() -> Vec<u8> {
        vec![0xFD]
    }

    /// Clear carry flag
    fn clc() -> Vec<u8> {
        vec![0xF8]
    }

    /// Clear direction flag
    fn cld() -> Vec<u8> {
        vec![0xFC]
    }

    /// Convert byte to word
    fn cbw() -> Vec<u8> {
        vec![0x66, 0x98]
    }
}
