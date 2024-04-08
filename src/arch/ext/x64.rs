use crate::arch::AsmCall::AsmCall;
use crate::x86::asm::{to_bytes_32, to_bytes_64, REGISTER};

pub trait X64 {
    /// Moves value into one of the 64bit register
    fn mov_64(register: REGISTER, value: u64) -> Vec<u8>;

    /// Add with carry value to 64Bit register
    fn adc_64(register: REGISTER, value: u64) -> Vec<u8>;
}

impl X64 for AsmCall {
    /// Moves value into one of the 64bit register
    fn mov_64(register: REGISTER, value: u64) -> Vec<u8> {
        match register {
            REGISTER::RAX => {
                let (x1, x2, x3, x4, x5, x6, x7, x8) = to_bytes_64(value);
                vec![0x48, 0xa3, x1, x2, x3, x4, x5, x6, x7, x8]
            }
            REGISTER::RBX => {
                let (x1, x2, x3, x4, x5, x6, x7, x8) = to_bytes_64(value);
                vec![0x48, 0xbb, x1, x2, x3, x4, x5, x6, x7, x8]
            }
            REGISTER::RCX => {
                let (x1, x2, x3, x4, x5, x6, x7, x8) = to_bytes_64(value);
                vec![0x48, 0xb9, x1, x2, x3, x4, x5, x6, x7, x8]
            }
            REGISTER::RDX => {
                let (x1, x2, x3, x4, x5, x6, x7, x8) = to_bytes_64(value);
                vec![0x48, 0xba, x1, x2, x3, x4, x5, x6, x7, x8]
            }
            _ => vec![0],
        }
    }
    /// Add with carry value to 64Bit register
    fn adc_64(register: REGISTER, value: u64) -> Vec<u8> {
        if value > u32::MAX as u64 {
            match register {
                REGISTER::RAX => {
                    let (x1, x2, x3, x4) = to_bytes_32((value - 0xffffffff) as u32);
                    vec![0x48, 0x11, 0x14, 0x25, x1, x2, x3, x4]
                }
                REGISTER::RBX => {
                    let (x1, x2, x3, x4) = to_bytes_32((value - 0xffffffff) as u32);
                    vec![0x48, 0x11, 0x1C, 0x25, x1, x2, x3, x4]
                }
                REGISTER::RCX => {
                    let (x1, x2, x3, x4) = to_bytes_32((value - 0xffffffff) as u32);
                    vec![0x48, 0x11, 0x0C, 0x25, x1, x2, x3, x4]
                }
                REGISTER::RDX => {
                    let (x1, x2, x3, x4) = to_bytes_32((value - 0xffffffff) as u32);
                    vec![0x48, 0x11, 0x14, 0x25, x1, x2, x3, x4]
                }
                REGISTER::RSI => {
                    let (x1, x2, x3, x4) = to_bytes_32((value - 0xffffffff) as u32);
                    vec![0x48, 0x81, 0xD6, x1, x2, x3, x4]
                }
                REGISTER::RDI => {
                    let (x1, x2, x3, x4) = to_bytes_32((value - 0xffffffff) as u32);
                    vec![0x48, 0x11, 0x34, 0x25, x1, x2, x3, x4]
                }
                REGISTER::RBP => {
                    let (x1, x2, x3, x4) = to_bytes_32((value - 0xffffffff) as u32);
                    vec![0x48, 0x11, 0x2C, 0x25, x1, x2, x3, x4]
                }
                REGISTER::RSP => {
                    let (x1, x2, x3, x4) = to_bytes_32((value - 0xffffffff) as u32);
                    vec![0x48, 0x11, 0x24, 0x25, x1, x2, x3, x4]
                }
                _ => vec![0],
            }
        } else {
            match register {
                REGISTER::RAX => {
                    let (x1, x2, x3, x4) = to_bytes_32(value as u32);
                    vec![0x48, 0x11, 0x05, x1, x2, x3, x4]
                }
                REGISTER::RBX => {
                    let (x1, x2, x3, x4) = to_bytes_32(value as u32);
                    vec![0x48, 0x11, 0x1D, x1, x2, x3, x4]
                }
                REGISTER::RCX => {
                    let (x1, x2, x3, x4) = to_bytes_32(value as u32);
                    vec![0x48, 0x11, 0x0D, x1, x2, x3, x4]
                }
                REGISTER::RDX => {
                    let (x1, x2, x3, x4) = to_bytes_32(value as u32);
                    vec![0x48, 0x11, 0x15, x1, x2, x3, x4]
                }
                REGISTER::RSI => {
                    let (x1, x2, x3, x4) = to_bytes_32(value as u32);
                    vec![0x48, 0x11, 0x35, x1, x2, x3, x4]
                }
                REGISTER::RDI => {
                    let (x1, x2, x3, x4) = to_bytes_32(value as u32);
                    vec![0x48, 0x11, 0x3D, x1, x2, x3, x4]
                }
                REGISTER::RBP => {
                    let (x1, x2, x3, x4) = to_bytes_32(value as u32);
                    vec![0x48, 0x11, 0x2D, x1, x2, x3, x4]
                }
                REGISTER::RSP => {
                    let (x1, x2, x3, x4) = to_bytes_32(value as u32);
                    vec![0x48, 0x11, 0x25, x1, x2, x3, x4]
                }
                _ => vec![0],
            }
        }
    }
}
