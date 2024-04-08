use crate::arch::AsmCall::AsmCall;
use crate::x86::asm::{to_bytes_16, to_bytes_32, REGISTER};

pub trait IShared {
    /// Inecrements the register by 1
    fn inc(register: REGISTER) -> Vec<u8>;

    /// Decrements the register by 1
    fn dec(register: REGISTER) -> Vec<u8>;

    /// Moves value from one register into another
    fn mov_reg(target: REGISTER, source: REGISTER) -> Vec<u8>;

    /// Moves the value from the register to specified memory adress
    fn to_memory(adress: u64, target: REGISTER) -> Vec<u8>;

    /// Moves the value from the sepcified memory adress into the target register
    fn from_memory(adress: u32, target: REGISTER) -> Vec<u8>;

    /// Pushes the register onto the stack
    fn push(reg: REGISTER) -> Vec<u8>;

    /// Pops the register from the stack
    fn pop(reg: REGISTER) -> Vec<u8>;

    /// Jumps to the specifed adress
    fn jmp(adress: u32) -> Vec<u8>;

    /// Calls the specified adress
    fn call(adress: u32) -> Vec<u8>;

    /// Add with carry value to register `dest` to register `target`
    fn adc_reg(dest: REGISTER, src: REGISTER) -> Vec<u8>;

    /// Calls the adress which is stored into the register
    fn call_reg(target: REGISTER) -> Vec<u8>;

    /// Jumps to the adress which is stored into the register
    fn jmp_reg(target: REGISTER) -> Vec<u8>;

    /// Just endbr64
    fn endbr64() -> Vec<u8>;
}

impl IShared for AsmCall {
    /// Inecrements the register by 1
    fn inc(register: REGISTER) -> Vec<u8> {
        match register {
            REGISTER::RCX => vec![0x48, 0xFF, 0xC1],
            REGISTER::RDX => vec![0x48, 0xFF, 0xC2],
            REGISTER::RBX => vec![0x48, 0xFF, 0xC3],
            REGISTER::RAX => vec![0x48, 0xFF, 0xC0],
            REGISTER::EAX => vec![0xFF, 0xC0],
            REGISTER::EBX => vec![0xFF, 0xC3],
            REGISTER::ECX => vec![0xFF, 0xC1],
            REGISTER::EDX => vec![0xFF, 0xC2],
            REGISTER::AX => vec![0x66, 0xFF, 0xC0],
            REGISTER::BX => vec![0x66, 0xFF, 0xC3],
            REGISTER::CX => vec![0x66, 0xFF, 0xC1],
            REGISTER::DX => vec![0x66, 0xFF, 0xC2],
            REGISTER::AH => vec![0xFE, 0xC4],
            REGISTER::AL => vec![0xFE, 0xC0],
            REGISTER::BH => vec![0xFE, 0xC7],
            REGISTER::BL => vec![0xFE, 0xC3],
            REGISTER::CH => vec![0xFE, 0xC5],
            REGISTER::CL => vec![0xFE, 0xC1],
            REGISTER::DH => vec![0xFE, 0xC6],
            REGISTER::DL => vec![0xFE, 0xC2],
            REGISTER::RSI => vec![0x48, 0xFF, 0xC6],
            REGISTER::RDI => vec![0x48, 0xFF, 0xC7],
            REGISTER::RBP => vec![0x48, 0xFF, 0xC5],
            REGISTER::RSP => vec![0x48, 0xFF, 0xC5],
            _ => vec![0x00],
        }
    }

    /// Decrements the register by 1
    fn dec(register: REGISTER) -> Vec<u8> {
        match register {
            REGISTER::RAX => vec![0x48, 0xFF, 0xC8],
            REGISTER::RBX => vec![0x48, 0xFF, 0xCB],
            REGISTER::RCX => vec![0x48, 0xFF, 0xC9],
            REGISTER::RDX => vec![0x48, 0xFF, 0xCA],
            REGISTER::EAX => vec![0xFF, 0xC8],
            REGISTER::EBX => vec![0xFF, 0xCB],
            REGISTER::ECX => vec![0xFF, 0xC9],
            REGISTER::EDX => vec![0xFF, 0xCA],
            REGISTER::AX => vec![0x66, 0xFF, 0xC8],
            REGISTER::BX => vec![0x66, 0xFF, 0xCB],
            REGISTER::CX => vec![0x66, 0xFF, 0xC9],
            REGISTER::DX => vec![0x66, 0xFF, 0xCA],
            REGISTER::AH => vec![0xFE, 0xCC],
            REGISTER::AL => vec![0xFE, 0xC8],
            REGISTER::BH => vec![0xFE, 0xCF],
            REGISTER::BL => vec![0xFE, 0xCB],
            REGISTER::CH => vec![0xFE, 0xCD],
            REGISTER::CL => vec![0xFE, 0xC9],
            REGISTER::DH => vec![0xFE, 0xCE],
            REGISTER::DL => vec![0xFE, 0xCA],
            REGISTER::RSI => vec![0x48, 0xFF, 0xCE],
            REGISTER::RDI => vec![0x48, 0xFF, 0xCF],
            REGISTER::RBP => vec![0x48, 0xFF, 0xCD],
            REGISTER::RSP => vec![0x48, 0xFF, 0xCC],
            _ => vec![0x00],
        }
    }

    /// Moves value from one register into another
    fn mov_reg(target: REGISTER, source: REGISTER) -> Vec<u8> {
        match target {
            REGISTER::EAX => match source {
                REGISTER::EAX => vec![0x89, 0xC0],
                REGISTER::EBX => vec![0x89, 0xD8],
                REGISTER::ECX => vec![0x89, 0xC8],
                REGISTER::EDX => vec![0x89, 0xD0],
                _ => vec![0x00],
            },
            REGISTER::EBX => match source {
                REGISTER::EAX => vec![0x89, 0xC3],
                REGISTER::EBX => vec![0x89, 0xDB],
                REGISTER::ECX => vec![0x89, 0xCB],
                REGISTER::EDX => vec![0x89, 0xD3],
                _ => vec![0x00],
            },
            REGISTER::ECX => match source {
                REGISTER::EAX => vec![0x89, 0xC1],
                REGISTER::EBX => vec![0x89, 0xD9],
                REGISTER::ECX => vec![0x89, 0xC9],
                REGISTER::EDX => vec![0x89, 0xD1],
                _ => vec![0x00],
            },
            REGISTER::EDX => match source {
                REGISTER::EAX => vec![0x89, 0xC2],
                REGISTER::EBX => vec![0x89, 0xDA],
                REGISTER::ECX => vec![0x89, 0xCA],
                REGISTER::EDX => vec![0x89, 0xD2],
                _ => vec![0x00],
            },
            REGISTER::AX => match source {
                REGISTER::AX => vec![0x66, 0x89, 0xc0],
                REGISTER::BX => vec![0x66, 0x89, 0xD8],
                REGISTER::CX => vec![0x66, 0x89, 0xC8],
                REGISTER::DX => vec![0x66, 0x89, 0xD0],
                _ => vec![0x00],
            },
            REGISTER::BX => match source {
                REGISTER::AX => vec![0x66, 0x89, 0xc3],
                REGISTER::BX => vec![0x66, 0x89, 0xDB],
                REGISTER::CX => vec![0x66, 0x89, 0xCB],
                REGISTER::DX => vec![0x66, 0x89, 0xD3],
                _ => vec![0x00],
            },
            REGISTER::CX => match source {
                REGISTER::AX => vec![0x66, 0x89, 0xc1],
                REGISTER::BX => vec![0x66, 0x89, 0xD9],
                REGISTER::CX => vec![0x66, 0x89, 0xC9],
                REGISTER::DX => vec![0x66, 0x89, 0xD1],
                _ => vec![0x00],
            },
            REGISTER::DX => match source {
                REGISTER::AX => vec![0x66, 0x89, 0xC2],
                REGISTER::BX => vec![0x66, 0x89, 0xDA],
                REGISTER::CX => vec![0x66, 0x89, 0xCA],
                REGISTER::DX => vec![0x66, 0x89, 0xD2],
                _ => vec![0x00],
            },
            REGISTER::AH => match source {
                REGISTER::AH => vec![0x88, 0xE4],
                REGISTER::AL => vec![0x88, 0xC4],
                REGISTER::BH => vec![0x88, 0xFC],
                REGISTER::BL => vec![0x88, 0xDC],
                REGISTER::CH => vec![0x88, 0xEC],
                REGISTER::CL => vec![0x88, 0xCC],
                REGISTER::DH => vec![0x88, 0xF4],
                REGISTER::DL => vec![0x88, 0xD4],
                _ => vec![0x00],
            },
            REGISTER::AL => match source {
                REGISTER::AH => vec![0x88, 0xE0],
                REGISTER::AL => vec![0x88, 0xC0],
                REGISTER::BH => vec![0x88, 0xF8],
                REGISTER::BL => vec![0x88, 0xD8],
                REGISTER::CH => vec![0x88, 0xE8],
                REGISTER::CL => vec![0x88, 0xC8],
                REGISTER::DH => vec![0x88, 0xF0],
                REGISTER::DL => vec![0x88, 0xD0],
                _ => vec![0x00],
            },
            REGISTER::BH => match source {
                REGISTER::AH => vec![0x88, 0xEF],
                REGISTER::AL => vec![0x88, 0xCF],
                REGISTER::BH => vec![0x88, 0xFF],
                REGISTER::BL => vec![0x88, 0xDF],
                REGISTER::CH => vec![0x88, 0xEF],
                REGISTER::CL => vec![0x88, 0xCF],
                REGISTER::DH => vec![0x88, 0xFF],
                REGISTER::DL => vec![0x88, 0xDF],
                _ => vec![0x00],
            },
            REGISTER::BL => match source {
                REGISTER::AH => vec![0x88, 0xE3],
                REGISTER::AL => vec![0x88, 0xC3],
                REGISTER::BH => vec![0x88, 0xFB],
                REGISTER::BL => vec![0x88, 0xDB],
                REGISTER::CH => vec![0x88, 0xEB],
                REGISTER::CL => vec![0x88, 0xCB],
                REGISTER::DH => vec![0x88, 0xF3],
                REGISTER::DL => vec![0x88, 0xD3],
                _ => vec![0x00],
            },
            REGISTER::CH => match source {
                REGISTER::AH => vec![0x88, 0xE5],
                REGISTER::AL => vec![0x88, 0xC5],
                REGISTER::BH => vec![0x88, 0xFD],
                REGISTER::BL => vec![0x88, 0xDD],
                REGISTER::CH => vec![0x88, 0xED],
                REGISTER::CL => vec![0x88, 0xCD],
                REGISTER::DH => vec![0x88, 0xF5],
                REGISTER::DL => vec![0x88, 0xD5],
                _ => vec![0x00],
            },
            REGISTER::CL => match source {
                REGISTER::AH => vec![0x88, 0xE1],
                REGISTER::AL => vec![0x88, 0xC1],
                REGISTER::BH => vec![0x88, 0xF9],
                REGISTER::BL => vec![0x88, 0xD9],
                REGISTER::CH => vec![0x88, 0xE9],
                REGISTER::CL => vec![0x88, 0xC9],
                REGISTER::DH => vec![0x88, 0xF1],
                REGISTER::DL => vec![0x88, 0xD1],
                _ => vec![0x00],
            },
            REGISTER::DH => match source {
                REGISTER::AH => vec![0x88, 0xE6],
                REGISTER::AL => vec![0x88, 0xC6],
                REGISTER::BH => vec![0x88, 0xFE],
                REGISTER::BL => vec![0x88, 0xDE],
                REGISTER::CH => vec![0x88, 0xEE],
                REGISTER::CL => vec![0x88, 0xCE],
                REGISTER::DH => vec![0x88, 0xF6],
                REGISTER::DL => vec![0x88, 0xD6],
                _ => vec![0x00],
            },
            REGISTER::DL => match source {
                REGISTER::AH => vec![0x88, 0xE2],
                REGISTER::AL => vec![0x88, 0xC2],
                REGISTER::BH => vec![0x88, 0xFA],
                REGISTER::BL => vec![0x88, 0xDA],
                REGISTER::CH => vec![0x88, 0xEA],
                REGISTER::CL => vec![0x88, 0xCA],
                REGISTER::DH => vec![0x88, 0xF2],
                REGISTER::DL => vec![0x88, 0xD2],
                _ => vec![0x00],
            },
            REGISTER::RAX => match source {
                REGISTER::RAX => vec![0x48, 0x89, 0xC0],
                REGISTER::RBP => vec![0x48, 0x89, 0xE8],
                REGISTER::RBX => vec![0x48, 0x89, 0xD8],
                REGISTER::RCX => vec![0x48, 0x89, 0xC8],
                REGISTER::RDI => vec![0x48, 0x89, 0xF8],
                REGISTER::RDX => vec![0x48, 0x89, 0xD0],
                REGISTER::RSI => vec![0x48, 0x89, 0xF0],
                REGISTER::RSP => vec![0x48, 0x89, 0xE0],
                _ => vec![0x00],
            },
            REGISTER::RBX => match source {
                REGISTER::RAX => vec![0x48, 0x89, 0xC3],
                REGISTER::RBP => vec![0x48, 0x89, 0xEB],
                REGISTER::RBX => vec![0x48, 0x89, 0xDB],
                REGISTER::RCX => vec![0x48, 0x89, 0xCB],
                REGISTER::RDI => vec![0x48, 0x89, 0xFB],
                REGISTER::RDX => vec![0x48, 0x89, 0xD3],
                REGISTER::RSI => vec![0x48, 0x89, 0xF3],
                REGISTER::RSP => vec![0x48, 0x89, 0xE3],
                _ => vec![0x00],
            },
            REGISTER::RCX => match source {
                REGISTER::RAX => vec![0x48, 0x89, 0xC1],
                REGISTER::RBP => vec![0x48, 0x89, 0xE9],
                REGISTER::RBX => vec![0x48, 0x89, 0xD9],
                REGISTER::RCX => vec![0x48, 0x89, 0xC9],
                REGISTER::RDI => vec![0x48, 0x89, 0xF9],
                REGISTER::RDX => vec![0x48, 0x89, 0xD1],
                REGISTER::RSI => vec![0x48, 0x89, 0xF1],
                REGISTER::RSP => vec![0x48, 0x89, 0xE1],
                _ => vec![0x00],
            },
            REGISTER::RDX => match source {
                REGISTER::RAX => vec![0x48, 0x89, 0xC2],
                REGISTER::RBP => vec![0x48, 0x89, 0xEA],
                REGISTER::RBX => vec![0x48, 0x89, 0xDA],
                REGISTER::RCX => vec![0x48, 0x89, 0xCA],
                REGISTER::RDI => vec![0x48, 0x89, 0xFA],
                REGISTER::RDX => vec![0x48, 0x89, 0xD2],
                REGISTER::RSI => vec![0x48, 0x89, 0xF2],
                REGISTER::RSP => vec![0x48, 0x89, 0xE2],
                _ => vec![0x00],
            },
            REGISTER::RSI => match source {
                REGISTER::RAX => vec![0x48, 0x89, 0xC6],
                REGISTER::RBP => vec![0x48, 0x89, 0xEE],
                REGISTER::RBX => vec![0x48, 0x89, 0xDE],
                REGISTER::RCX => vec![0x48, 0x89, 0xCE],
                REGISTER::RDI => vec![0x48, 0x89, 0xFE],
                REGISTER::RDX => vec![0x48, 0x89, 0xD6],
                REGISTER::RSI => vec![0x48, 0x89, 0xF6],
                REGISTER::RSP => vec![0x48, 0x89, 0xE6],
                _ => vec![0x00],
            },
            REGISTER::RDI => match source {
                REGISTER::RAX => vec![0x48, 0x89, 0xC7],
                REGISTER::RBP => vec![0x48, 0x89, 0xEF],
                REGISTER::RBX => vec![0x48, 0x89, 0xDF],
                REGISTER::RCX => vec![0x48, 0x89, 0xCF],
                REGISTER::RDI => vec![0x48, 0x89, 0xFF],
                REGISTER::RDX => vec![0x48, 0x89, 0xD7],
                REGISTER::RSI => vec![0x48, 0x89, 0xF7],
                REGISTER::RSP => vec![0x48, 0x89, 0xE7],
                _ => vec![0x00],
            },
            REGISTER::RBP => match source {
                REGISTER::RAX => vec![0x48, 0x89, 0xC5],
                REGISTER::RBP => vec![0x48, 0x89, 0xED],
                REGISTER::RBX => vec![0x48, 0x89, 0xDD],
                REGISTER::RCX => vec![0x48, 0x89, 0xCD],
                REGISTER::RDI => vec![0x48, 0x89, 0xFD],
                REGISTER::RDX => vec![0x48, 0x89, 0xD5],
                REGISTER::RSI => vec![0x48, 0x89, 0xF5],
                REGISTER::RSP => vec![0x48, 0x89, 0xE5],
                _ => vec![0x00],
            },
            REGISTER::RSP => match source {
                REGISTER::RAX => vec![0x48, 0x89, 0xC4],
                REGISTER::RBP => vec![0x48, 0x89, 0xEC],
                REGISTER::RBX => vec![0x48, 0x89, 0xDC],
                REGISTER::RCX => vec![0x48, 0x89, 0xCC],
                REGISTER::RDI => vec![0x48, 0x89, 0xFC],
                REGISTER::RDX => vec![0x48, 0x89, 0xD4],
                REGISTER::RSI => vec![0x48, 0x89, 0xF4],
                REGISTER::RSP => vec![0x48, 0x89, 0xE4],
                _ => vec![0x00],
            },
            _ => vec![0x00],
        }
    }

    /// Moves the value from the register to specified memory adress
    fn to_memory(adress: u64, target: REGISTER) -> Vec<u8> {
        match target {
            REGISTER::EAX => {
                let (x1, x2, x3, x4) = to_bytes_32(adress as u32);
                vec![0xA3, x1, x2, x3, x4]
            }
            REGISTER::EBX => {
                let (x1, x2, x3, x4) = to_bytes_32(adress as u32);
                vec![0x89, 0x1D, x1, x2, x3, x4]
            }
            REGISTER::ECX => {
                let (x1, x2, x3, x4) = to_bytes_32(adress as u32);
                vec![0x89, 0x0D, x1, x2, x3, x4]
            }
            REGISTER::EDX => {
                let (x1, x2, x3, x4) = to_bytes_32(adress as u32);
                vec![0x89, 0x15, x1, x2, x3, x4]
            }
            REGISTER::AX => {
                let (x1, x2) = to_bytes_16(adress as u16);
                vec![0x66, 0xa3, x1, x2]
            }
            REGISTER::BX => {
                let (x1, x2) = to_bytes_16(adress as u16);
                vec![0x66, 0x89, 0x1D, x1, x2]
            }
            REGISTER::CX => {
                let (x1, x2) = to_bytes_16(adress as u16);
                vec![0x66, 0x89, 0x0D, x1, x2]
            }
            REGISTER::DX => {
                let (x1, x2) = to_bytes_16(adress as u16);
                vec![0x66, 0x89, 0x15, x1, x2]
            }
            REGISTER::AH => {
                vec![0x88, 0x25, adress as u8]
            }
            REGISTER::AL => {
                vec![0xa2, adress as u8]
            }
            REGISTER::BH => {
                vec![0x88, 0x3d, adress as u8]
            }
            REGISTER::BL => {
                vec![0x88, 0x1D, adress as u8]
            }
            REGISTER::CH => {
                vec![0x88, 0x2D, adress as u8]
            }
            REGISTER::CL => {
                vec![0x88, 0x0D, adress as u8]
            }
            REGISTER::DH => {
                vec![0x88, 0x35, adress as u8]
            }
            REGISTER::DL => {
                vec![0x88, 0x15, adress as u8]
            }
            _ => vec![0x00],
        }
    }

    /// Moves the value from the sepcified memory adress into the target register
    fn from_memory(adress: u32, target: REGISTER) -> Vec<u8> {
        match target {
            REGISTER::EAX => {
                let (x1, x2, x3, x4) = to_bytes_32(adress);
                vec![0xa1, x1, x2, x3, x4]
            }
            REGISTER::EBX => {
                let (x1, x2, x3, x4) = to_bytes_32(adress);
                vec![0x8b, 0x1D, x1, x2, x3, x4]
            }
            REGISTER::ECX => {
                let (x1, x2, x3, x4) = to_bytes_32(adress);
                vec![0x8B, 0x0D, x1, x2, x3, x4]
            }
            REGISTER::EDX => {
                let (x1, x2, x3, x4) = to_bytes_32(adress);
                vec![0x8B, 0x15, x1, x2, x3, x4]
            }
            REGISTER::AX => {
                let (x1, x2, x3, x4) = to_bytes_32(adress);
                vec![0x66, 0xa1, x1, x2, x3, x4]
            }
            REGISTER::BX => {
                let (x1, x2, x3, x4) = to_bytes_32(adress);
                vec![0x66, 0x8b, 0x1D, x1, x2, x3, x4]
            }
            REGISTER::CX => {
                let (x1, x2, x3, x4) = to_bytes_32(adress);
                vec![0x66, 0x8B, 0x0D, x1, x2, x3, x4]
            }
            REGISTER::DX => {
                let (x1, x2, x3, x4) = to_bytes_32(adress);
                vec![0x66, 0x8b, 0x15, x1, x2, x3, x4]
            }
            REGISTER::AH => {
                let (x1, x2, x3, x4) = to_bytes_32(adress);
                vec![0x8a, 0x25, x1, x2, x3, x4]
            }
            REGISTER::AL => {
                let (x1, x2, x3, x4) = to_bytes_32(adress);
                vec![0xa0, x1, x2, x3, x4]
            }
            REGISTER::BH => {
                let (x1, x2, x3, x4) = to_bytes_32(adress);
                vec![0x8a, 0x3d, x1, x2, x3, x4]
            }
            REGISTER::BL => {
                let (x1, x2, x3, x4) = to_bytes_32(adress);
                vec![0x8A, 0x1D, x1, x2, x3, x4]
            }
            REGISTER::CH => {
                let (x1, x2, x3, x4) = to_bytes_32(adress);
                vec![0x8a, 0x2D, x1, x2, x3, x4]
            }
            REGISTER::CL => {
                let (x1, x2, x3, x4) = to_bytes_32(adress);
                vec![0x8a, 0x0D, x1, x2, x3, x4]
            }
            REGISTER::DH => {
                let (x1, x2, x3, x4) = to_bytes_32(adress);
                vec![0x8a, 0x35, x1, x2, x3, x4]
            }
            REGISTER::DL => {
                let (x1, x2, x3, x4) = to_bytes_32(adress);
                vec![0x8a, 0x15, x1, x2, x3, x4]
            }
            _ => vec![0x00],
        }
    }

    /// Pushes the register onto the stack
    fn push(reg: REGISTER) -> Vec<u8> {
        match reg {
            REGISTER::AX => vec![0x66, 0x50],
            REGISTER::CX => vec![0x66, 0x51],
            REGISTER::DX => vec![0x66, 0x52],
            REGISTER::BX => vec![0x66, 0x53],
            REGISTER::RAX => vec![0x50],
            REGISTER::RBX => vec![0x53],
            REGISTER::RCX => vec![0x51],
            REGISTER::RDX => vec![0x52],
            REGISTER::RSI => vec![0x56],
            REGISTER::RDI => vec![0x57],
            REGISTER::RBP => vec![0x55],
            REGISTER::RSP => vec![0x54],
            _ => vec![0x00],
        }
    }

    /// Pops the register from the stack
    fn pop(reg: REGISTER) -> Vec<u8> {
        match reg {
            REGISTER::AX => vec![0x66, 0x58],
            REGISTER::CX => vec![0x66, 0x59],
            REGISTER::DX => vec![0x66, 0x5a],
            REGISTER::BX => vec![0x66, 0x5b],
            REGISTER::RAX => vec![0x58],
            REGISTER::RBX => vec![0x59],
            REGISTER::RCX => vec![0x5b],
            REGISTER::RDX => vec![0x5a],
            REGISTER::RSI => vec![0x5e],
            REGISTER::RDI => vec![0x5f],
            REGISTER::RBP => vec![0x5d],
            REGISTER::RSP => vec![0x5c],
            _ => vec![0x00],
        }
    }

    /// Jumps to the specifed adress
    fn jmp(adress: u32) -> Vec<u8> {
        let (x1, x2, x3, x4) = to_bytes_32(adress);
        vec![0xe9, x1, x2, x3, x4]
    }

    /// Calls the specified adress
    fn call(adress: u32) -> Vec<u8> {
        let (x1, x2, x3, x4) = to_bytes_32(adress);
        vec![0xe8, x1, x2, x3, x4]
    }

    /// Add with carry value to register `dest` to register `target`
    fn adc_reg(dest: REGISTER, src: REGISTER) -> Vec<u8> {
        match dest {
            REGISTER::RAX => match src {
                REGISTER::RAX => vec![0x48, 0x11, 0xC0],
                REGISTER::RBX => vec![0x48, 0x11, 0xC3],
                REGISTER::RCX => vec![0x48, 0x11, 0xC1],
                REGISTER::RDX => vec![0x48, 0x11, 0xC2],
                REGISTER::RBP => vec![0x48, 0x11, 0xC5],
                REGISTER::RSI => vec![0x48, 0x11, 0xC6],
                REGISTER::RDI => vec![0x48, 0x11, 0xC7],
                REGISTER::RSP => vec![0x48, 0x11, 0xC4],
                _ => vec![0],
            },
            REGISTER::RBX => match src {
                REGISTER::RAX => vec![0x48, 0x11, 0xD8],
                REGISTER::RBX => vec![0x48, 0x11, 0xDB],
                REGISTER::RCX => vec![0x48, 0x11, 0xD9],
                REGISTER::RDX => vec![0x48, 0x11, 0xDA],
                REGISTER::RBP => vec![0x48, 0x11, 0xDD],
                REGISTER::RSI => vec![0x48, 0x11, 0xDE],
                REGISTER::RDI => vec![0x48, 0x11, 0xDF],
                REGISTER::RSP => vec![0x48, 0x11, 0xDC],
                _ => vec![0],
            },
            REGISTER::RCX => match src {
                REGISTER::RAX => vec![0x48, 0x11, 0xC8],
                REGISTER::RBX => vec![0x48, 0x11, 0xCB],
                REGISTER::RCX => vec![0x48, 0x11, 0xC9],
                REGISTER::RDX => vec![0x48, 0x11, 0xCA],
                REGISTER::RBP => vec![0x48, 0x11, 0xCD],
                REGISTER::RSI => vec![0x48, 0x11, 0xCE],
                REGISTER::RDI => vec![0x48, 0x11, 0xCF],
                REGISTER::RSP => vec![0x48, 0x11, 0xCC],
                _ => vec![0],
            },
            REGISTER::RDX => match src {
                REGISTER::RAX => vec![0x48, 0x11, 0xD0],
                REGISTER::RBX => vec![0x48, 0x11, 0xD3],
                REGISTER::RCX => vec![0x48, 0x11, 0xD1],
                REGISTER::RDX => vec![0x48, 0x11, 0xD2],
                REGISTER::RBP => vec![0x48, 0x11, 0xD5],
                REGISTER::RSI => vec![0x48, 0x11, 0xD6],
                REGISTER::RDI => vec![0x48, 0x11, 0xD7],
                REGISTER::RSP => vec![0x48, 0x11, 0xD4],
                _ => vec![0],
            },
            REGISTER::RSI => match src {
                REGISTER::RAX => vec![0x48, 0x11, 0xF0],
                REGISTER::RBX => vec![0x48, 0x11, 0xF3],
                REGISTER::RCX => vec![0x48, 0x11, 0xF1],
                REGISTER::RDX => vec![0x48, 0x11, 0xF2],
                REGISTER::RBP => vec![0x48, 0x11, 0xF5],
                REGISTER::RSI => vec![0x48, 0x11, 0xF6],
                REGISTER::RDI => vec![0x48, 0x11, 0xF7],
                REGISTER::RSP => vec![0x48, 0x11, 0xF4],
                _ => vec![0],
            },
            REGISTER::RDI => match src {
                REGISTER::RAX => vec![0x48, 0x11, 0xF8],
                REGISTER::RBX => vec![0x48, 0x11, 0xFB],
                REGISTER::RCX => vec![0x48, 0x11, 0xF9],
                REGISTER::RDX => vec![0x48, 0x11, 0xFA],
                REGISTER::RBP => vec![0x48, 0x11, 0xFD],
                REGISTER::RSI => vec![0x48, 0x11, 0xFE],
                REGISTER::RDI => vec![0x48, 0x11, 0xFF],
                REGISTER::RSP => vec![0x48, 0x11, 0xFC],
                _ => vec![0],
            },
            REGISTER::RBP => match src {
                REGISTER::RAX => vec![0x48, 0x11, 0xEA],
                REGISTER::RBX => vec![0x48, 0x11, 0xEB],
                REGISTER::RCX => vec![0x48, 0x11, 0xE9],
                REGISTER::RDX => vec![0x48, 0x11, 0xEA],
                REGISTER::RBP => vec![0x48, 0x11, 0xED],
                REGISTER::RSI => vec![0x48, 0x11, 0xEE],
                REGISTER::RDI => vec![0x48, 0x11, 0xEF],
                REGISTER::RSP => vec![0x48, 0x11, 0xEC],
                _ => vec![0],
            },
            REGISTER::RSP => match src {
                REGISTER::RAX => vec![0x48, 0x11, 0xE0],
                REGISTER::RBX => vec![0x48, 0x11, 0xE3],
                REGISTER::RCX => vec![0x48, 0x11, 0xE1],
                REGISTER::RDX => vec![0x48, 0x11, 0xE2],
                REGISTER::RBP => vec![0x48, 0x11, 0xE5],
                REGISTER::RSI => vec![0x48, 0x11, 0xE6],
                REGISTER::RDI => vec![0x48, 0x11, 0xE7],
                REGISTER::RSP => vec![0x48, 0x11, 0xE4],
                _ => vec![0],
            },
            REGISTER::EAX => match src {
                REGISTER::EAX => vec![0x11, 0xC0],
                REGISTER::EBX => vec![0x11, 0xC3],
                REGISTER::ECX => vec![0x11, 0xC1],
                REGISTER::EDX => vec![0x11, 0xC2],
                _ => vec![0],
            },
            REGISTER::EBX => match src {
                REGISTER::EAX => vec![0x11, 0xD8],
                REGISTER::EBX => vec![0x11, 0xDA],
                REGISTER::ECX => vec![0x11, 0xD9],
                REGISTER::EDX => vec![0x11, 0xDA],
                _ => vec![0],
            },
            REGISTER::ECX => match src {
                REGISTER::EAX => vec![0x11, 0xC8],
                REGISTER::EBX => vec![0x11, 0xCB],
                REGISTER::ECX => vec![0x11, 0xC9],
                REGISTER::EDX => vec![0x11, 0xCA],
                _ => vec![0],
            },
            REGISTER::EDX => match src {
                REGISTER::EAX => vec![0x11, 0xD0],
                REGISTER::EBX => vec![0x11, 0xD3],
                REGISTER::ECX => vec![0x11, 0xD1],
                REGISTER::EDX => vec![0x11, 0xD2],
                _ => vec![0],
            },
            REGISTER::AX => match src {
                REGISTER::AX => vec![0x66, 0x11, 0xC0],
                REGISTER::BX => vec![0x66, 0x11, 0xC3],
                REGISTER::CX => vec![0x66, 0x11, 0xC1],
                REGISTER::DX => vec![0x66, 0x11, 0xC2],
                _ => vec![0],
            },
            REGISTER::BX => match src {
                REGISTER::AX => vec![0x66, 0x11, 0xD8],
                REGISTER::BX => vec![0x66, 0x11, 0xDB],
                REGISTER::CX => vec![0x66, 0x11, 0xD9],
                REGISTER::DX => vec![0x66, 0x11, 0xDA],
                _ => vec![0],
            },
            REGISTER::CX => match src {
                REGISTER::AX => vec![0x66, 0x11, 0xC8],
                REGISTER::BX => vec![0x66, 0x11, 0xCB],
                REGISTER::CX => vec![0x66, 0x11, 0xC9],
                REGISTER::DX => vec![0x66, 0x11, 0xCA],
                _ => vec![0],
            },
            REGISTER::DX => match src {
                REGISTER::AX => vec![0x66, 0x11, 0xD0],
                REGISTER::BX => vec![0x66, 0x11, 0xD3],
                REGISTER::CX => vec![0x66, 0x11, 0xD1],
                REGISTER::DX => vec![0x66, 0x11, 0xD2],
                _ => vec![0],
            },
            REGISTER::AH => match src {
                REGISTER::AH => vec![0x10, 0xE4],
                REGISTER::AL => vec![0x10, 0xE0],
                REGISTER::BH => vec![0x10, 0xE7],
                REGISTER::BL => vec![0x10, 0xE3],
                REGISTER::CH => vec![0x10, 0xE5],
                REGISTER::CL => vec![0x10, 0xE1],
                REGISTER::DH => vec![0x10, 0xE6],
                REGISTER::DL => vec![0x10, 0xE2],
                _ => vec![0],
            },
            REGISTER::AL => match src {
                REGISTER::AH => vec![0x10, 0xC4],
                REGISTER::AL => vec![0x10, 0xC0],
                REGISTER::BH => vec![0x10, 0xC7],
                REGISTER::BL => vec![0x10, 0xC3],
                REGISTER::CH => vec![0x10, 0xC5],
                REGISTER::CL => vec![0x10, 0xC1],
                REGISTER::DH => vec![0x10, 0xC6],
                REGISTER::DL => vec![0x10, 0xC2],
                _ => vec![0],
            },
            REGISTER::BH => match src {
                REGISTER::AH => vec![0x10, 0xFC],
                REGISTER::AL => vec![0x10, 0xF8],
                REGISTER::BH => vec![0x10, 0xFF],
                REGISTER::BL => vec![0x10, 0xFB],
                REGISTER::CH => vec![0x10, 0xFD],
                REGISTER::CL => vec![0x10, 0xF9],
                REGISTER::DH => vec![0x10, 0xFE],
                REGISTER::DL => vec![0x10, 0xFA],
                _ => vec![0],
            },
            REGISTER::BL => match src {
                REGISTER::AH => vec![0x10, 0xDC],
                REGISTER::AL => vec![0x10, 0xD8],
                REGISTER::BH => vec![0x10, 0xDF],
                REGISTER::BL => vec![0x10, 0xDB],
                REGISTER::CH => vec![0x10, 0xDD],
                REGISTER::CL => vec![0x10, 0xD9],
                REGISTER::DH => vec![0x10, 0xDE],
                REGISTER::DL => vec![0x10, 0xDA],
                _ => vec![0],
            },
            REGISTER::CH => match src {
                REGISTER::AH => vec![0x10, 0xEC],
                REGISTER::AL => vec![0x10, 0xE8],
                REGISTER::BH => vec![0x10, 0xEF],
                REGISTER::BL => vec![0x10, 0xEB],
                REGISTER::CH => vec![0x10, 0xED],
                REGISTER::CL => vec![0x10, 0xE9],
                REGISTER::DH => vec![0x10, 0xEE],
                REGISTER::DL => vec![0x10, 0xEA],
                _ => vec![0],
            },
            REGISTER::CL => match src {
                REGISTER::AH => vec![0x10, 0xCC],
                REGISTER::AL => vec![0x10, 0xC8],
                REGISTER::BH => vec![0x10, 0xCF],
                REGISTER::BL => vec![0x10, 0xCB],
                REGISTER::CH => vec![0x10, 0xCD],
                REGISTER::CL => vec![0x10, 0xC9],
                REGISTER::DH => vec![0x10, 0xCE],
                REGISTER::DL => vec![0x10, 0xCA],
                _ => vec![0],
            },
            REGISTER::DH => match src {
                REGISTER::AH => vec![0x10, 0xF4],
                REGISTER::AL => vec![0x10, 0xF0],
                REGISTER::BH => vec![0x10, 0xF7],
                REGISTER::BL => vec![0x10, 0xF3],
                REGISTER::CH => vec![0x10, 0xF5],
                REGISTER::CL => vec![0x10, 0xF1],
                REGISTER::DH => vec![0x10, 0xF6],
                REGISTER::DL => vec![0x10, 0xF2],
                _ => vec![0],
            },
            REGISTER::DL => match src {
                REGISTER::AH => vec![0x10, 0xD4],
                REGISTER::AL => vec![0x10, 0xD0],
                REGISTER::BH => vec![0x10, 0xD7],
                REGISTER::BL => vec![0x10, 0xD3],
                REGISTER::CH => vec![0x10, 0xD5],
                REGISTER::CL => vec![0x10, 0xD1],
                REGISTER::DH => vec![0x10, 0xD6],
                REGISTER::DL => vec![0x10, 0xD2],
                _ => vec![0],
            },
            _ => vec![0x00],
        }
    }

    /// Calls the adress which is stored into the register
    fn call_reg(target: REGISTER) -> Vec<u8> {
        match target {
            REGISTER::RAX => {
                vec![0xFF, 0xD0]
            }
            REGISTER::RBX => {
                vec![0xFF, 0xD3]
            }
            REGISTER::RCX => {
                vec![0xFF, 0xD1]
            }
            REGISTER::RDX => {
                vec![0xFF, 0xD2]
            }
            REGISTER::RSI => {
                vec![0xFF, 0xD6]
            }
            REGISTER::RDI => {
                vec![0xFF, 0xD7]
            }
            REGISTER::RBP => {
                vec![0xFF, 0xD5]
            }
            REGISTER::RSP => {
                vec![0xFF, 0xD4]
            }
            _ => vec![0x00],
        }
    }

    /// Jumps to the adress which is stored into the register
    fn jmp_reg(target: REGISTER) -> Vec<u8> {
        match target {
            REGISTER::RAX => {
                vec![0xFF, 0xE0]
            }
            REGISTER::RBX => {
                vec![0xFF, 0xE3]
            }
            REGISTER::RCX => {
                vec![0xFF, 0xE1]
            }
            REGISTER::RDX => {
                vec![0xFF, 0xE2]
            }
            REGISTER::RSI => {
                vec![0xFF, 0xE6]
            }
            REGISTER::RDI => {
                vec![0xFF, 0xE7]
            }
            REGISTER::RBP => {
                vec![0xFF, 0xE5]
            }
            REGISTER::RSP => {
                vec![0xFF, 0xE4]
            }
            _ => vec![0x00],
        }
    }

    /// Just endbr64
    fn endbr64() -> Vec<u8> {
        vec![0xF3, 0x0F, 0x1E, 0xFA]
    }
}
