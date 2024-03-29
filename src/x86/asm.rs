use std::vec;

/// The name of the register to use
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum REGISTER {
    RAX, RBX, RCX, RDX,                 // 64bit
    EAX, EBX, ECX, EDX,                 // 32bit
    AX, BX, CX, DX,                     // 16bit
    AH, AL, BH, BL, CH, CL, DH, DL,     // 8bit

    RSI, RDI, RBP, RIP, RSP,            // Indexs + Pointers (64bit)
}

fn to_bytes_64(value: u64) -> (u8, u8, u8, u8, u8, u8, u8, u8) {
    let arr = value.to_le_bytes();
    (arr[0], arr[1], arr[2], arr[3], arr[4], arr[5], arr[6], arr[7])
}

fn to_bytes_32(value: u32) -> (u8, u8, u8, u8) {
    let arr = value.to_le_bytes();
    (arr[0], arr[1], arr[2], arr[3])
}

fn to_bytes_16(value: u16) -> (u8, u8) {
    let arr = value.to_le_bytes();

    (arr[0], arr[1])
}


/// The struct ASMCall generates the machine code
/// based on assembly instructions
///
/// `generated` - `Vec<u8>` of the generated machine code (**only the last instruction**)
#[derive(Clone)]
pub struct ASMCall {
    pub generated: Vec<u8>,
}

impl ASMCall {
    /// Creates new instance of ASMCall
    pub fn new() -> Self {
        Self {
            generated: vec![]
        }
    }

    /// Moves value into one of the 64bit register
    pub fn mov_64(&mut self, register: REGISTER, value: u64) {
        match register {
            REGISTER::RAX =>  {
                let (x1, x2, x3, x4, x5, x6, x7, x8) = to_bytes_64(value); 
                self.generated = vec![0x48, 0xa3, x1, x2, x3, x4, x5, x6, x7, x8];
            },
            REGISTER::RBX => {
                let (x1, x2, x3, x4, x5, x6, x7, x8) = to_bytes_64(value);
                self.generated = vec![0x48, 0xbb, x1, x2, x3, x4, x5, x6, x7, x8];
            },
            REGISTER::RCX => {
                let (x1, x2, x3, x4, x5, x6, x7, x8) = to_bytes_64(value);
                self.generated = vec![0x48, 0xb9, x1, x2, x3, x4, x5, x6, x7, x8];
            },
            REGISTER::RDX => {
                let (x1, x2, x3, x4, x5, x6, x7, x8) = to_bytes_64(value);
                self.generated = vec![0x48, 0xba, x1, x2, x3, x4, x5, x6, x7, x8];
            },
            _ => {}
        }
    }

    /// Moves value into one of the 32bit register
    pub fn mov_32(&mut self, register: REGISTER, value: u32) {
        match register {
            REGISTER::EAX =>  {
                let (x1, x2, x3, x4) = to_bytes_32(value); 
                self.generated = vec![0xb8, x1, x2, x3, x4];
            },
            REGISTER::EBX => {
                let (x1, x2, x3, x4) = to_bytes_32(value);
                self.generated = vec![0xbb, x1, x2, x3, x4];
            },
            REGISTER::ECX => {
                let (x1, x2, x3, x4) = to_bytes_32(value);
                self.generated = vec![0xb9, x1, x2, x3, x4];
            },
            REGISTER::EDX => {
                let (x1, x2, x3, x4) = to_bytes_32(value);
                self.generated = vec![0xba, x1, x2, x3, x4];
            },
            _ => {}
        }
    }

    /// Moves value into one of the 16bit register
    pub fn mov_16(&mut self, register: REGISTER, value: u16) {
        match register {
            REGISTER::AX =>{
                let (x1, x2) = to_bytes_16(value);
                self.generated = vec![0x66, 0xb8, x1, x2];
            },
            REGISTER::BX =>{
                let (x1, x2) = to_bytes_16(value);
                self.generated = vec![0x66, 0xbb, x1, x2];
            },
            REGISTER::DX =>{
                let (x1, x2) = to_bytes_16(value);
                self.generated = vec![0x66, 0xba, x1, x2];
            },
            _ => {}
        }
    }

    /// Moves value into one of the 8bit register
    pub fn mov_8(&mut self, register: REGISTER, value: u8) {
        match register {
            REGISTER::AH => { self.generated = vec![0xb4, value]; },
            REGISTER::AL => { self.generated = vec![0xb0, value]; },
            REGISTER::BH => { self.generated = vec![0xb7, value]; },
            REGISTER::BL => { self.generated = vec![0xb3, value]; },
            REGISTER::CH => { self.generated = vec![0xb5, value]; },
            REGISTER::CL => { self.generated = vec![0xb1, value]; },
            REGISTER::DH => { self.generated = vec![0xb6, value]; },
            REGISTER::DL => { self.generated = vec![0xb2, value]; },
            _ => {},
        }
    }

    /// Moves value from one register into another
    pub fn mov_reg(&mut self, target: REGISTER, source: REGISTER) {
        match target {
            REGISTER::EAX => {
                let gen = match source {
                    REGISTER::EAX => vec![0x89, 0xC0],
                    REGISTER::EBX => vec![0x89, 0xD8],
                    REGISTER::ECX => vec![0x89, 0xC8],
                    REGISTER::EDX => vec![0x89, 0xD0],
                    _ => vec![0x00],
                };
                self.generated = gen;
            },
            REGISTER::EBX => {
                let gen = match source {
                    REGISTER::EAX => vec![0x89, 0xC3],
                    REGISTER::EBX => vec![0x89, 0xDB],
                    REGISTER::ECX => vec![0x89, 0xCB],
                    REGISTER::EDX => vec![0x89, 0xD3],
                    _ => vec![0x00],
                };
                self.generated = gen;
            },
            REGISTER::ECX => {
                let gen = match source {
                    REGISTER::EAX => vec![0x89, 0xC1],
                    REGISTER::EBX => vec![0x89, 0xD9],
                    REGISTER::ECX => vec![0x89, 0xC9],
                    REGISTER::EDX => vec![0x89, 0xD1],
                    _ => vec![0x00],
                };
                self.generated = gen;
            },
            REGISTER::EDX => {
                let gen = match source {
                    REGISTER::EAX => vec![0x89, 0xC2],
                    REGISTER::EBX => vec![0x89, 0xDA],
                    REGISTER::ECX => vec![0x89, 0xCA],
                    REGISTER::EDX => vec![0x89, 0xD2],
                    _ => vec![0x00],
                };
                self.generated = gen;
            },
            REGISTER::AX => {
                let gen = match source {
                    REGISTER::AX => vec![0x66, 0x89, 0xc0],
                    REGISTER::BX => vec![0x66, 0x89, 0xD8],
                    REGISTER::CX => vec![0x66, 0x89, 0xC8],
                    REGISTER::DX => vec![0x66, 0x89, 0xD0],
                    _ => vec![0x00]
                };
                self.generated = gen;
            },
            REGISTER::BX => {
                let gen = match source {
                    REGISTER::AX => vec![0x66, 0x89, 0xc3],
                    REGISTER::BX => vec![0x66, 0x89, 0xDB],
                    REGISTER::CX => vec![0x66, 0x89, 0xCB],
                    REGISTER::DX => vec![0x66, 0x89, 0xD3],
                    _ => vec![0x00]
                };
                self.generated = gen;
            },
            REGISTER::CX => {
                let gen = match source {
                    REGISTER::AX => vec![0x66, 0x89, 0xc1],
                    REGISTER::BX => vec![0x66, 0x89, 0xD9],
                    REGISTER::CX => vec![0x66, 0x89, 0xC9],
                    REGISTER::DX => vec![0x66, 0x89, 0xD1],
                    _ => vec![0x00]
                };
                self.generated = gen;
            },
            REGISTER::DX => {
                let gen = match source {
                    REGISTER::AX => vec![0x66, 0x89, 0xC2],
                    REGISTER::BX => vec![0x66, 0x89, 0xDA],
                    REGISTER::CX => vec![0x66, 0x89, 0xCA],
                    REGISTER::DX => vec![0x66, 0x89, 0xD2],
                    _ => vec![0x00]
                };
                self.generated = gen;
            },
            REGISTER::AH => {
                let gen = match source {
                    REGISTER::AH => vec![0x88, 0xE4],
                    REGISTER::AL => vec![0x88, 0xC4],
                    REGISTER::BH => vec![0x88, 0xFC],
                    REGISTER::BL => vec![0x88, 0xDC],
                    REGISTER::CH => vec![0x88, 0xEC],
                    REGISTER::CL => vec![0x88, 0xCC],
                    REGISTER::DH => vec![0x88, 0xF4],
                    REGISTER::DL => vec![0x88, 0xD4],
                    _ => vec![0x00],
                };
                self.generated = gen;
            },
            REGISTER::AL => {
                let gen = match source {
                    REGISTER::AH => vec![0x88, 0xE0],
                    REGISTER::AL => vec![0x88, 0xC0],
                    REGISTER::BH => vec![0x88, 0xF8],
                    REGISTER::BL => vec![0x88, 0xD8],
                    REGISTER::CH => vec![0x88, 0xE8],
                    REGISTER::CL => vec![0x88, 0xC8],
                    REGISTER::DH => vec![0x88, 0xF0],
                    REGISTER::DL => vec![0x88, 0xD0],
                    _ => vec![0x00],
                };
                self.generated = gen;
            },
            REGISTER::BH => {
                let gen = match source {
                    REGISTER::AH => vec![0x88, 0xEF],
                    REGISTER::AL => vec![0x88, 0xCF],
                    REGISTER::BH => vec![0x88, 0xFF],
                    REGISTER::BL => vec![0x88, 0xDF],
                    REGISTER::CH => vec![0x88, 0xEF],
                    REGISTER::CL => vec![0x88, 0xCF],
                    REGISTER::DH => vec![0x88, 0xFF],
                    REGISTER::DL => vec![0x88, 0xDF],
                    _ => vec![0x00],
                };
                self.generated = gen;
            },
            REGISTER::BL => {
                let gen = match source {
                    REGISTER::AH => vec![0x88, 0xE3],
                    REGISTER::AL => vec![0x88, 0xC3],
                    REGISTER::BH => vec![0x88, 0xFB],
                    REGISTER::BL => vec![0x88, 0xDB],
                    REGISTER::CH => vec![0x88, 0xEB],
                    REGISTER::CL => vec![0x88, 0xCB],
                    REGISTER::DH => vec![0x88, 0xF3],
                    REGISTER::DL => vec![0x88, 0xD3],
                    _ => vec![0x00],
                };
                self.generated = gen;
            },
            REGISTER::CH => {
                let gen = match source {
                    REGISTER::AH => vec![0x88, 0xE5],
                    REGISTER::AL => vec![0x88, 0xC5],
                    REGISTER::BH => vec![0x88, 0xFD],
                    REGISTER::BL => vec![0x88, 0xDD],
                    REGISTER::CH => vec![0x88, 0xED],
                    REGISTER::CL => vec![0x88, 0xCD],
                    REGISTER::DH => vec![0x88, 0xF5],
                    REGISTER::DL => vec![0x88, 0xD5],
                    _ => vec![0x00],
                };
                self.generated = gen;
            },
            REGISTER::CL => {
                let gen = match source {
                    REGISTER::AH => vec![0x88, 0xE1],
                    REGISTER::AL => vec![0x88, 0xC1],
                    REGISTER::BH => vec![0x88, 0xF9],
                    REGISTER::BL => vec![0x88, 0xD9],
                    REGISTER::CH => vec![0x88, 0xE9],
                    REGISTER::CL => vec![0x88, 0xC9],
                    REGISTER::DH => vec![0x88, 0xF1],
                    REGISTER::DL => vec![0x88, 0xD1],
                    _ => vec![0x00],
                };
                self.generated = gen;
            },
            REGISTER::DH => {
                let gen = match source {
                    REGISTER::AH => vec![0x88, 0xE6],
                    REGISTER::AL => vec![0x88, 0xC6],
                    REGISTER::BH => vec![0x88, 0xFE],
                    REGISTER::BL => vec![0x88, 0xDE],
                    REGISTER::CH => vec![0x88, 0xEE],
                    REGISTER::CL => vec![0x88, 0xCE],
                    REGISTER::DH => vec![0x88, 0xF6],
                    REGISTER::DL => vec![0x88, 0xD6],
                    _ => vec![0x00],
                };
                self.generated = gen;
            },
            REGISTER::DL => {
                let gen = match source {
                    REGISTER::AH => vec![0x88, 0xE2],
                    REGISTER::AL => vec![0x88, 0xC2],
                    REGISTER::BH => vec![0x88, 0xFA],
                    REGISTER::BL => vec![0x88, 0xDA],
                    REGISTER::CH => vec![0x88, 0xEA],
                    REGISTER::CL => vec![0x88, 0xCA],
                    REGISTER::DH => vec![0x88, 0xF2],
                    REGISTER::DL => vec![0x88, 0xD2],
                    _ => vec![0x00],
                };
                self.generated = gen;
            },
            REGISTER::RAX => {
                let gen = match source {
                    REGISTER::RAX => vec![0x48, 0x89, 0xC0],
                    REGISTER::RBP => vec![0x48, 0x89, 0xE8],
                    REGISTER::RBX => vec![0x48, 0x89, 0xD8],
                    REGISTER::RCX => vec![0x48, 0x89, 0xC8],
                    REGISTER::RDI => vec![0x48, 0x89, 0xF8],
                    REGISTER::RDX => vec![0x48, 0x89, 0xD0],
                    REGISTER::RSI => vec![0x48, 0x89, 0xF0],
                    REGISTER::RSP => vec![0x48, 0x89, 0xE0],
                    _ => vec![0x00],
                };
                self.generated = gen;
            }
            REGISTER::RBX => {
                let gen = match source {
                    REGISTER::RAX => vec![0x48, 0x89, 0xC3],
                    REGISTER::RBP => vec![0x48, 0x89, 0xEB],
                    REGISTER::RBX => vec![0x48, 0x89, 0xDB],
                    REGISTER::RCX => vec![0x48, 0x89, 0xCB],
                    REGISTER::RDI => vec![0x48, 0x89, 0xFB],
                    REGISTER::RDX => vec![0x48, 0x89, 0xD3],
                    REGISTER::RSI => vec![0x48, 0x89, 0xF3],
                    REGISTER::RSP => vec![0x48, 0x89, 0xE3],
                    _ => vec![0x00],
                };
                self.generated = gen;
            }
            REGISTER::RCX => {
                let gen = match source {
                    REGISTER::RAX => vec![0x48, 0x89, 0xC1],
                    REGISTER::RBP => vec![0x48, 0x89, 0xE9],
                    REGISTER::RBX => vec![0x48, 0x89, 0xD9],
                    REGISTER::RCX => vec![0x48, 0x89, 0xC9],
                    REGISTER::RDI => vec![0x48, 0x89, 0xF9],
                    REGISTER::RDX => vec![0x48, 0x89, 0xD1],
                    REGISTER::RSI => vec![0x48, 0x89, 0xF1],
                    REGISTER::RSP => vec![0x48, 0x89, 0xE1],
                    _ => vec![0x00],
                };
                self.generated = gen;
            }
            REGISTER::RDX => {
                let gen = match source {
                    REGISTER::RAX => vec![0x48, 0x89, 0xC2],
                    REGISTER::RBP => vec![0x48, 0x89, 0xEA],
                    REGISTER::RBX => vec![0x48, 0x89, 0xDA],
                    REGISTER::RCX => vec![0x48, 0x89, 0xCA],
                    REGISTER::RDI => vec![0x48, 0x89, 0xFA],
                    REGISTER::RDX => vec![0x48, 0x89, 0xD2],
                    REGISTER::RSI => vec![0x48, 0x89, 0xF2],
                    REGISTER::RSP => vec![0x48, 0x89, 0xE2],
                    _ => vec![0x00],
                };
                self.generated = gen;
            }
            REGISTER::RSI => {
                let gen = match source {
                    REGISTER::RAX => vec![0x48, 0x89, 0xC6],
                    REGISTER::RBP => vec![0x48, 0x89, 0xEE],
                    REGISTER::RBX => vec![0x48, 0x89, 0xDE],
                    REGISTER::RCX => vec![0x48, 0x89, 0xCE],
                    REGISTER::RDI => vec![0x48, 0x89, 0xFE],
                    REGISTER::RDX => vec![0x48, 0x89, 0xD6],
                    REGISTER::RSI => vec![0x48, 0x89, 0xF6],
                    REGISTER::RSP => vec![0x48, 0x89, 0xE6],
                    _ => vec![0x00],
                };
                self.generated = gen;
            }
            REGISTER::RDI => {
                let gen = match source {
                    REGISTER::RAX => vec![0x48, 0x89, 0xC7],
                    REGISTER::RBP => vec![0x48, 0x89, 0xEF],
                    REGISTER::RBX => vec![0x48, 0x89, 0xDF],
                    REGISTER::RCX => vec![0x48, 0x89, 0xCF],
                    REGISTER::RDI => vec![0x48, 0x89, 0xFF],
                    REGISTER::RDX => vec![0x48, 0x89, 0xD7],
                    REGISTER::RSI => vec![0x48, 0x89, 0xF7],
                    REGISTER::RSP => vec![0x48, 0x89, 0xE7],
                    _ => vec![0x00],
                };
                self.generated = gen;
            }
            REGISTER::RBP => {
                let gen = match source {
                    REGISTER::RAX => vec![0x48, 0x89, 0xC5],
                    REGISTER::RBP => vec![0x48, 0x89, 0xED],
                    REGISTER::RBX => vec![0x48, 0x89, 0xDD],
                    REGISTER::RCX => vec![0x48, 0x89, 0xCD],
                    REGISTER::RDI => vec![0x48, 0x89, 0xFD],
                    REGISTER::RDX => vec![0x48, 0x89, 0xD5],
                    REGISTER::RSI => vec![0x48, 0x89, 0xF5],
                    REGISTER::RSP => vec![0x48, 0x89, 0xE5],
                    _ => vec![0x00],
                };
                self.generated = gen;
            }
            REGISTER::RSP => {
                let gen = match source {
                    REGISTER::RAX => vec![0x48, 0x89, 0xC4],
                    REGISTER::RBP => vec![0x48, 0x89, 0xEC],
                    REGISTER::RBX => vec![0x48, 0x89, 0xDC],
                    REGISTER::RCX => vec![0x48, 0x89, 0xCC],
                    REGISTER::RDI => vec![0x48, 0x89, 0xFC],
                    REGISTER::RDX => vec![0x48, 0x89, 0xD4],
                    REGISTER::RSI => vec![0x48, 0x89, 0xF4],
                    REGISTER::RSP => vec![0x48, 0x89, 0xE4],
                    _ => vec![0x00],
                };
                self.generated = gen;
            }
            _ => {}
        }
    }

    /// Moves the value from the register to specified memory adress
    pub fn to_memory(&mut self, adress: u64, target: REGISTER) {
        match target {
            REGISTER::EAX => {
                let (x1, x2, x3, x4) = to_bytes_32(adress as u32);
                self.generated = vec![0xA3, x1, x2, x3, x4];
            },
            REGISTER::EBX => {
                let (x1, x2, x3, x4) = to_bytes_32(adress as u32);
                self.generated = vec![0x89, 0x1D, x1, x2, x3, x4];
            },
            REGISTER::ECX => {
                let (x1, x2, x3, x4) = to_bytes_32(adress as u32);
                self.generated = vec![0x89, 0x0D, x1, x2, x3, x4];
            },
            REGISTER::EDX => {
                let (x1, x2, x3, x4) = to_bytes_32(adress as u32);
                self.generated = vec![0x89, 0x15, x1, x2, x3, x4];
            },
            REGISTER::AX => {
                let (x1, x2) = to_bytes_16(adress as u16);
                self.generated = vec![0x66, 0xa3, x1, x2];
            },
            REGISTER::BX => {
                let (x1, x2) = to_bytes_16(adress as u16);
                self.generated = vec![0x66, 0x89, 0x1D, x1, x2];
            },
            REGISTER::CX => {
                let (x1, x2) = to_bytes_16(adress as u16);
                self.generated = vec![0x66, 0x89, 0x0D, x1, x2];
            },
            REGISTER::DX => {
                let (x1, x2) = to_bytes_16(adress as u16);
                self.generated = vec![0x66, 0x89, 0x15, x1, x2];
            },
            REGISTER::AH => {
                self.generated = vec![0x88, 0x25,   adress as u8];
            },
            REGISTER::AL => {
                self.generated = vec![0xa2,         adress as u8];
            },
            REGISTER::BH => {
                self.generated = vec![0x88, 0x3d,   adress as u8];
            },
            REGISTER::BL => {
                self.generated = vec![0x88, 0x1D,   adress as u8];
            },
            REGISTER::CH => {
                self.generated = vec![0x88, 0x2D,   adress as u8];
            },
            REGISTER::CL => {
                self.generated = vec![0x88, 0x0D,   adress as u8];
            },
            REGISTER::DH => {
                self.generated = vec![0x88, 0x35,   adress as u8];
            },
            REGISTER::DL => {
                self.generated = vec![0x88, 0x15,   adress as u8];
            },
            _ => {},
        }
    }

    /// Moves the value from the sepcified memory adress into the target register
    pub fn from_memory(&mut self, adress: u32, target: REGISTER) {
        match target {
            REGISTER::EAX => {
                let (x1, x2, x3, x4) = to_bytes_32(adress);
                self.generated = vec![0xa1, x1, x2, x3, x4];
            },
            REGISTER::EBX => {
                let (x1, x2, x3, x4) = to_bytes_32(adress);
                self.generated = vec![0x8b, 0x1D, x1, x2, x3, x4];
            },
            REGISTER::ECX => {
                let (x1, x2, x3, x4) = to_bytes_32(adress);
                self.generated = vec![0x8B, 0x0D, x1, x2, x3, x4];
            },
            REGISTER::EDX => {
                let (x1, x2, x3, x4) = to_bytes_32(adress);
                self.generated = vec![0x8B, 0x15, x1, x2, x3, x4];
            },
            REGISTER::AX => {
                let (x1, x2, x3, x4) = to_bytes_32(adress);
                self.generated = vec![0x66, 0xa1, x1, x2, x3, x4];
            },
            REGISTER::BX => {
                let (x1, x2, x3, x4) = to_bytes_32(adress);
                self.generated = vec![0x66, 0x8b, 0x1D, x1, x2, x3, x4];
            },
            REGISTER::CX => {
                let (x1, x2, x3, x4) = to_bytes_32(adress);
                self.generated = vec![0x66, 0x8B, 0x0D, x1, x2, x3, x4];
            },
            REGISTER::DX => {
                let (x1, x2, x3, x4) = to_bytes_32(adress);
                self.generated = vec![0x66, 0x8b, 0x15, x1, x2, x3, x4];
            },
            REGISTER::AH => {
                let (x1, x2, x3, x4) = to_bytes_32(adress);
                self.generated = vec![0x8a, 0x25, x1, x2, x3, x4];
            },
            REGISTER::AL => {
                let (x1, x2, x3, x4) = to_bytes_32(adress);
                self.generated = vec![0xa0, x1, x2, x3, x4];
            },
            REGISTER::BH => {
                let (x1, x2, x3, x4) = to_bytes_32(adress);
                self.generated = vec![0x8a, 0x3d, x1, x2, x3, x4];
            },
            REGISTER::BL => {
                let (x1, x2, x3, x4) = to_bytes_32(adress);
                self.generated = vec![0x8A, 0x1D, x1, x2, x3, x4];
            },
            REGISTER::CH => {
                let (x1, x2, x3, x4) = to_bytes_32(adress);
                self.generated = vec![0x8a, 0x2D, x1, x2, x3, x4];
            },
            REGISTER::CL => {
                let (x1, x2, x3, x4) = to_bytes_32(adress);
                self.generated = vec![0x8a, 0x0D, x1, x2, x3, x4];
            },
            REGISTER::DH => {
                let (x1, x2, x3, x4) = to_bytes_32(adress);
                self.generated = vec![0x8a, 0x35, x1, x2, x3, x4];
            },
            REGISTER::DL => {
                let (x1, x2, x3, x4) = to_bytes_32(adress);
                self.generated = vec![0x8a, 0x15, x1, x2, x3, x4];
            },
            _ => {}
        }
    }

    /// Pushes the register onto the stack
    pub fn push(&mut self, reg: REGISTER) {
        match reg {
            REGISTER::AX  =>    {   self.generated = vec![0x66, 0x50];  }
            REGISTER::CX  =>    {   self.generated = vec![0x66, 0x51];  }
            REGISTER::DX  =>    {   self.generated = vec![0x66, 0x52];  }
            REGISTER::BX  =>    {   self.generated = vec![0x66, 0x53];  }
            REGISTER::RAX =>    {   self.generated = vec![0x50];        }
            REGISTER::RBX =>    {   self.generated = vec![0x53];        }
            REGISTER::RCX =>    {   self.generated = vec![0x51];        }
            REGISTER::RDX =>    {   self.generated = vec![0x52];        }
            REGISTER::RSI =>    {   self.generated = vec![0x56];        }
            REGISTER::RDI =>    {   self.generated = vec![0x57];        }
            REGISTER::RBP =>    {   self.generated = vec![0x55];        }
            REGISTER::RSP =>    {   self.generated = vec![0x54];        }
            _ => {}
        }
    }

    /// Pops the register from the stack
    pub fn pop(&mut self, reg: REGISTER) {
        match reg {
            REGISTER::AX =>     {   self.generated = vec![0x66, 0x58]; }
            REGISTER::CX =>     {   self.generated = vec![0x66, 0x59]; }
            REGISTER::DX =>     {   self.generated = vec![0x66, 0x5a]; }
            REGISTER::BX =>     {   self.generated = vec![0x66, 0x5b]; }
            REGISTER::RAX =>    {   self.generated = vec![0x58];        }
            REGISTER::RBX =>    {   self.generated = vec![0x59];        }
            REGISTER::RCX =>    {   self.generated = vec![0x5b];        }
            REGISTER::RDX =>    {   self.generated = vec![0x5a];        }
            REGISTER::RSI =>    {   self.generated = vec![0x5e];        }
            REGISTER::RDI =>    {   self.generated = vec![0x5f];        }
            REGISTER::RBP =>    {   self.generated = vec![0x5d];        }
            REGISTER::RSP =>    {   self.generated = vec![0x5c];        }
            _ => {},
        }
    }

    /// Does nothing
    pub fn nop(&mut self) {
        self.generated = vec![0x90];
    }

    /// Jumps to the specifed adress
    pub fn jmp(&mut self, adress: u32) {
        let (x1, x2, x3, x4) = to_bytes_32(adress);
        self.generated = vec![0xe9, x1, x2, x3, x4];
    }

    /// Calls the specified adress
    pub fn call(&mut self, adress: u32) {
        let (x1, x2, x3, x4) = to_bytes_32(adress);
        self.generated = vec![0xe8, x1, x2, x3, x4];
    }

    /// Calls the adress which is stored into the register
    pub fn call_reg(&mut self, target: REGISTER) {
        match target {
            REGISTER::RAX => { self.generated = vec![0xFF, 0xD0] }
            REGISTER::RBX => { self.generated = vec![0xFF, 0xD3] }
            REGISTER::RCX => { self.generated = vec![0xFF, 0xD1] }
            REGISTER::RDX => { self.generated = vec![0xFF, 0xD2] }
            REGISTER::RSI => { self.generated = vec![0xFF, 0xD6] }
            REGISTER::RDI => { self.generated = vec![0xFF, 0xD7] }
            REGISTER::RBP => { self.generated = vec![0xFF, 0xD5] }
            REGISTER::RSP => { self.generated = vec![0xFF, 0xD4] }
            _ => {},
        }
    }

    /// Jumps to the adress which is stored into the register
    pub fn jmp_reg(&mut self, target: REGISTER) {
        match target {
            REGISTER::RAX => { self.generated = vec![0xFF, 0xE0] }
            REGISTER::RBX => { self.generated = vec![0xFF, 0xE3] }
            REGISTER::RCX => { self.generated = vec![0xFF, 0xE1] }
            REGISTER::RDX => { self.generated = vec![0xFF, 0xE2] }
            REGISTER::RSI => { self.generated = vec![0xFF, 0xE6] }
            REGISTER::RDI => { self.generated = vec![0xFF, 0xE7] }
            REGISTER::RBP => { self.generated = vec![0xFF, 0xE5] }
            REGISTER::RSP => { self.generated = vec![0xFF, 0xE4] }
            _ => {},
        }
    }

    /// Return
    pub fn ret(&mut self) {
        self.generated = vec![0xC3];
    }

    /// Calls an interupt
    pub fn int(&mut self, nr: u8) {
        self.generated = vec![0xCD, nr];
    }

    /// Just endbr64
    pub fn endbr64(&mut self) {
        self.generated = vec![0xF3, 0x0F, 0x1E, 0xFA];
    }

    /// deactivates interrupts
    pub fn cli(&mut self) {
         self.generated = vec![0xFA];
    }

    /// activates interrupts
    pub fn sti(&mut self) {
         self.generated = vec![0xFB];
    }

    /// Clear carry flag
    pub fn clc(&mut self) {
         self.generated = vec![0xF8];
    }

    /// Clear direction flag
    pub fn cld(&mut self) {
        self.generated = vec![0xFC];
   }

    /// ASCII adjust AL after addition
    pub fn aaa(&mut self) {
        self.generated = vec![0x37];
   }

    /// Inecrements the register by 1
    pub fn inc(&mut self, register: REGISTER) {
        match register {
            REGISTER::RAX =>    { self.generated = vec![0x48, 0xFF, 0xC0]; }, 
            REGISTER::RBX =>    { self.generated = vec![0x48, 0xFF, 0xC3]; }, 
            REGISTER::RCX =>    { self.generated = vec![0x48, 0xFF, 0xC1]; }, 
            REGISTER::RDX =>    { self.generated = vec![0x48, 0xFF, 0xC2]; },
            REGISTER::EAX =>    { self.generated = vec![0xFF, 0xC0]; }, 
            REGISTER::EBX =>    { self.generated = vec![0xFF, 0xC3]; }, 
            REGISTER::ECX =>    { self.generated = vec![0xFF, 0xC1]; }, 
            REGISTER::EDX =>    { self.generated = vec![0xFF, 0xC2]; }, 
            REGISTER:::AX =>    { self.generated = vec![0x66, 0xFF, 0xC0]},
            REGISTER:::BX =>    { self.generated = vec![0x66, 0xFF, 0xC3]},
            REGISTER:::CX =>    { self.generated = vec![0x66, 0xFF, 0xC1]},
            REGISTER:::DX =>    { self.generated = vec![0x66, 0xFF, 0xC2]},
            REGISTER::AH =>     { self.generated = vec![0xFE, 0xC4]}, 
            REGISTER::AL =>     { self.generated = vec![0xFE, 0xC0]}, 
            REGISTER::BH =>     { self.generated = vec![0xFE, 0xC7]}, 
            REGISTER::BL =>     { self.generated = vec![0xFE, 0xC3]}, 
            REGISTER::CH =>     { self.generated = vec![0xFE, 0xC5]}, 
            REGISTER::CL =>     { self.generated = vec![0xFE, 0xC1]}, 
            REGISTER::DH =>     { self.generated = vec![0xFE, 0xC6]}, 
            REGISTER::DL =>     { self.generated = vec![0xFE, 0xC2]},
            REGISTER::RSI =>    { self.generated = vec![0x48, 0xFF, 0xC6]}, 
            REGISTER::RDI =>    { self.generated = vec![0x48, 0xFF, 0xC7]}, 
            REGISTER::RBP =>    { self.generated = vec![0x48, 0xFF, 0xC5]},
            REGISTER::RSP =>    { self.generated = vec![0x48, 0xFF, 0xC5]},
            _ => {}
        }
    }

    /// Decrements the register by 1
    pub fn dec(&mut self, register: REGISTER) {
        match register {
            REGISTER::RAX =>    { self.generated = vec![0x48, 0xFF, 0xC8]; }, 
            REGISTER::RBX =>    { self.generated = vec![0x48, 0xFF, 0xCB]; }, 
            REGISTER::RCX =>    { self.generated = vec![0x48, 0xFF, 0xC9]; }, 
            REGISTER::RDX =>    { self.generated = vec![0x48, 0xFF, 0xCA]; },
            REGISTER::EAX =>    { self.generated = vec![0xFF, 0xC8]; }, 
            REGISTER::EBX =>    { self.generated = vec![0xFF, 0xCB]; }, 
            REGISTER::ECX =>    { self.generated = vec![0xFF, 0xC9]; }, 
            REGISTER::EDX =>    { self.generated = vec![0xFF, 0xCA]; }, 
            REGISTER:::AX =>    { self.generated = vec![0x66, 0xFF, 0xC8]},
            REGISTER:::BX =>    { self.generated = vec![0x66, 0xFF, 0xCB]},
            REGISTER:::CX =>    { self.generated = vec![0x66, 0xFF, 0xC9]},
            REGISTER:::DX =>    { self.generated = vec![0x66, 0xFF, 0xCA]},
            REGISTER::AH =>     { self.generated = vec![0xFE, 0xCC]}, 
            REGISTER::AL =>     { self.generated = vec![0xFE, 0xC8]}, 
            REGISTER::BH =>     { self.generated = vec![0xFE, 0xCF]}, 
            REGISTER::BL =>     { self.generated = vec![0xFE, 0xCB]}, 
            REGISTER::CH =>     { self.generated = vec![0xFE, 0xCD]}, 
            REGISTER::CL =>     { self.generated = vec![0xFE, 0xC9]}, 
            REGISTER::DH =>     { self.generated = vec![0xFE, 0xCE]}, 
            REGISTER::DL =>     { self.generated = vec![0xFE, 0xCA]},
            REGISTER::RSI =>    { self.generated = vec![0x48, 0xFF, 0xCE]}, 
            REGISTER::RDI =>    { self.generated = vec![0x48, 0xFF, 0xCF]}, 
            REGISTER::RBP =>    { self.generated = vec![0x48, 0xFF, 0xCD]},
            REGISTER::RSP =>    { self.generated = vec![0x48, 0xFF, 0xCC]},
            _ => {}
        }
    }

    /// ASCII adjust AX before division
    pub fn aad(&mut self) {
        self.generated = vec![0xD5];
    }

    /// ASCII adjust AX after multiplication
    pub fn aam(&mut self) {
        self.generated = vec![0xD4]
    } 

    /// ASCII adjust AL after subtraction
    pub fn aas(&mut self) {
        self.generated = vec![0x3F]
    }
    
    /// Add with carry value to 64Bit register
    pub fn adc_64(&mut self, register: REGISTER, value: u64) {
        if value > u32::MAX {
            match register {
                REGISTER::RAX => { 
                    let (x1, x2, x3, x4) = to_bytes_32((value - 0xffffffff) as u32);
                    self.generated = vec![0x48, 0x11, 0x14, 0x25, x1, x2, x3, x4];
                },
                REGISTER::RBX => { 
                    let (x1, x2, x3, x4) = to_bytes_32((value - 0xffffffff) as u32);
                    self.generated = vec![0x48, 0x11, 0x1C, 0x25, x1, x2, x3, x4];
                },
                REGISTER::RCX => { 
                    let (x1, x2, x3, x4) = to_bytes_32((value - 0xffffffff) as u32);
                    self.generated = vec![0x48, 0x11, 0x0C, 0x25, x1, x2, x3, x4];
                },
                REGISTER::RDX => { 
                    let (x1, x2, x3, x4) = to_bytes_32((value - 0xffffffff) as u32);
                    self.generated = vec![0x48, 0x11, 0x14, 0x25, x1, x2, x3, x4];
                },
                REGISTER::RSI => { 
                    let (x1, x2, x3, x4) = to_bytes_32((value - 0xffffffff) as u32);
                    self.generated = vec![0x48, 0x81, 0xD6, x1, x2, x3, x4];
                },
                REGISTER::RDI => { 
                    let (x1, x2, x3, x4) = to_bytes_32((value - 0xffffffff) as u32);
                    self.generated = vec![0x48, 0x11, 0x34, 0x25, x1, x2, x3, x4];
                },
                REGISTER::RBP => { 
                    let (x1, x2, x3, x4) = to_bytes_32((value - 0xffffffff) as u32);
                    self.generated = vec![0x48, 0x11, 0x2C, 0x25, x1, x2, x3, x4];
                },
                REGISTER::RSP => { 
                    let (x1, x2, x3, x4) = to_bytes_32((value - 0xffffffff) as u32);
                    self.generated = vec![0x48, 0x11, 0x24, 0x25, x1, x2, x3, x4];
                },
                _ => {},
            }
        } else {
            match register {
                REGISTER::RAX => { 
                    let (x1, x2, x3, x4) = to_bytes_32(value as u32);
                    self.generated = vec![0x48, 0x11, 0x05, x1, x2, x3, x4];
                },
                REGISTER::RBX => { 
                    let (x1, x2, x3, x4) = to_bytes_32(value as u32);
                    self.generated = vec![0x48, 0x11, 0x1D, x1, x2, x3, x4];
                },
                REGISTER::RCX => { 
                    let (x1, x2, x3, x4) = to_bytes_32(value as u32);
                    self.generated = vec![0x48, 0x11, 0x0D, x1, x2, x3, x4];
                },
                REGISTER::RDX => { 
                    let (x1, x2, x3, x4) = to_bytes_32(value as u32);
                    self.generated = vec![0x48, 0x11, 0x15, x1, x2, x3, x4];
                },
                REGISTER::RSI => { 
                    let (x1, x2, x3, x4) = to_bytes_32(value as u32);
                    self.generated = vec![0x48, 0x11, 0x35, x1, x2, x3, x4];
                },
                REGISTER::RDI => { 
                    let (x1, x2, x3, x4) = to_bytes_32(value as u32);
                    self.generated = vec![0x48, 0x11, 0x3D, x1, x2, x3, x4];
                },
                REGISTER::RBP => { 
                    let (x1, x2, x3, x4) = to_bytes_32(value as u32);
                    self.generated = vec![0x48, 0x11, 0x2D, x1, x2, x3, x4];
                },
                REGISTER::RSP => { 
                    let (x1, x2, x3, x4) = to_bytes_32(value as u32);
                    self.generated = vec![0x48, 0x11, 0x25, x1, x2, x3, x4];
                },
                _ => {},
            }
        }
    }

    /// Add with carry value to 32Bit register
    pub fn adc_32(&mut self, register: REGISTER, value: u32) {
        match register {
            REGISTER::EAX => { 
                let (x1, x2, x3, x4) = to_bytes_32(value);
                self.generated = vec![0x15, x1, x2, x3, x4];
            },
            REGISTER::EBX => { 
                let (x1, x2, x3, x4) = to_bytes_32(value);
                self.generated = vec![0x81, 0xD3, x1, x2, x3, x4];
            },
            REGISTER::ECX => { 
                let (x1, x2, x3, x4) = to_bytes_32(value);
                self.generated = vec![0x81, 0xD1, x1, x2, x3, x4];
            },
            REGISTER::EDX => { 
                let (x1, x2, x3, x4) = to_bytes_32(value);
                self.generated = vec![0x81, 0xD2, x1, x2, x3, x4];
            },
            _ => {},
        }
    }

    /// Add with carry value to 16Bit register
    pub fn adc_16(&mut self, register: REGISTER, value: u16) {
        match register {
            REGISTER::AX => { 
                let (x1, x2) = to_bytes_16(value);
                self.generated = vec![0x66, 0x15, x1, x2];
            },
            REGISTER::BX => { 
                let (x1, x2) = to_bytes_16(value);
                self.generated = vec![0x66, 0x81, 0xD3, x1, x2];
            },
            REGISTER::CX => { 
                let (x1, x2) = to_bytes_16(value);
                self.generated = vec![0x66, 0x81, 0xD1, x1, x2];
            },
            REGISTER::DX => { 
                let (x1, x2) = to_bytes_16(value);
                self.generated = vec![0x66, 0x81, 0xD2, x1, x2];
            },
            _ => {},
        }
    }

    /// Add with carry value to 8Bit register
    pub fn adc_8(&mut self, register: REGISTER, value: u8) {
        match register {
            REGISTER::AH => { 
                self.generated = vec![0x80, 0xD4, value.to_le_bytes()];
            },
            REGISTER::AL => { self.generated = vec![0x14, 0xFF, value.to_le_bytes()]; },
            REGISTER::BH => { self.generated = vec![0x80, 0xD7, value.to_le_bytes()]; },
            REGISTER::BL => { self.generated = vec![0x80, 0xD3, value.to_le_bytes()]; },
            REGISTER::CH => { self.generated = vec![0x80, 0xD5, value.to_le_bytes()]; },
            REGISTER::CL => { self.generated = vec![0x80, 0xD1, value.to_le_bytes()]; },
            REGISTER::DH => { self.generated = vec![0x80, 0xD6, value.to_le_bytes()]; },
            REGISTER::DL => { self.generated = vec![0x80, 0xD2, value.to_le_bytes()]; },
            _ => {},
        }
    }

    /// Add with carry value to register `dest` to register `target`
    pub fn adc_reg(&mut self, dest: REGISTER, src: REGISTER) {
        match dest {
            REGISTER::RAX => { 
                let gen = match src {
                    REGISTER::RAX => vec![0x48, 0x11, 0xC0],
                    REGISTER::RBX => vec![0x48, 0x11, 0xC3],
                    REGISTER::RCX => vec![0x48, 0x11, 0xC1],
                    REGISTER::RDX => vec![0x48, 0x11, 0xC2],
                    REGISTER::RBP => vec![0x48, 0x11, 0xC5],
                    REGISTER::RSI => vec![0x48, 0x11, 0xC6],
                    REGISTER::RDI => vec![0x48, 0x11, 0xC7],
                    REGISTER::RSP => vec![0x48, 0x11, 0xC4],
                    _ => vec![0]
                };
                self.generated = gen;
            }, REGISTER::RBX => { 
                let gen = match src {
                    REGISTER::RAX => vec![0x48, 0x11, 0xD8],
                    REGISTER::RBX => vec![0x48, 0x11, 0xDB],
                    REGISTER::RCX => vec![0x48, 0x11, 0xD9],
                    REGISTER::RDX => vec![0x48, 0x11, 0xDA],
                    REGISTER::RBP => vec![0x48, 0x11, 0xDD],
                    REGISTER::RSI => vec![0x48, 0x11, 0xDE],
                    REGISTER::RDI => vec![0x48, 0x11, 0xDF],
                    REGISTER::RSP => vec![0x48, 0x11, 0xDC],
                    _ => vec![0]
                };
                self.generated = gen;
            }, REGISTER::RCX => { 
                let gen = match src {
                    REGISTER::RAX => vec![0x48, 0x11, 0xC8],
                    REGISTER::RBX => vec![0x48, 0x11, 0xCB],
                    REGISTER::RCX => vec![0x48, 0x11, 0xC9],
                    REGISTER::RDX => vec![0x48, 0x11, 0xCA],
                    REGISTER::RBP => vec![0x48, 0x11, 0xCD],
                    REGISTER::RSI => vec![0x48, 0x11, 0xCE],
                    REGISTER::RDI => vec![0x48, 0x11, 0xCF],
                    REGISTER::RSP => vec![0x48, 0x11, 0xCC],
                    _ => vec![0]
                };
                self.generated = gen;
            }, REGISTER::RDX => { 
                let gen = match src {
                    REGISTER::RAX => vec![0x48, 0x11, 0xD0],
                    REGISTER::RBX => vec![0x48, 0x11, 0xD3],
                    REGISTER::RCX => vec![0x48, 0x11, 0xD1],
                    REGISTER::RDX => vec![0x48, 0x11, 0xD2],
                    REGISTER::RBP => vec![0x48, 0x11, 0xD5],
                    REGISTER::RSI => vec![0x48, 0x11, 0xD6],
                    REGISTER::RDI => vec![0x48, 0x11, 0xD7],
                    REGISTER::RSP => vec![0x48, 0x11, 0xD4],
                    _ => vec![0]
                };
                self.generated = gen;
            }, REGISTER::RSI => { 
                let gen = match src {
                    REGISTER::RAX => vec![0x48, 0x11, 0xF0],
                    REGISTER::RBX => vec![0x48, 0x11, 0xF3],
                    REGISTER::RCX => vec![0x48, 0x11, 0xF1],
                    REGISTER::RDX => vec![0x48, 0x11, 0xF2],
                    REGISTER::RBP => vec![0x48, 0x11, 0xF5],
                    REGISTER::RSI => vec![0x48, 0x11, 0xF6],
                    REGISTER::RDI => vec![0x48, 0x11, 0xF7],
                    REGISTER::RSP => vec![0x48, 0x11, 0xF4],
                    _ => vec![0]
                };
                self.generated = gen;
            }, REGISTER::RDI => { 
                let gen = match src {
                    REGISTER::RAX => vec![0x48, 0x11, 0xF8],
                    REGISTER::RBX => vec![0x48, 0x11, 0xFB],
                    REGISTER::RCX => vec![0x48, 0x11, 0xF9],
                    REGISTER::RDX => vec![0x48, 0x11, 0xFA],
                    REGISTER::RBP => vec![0x48, 0x11, 0xFD],
                    REGISTER::RSI => vec![0x48, 0x11, 0xFE],
                    REGISTER::RDI => vec![0x48, 0x11, 0xFF],
                    REGISTER::RSP => vec![0x48, 0x11, 0xFC],
                    _ => vec![0]
                };
                self.generated = gen;
            }, REGISTER::RBP => { 
                let gen = match src {
                    REGISTER::RAX => vec![0x48, 0x11, 0xEA],
                    REGISTER::RBX => vec![0x48, 0x11, 0xEB],
                    REGISTER::RCX => vec![0x48, 0x11, 0xE9],
                    REGISTER::RDX => vec![0x48, 0x11, 0xEA],
                    REGISTER::RBP => vec![0x48, 0x11, 0xED],
                    REGISTER::RSI => vec![0x48, 0x11, 0xEE],
                    REGISTER::RDI => vec![0x48, 0x11, 0xEF],
                    REGISTER::RSP => vec![0x48, 0x11, 0xEC],
                    _ => vec![0]
                };
                self.generated = gen;
            }, REGISTER::RSP => { 
                let gen = match src {
                    REGISTER::RAX => vec![0x48, 0x11, 0xE0],
                    REGISTER::RBX => vec![0x48, 0x11, 0xE3],
                    REGISTER::RCX => vec![0x48, 0x11, 0xE1],
                    REGISTER::RDX => vec![0x48, 0x11, 0xE2],
                    REGISTER::RBP => vec![0x48, 0x11, 0xE5],
                    REGISTER::RSI => vec![0x48, 0x11, 0xE6],
                    REGISTER::RDI => vec![0x48, 0x11, 0xE7],
                    REGISTER::RSP => vec![0x48, 0x11, 0xE4],
                    _ => vec![0]
                };
                self.generated = gen;
            }, REGISTER::EAX => { 
                let gen = match src {
                    REGISTER::EAX => vec![0x11, 0xC0], 
                    REGISTER::EBX => vec![0x11, 0xC3], 
                    REGISTER::ECX => vec![0x11, 0xC1], 
                    REGISTER::EDX => vec![0x11, 0xC2],
                    _ => vec![0]
                };
                self.generated = gen;
            }, REGISTER::EBX => { 
                let gen = match src {
                    REGISTER::EAX => vec![0x11, 0xD8], 
                    REGISTER::EBX => vec![0x11, 0xDA], 
                    REGISTER::ECX => vec![0x11, 0xD9], 
                    REGISTER::EDX => vec![0x11, 0xDA],
                    _ => vec![0]
                };
                self.generated = gen;
            }, REGISTER::ECX => { 
                let gen = match src {
                    REGISTER::EAX => vec![0x11, 0xC8], 
                    REGISTER::EBX => vec![0x11, 0xCB], 
                    REGISTER::ECX => vec![0x11, 0xC9], 
                    REGISTER::EDX => vec![0x11, 0xCA],
                    _ => vec![0]
                };
                self.generated = gen;
            }, REGISTER::EDX => { 
                let gen = match src {
                    REGISTER::EAX => vec![0x11, 0xD0], 
                    REGISTER::EBX => vec![0x11, 0xD3], 
                    REGISTER::ECX => vec![0x11, 0xD1], 
                    REGISTER::EDX => vec![0x11, 0xD2],
                    _ => vec![0]
                };
                self.generated = gen;
            },
            REGISTER::AX => {
                let gen match src {
                    REGISTER::AX => vec![0x66, 0x11, 0xC0],
                    REGISTER::BX => vec![0x66, 0x11, 0xC3],
                    REGISTER::CX => vec![0x66, 0x11, 0xC1],
                    REGISTER::DX => vec![0x66, 0x11, 0xC2],
                    _ => vec![0]
                }
                self.generated = gen;
            }, REGISTER::BX => {
                let gen match src {
                    REGISTER::AX => vec![0x66, 0x11, 0xD8],
                    REGISTER::BX => vec![0x66, 0x11, 0xDB],
                    REGISTER::CX => vec![0x66, 0x11, 0xD9],
                    REGISTER::DX => vec![0x66, 0x11, 0xDA],
                    _ => vec![0]
                }
                self.generated = gen;
            }, REGISTER::CX => {
                let gen match src {
                    REGISTER::AX => vec![0x66, 0x11, 0xC8],
                    REGISTER::BX => vec![0x66, 0x11, 0xCB],
                    REGISTER::CX => vec![0x66, 0x11, 0xC9],
                    REGISTER::DX => vec![0x66, 0x11, 0xCA],
                    _ => vec![0]
                }    
                self.generated = gen;
            }, REGISTER::DX => {
                let gen match src {
                    REGISTER::AX => vec![0x66, 0x11, 0xD0],
                    REGISTER::BX => vec![0x66, 0x11, 0xD3],
                    REGISTER::CX => vec![0x66, 0x11, 0xD1],
                    REGISTER::DX => vec![0x66, 0x11, 0xD2],
                    _ => vec![0]
                }
                self.generated = gen;
            },
            REGISTER::AH => {
                let gen = match src {
                    REGISTER::AH => vec![0x10, 0xE4],
                    REGISTER::AL => vec![0x10, 0xE0],
                    REGISTER::BH => vec![0x10, 0xE7],
                    REGISTER::BL => vec![0x10, 0xE3],
                    REGISTER::CH => vec![0x10, 0xE5],
                    REGISTER::CL => vec![0x10, 0xE1],
                    REGISTER::DH => vec![0x10, 0xE6],
                    REGISTER::DL => vec![0x10, 0xE2],
                }
                self.generated = gen;
            }, REGISTER::AL => {
                let gen = match src {
                    REGISTER::AH => vec![0x10, 0xC4],
                    REGISTER::AL => vec![0x10, 0xC0],
                    REGISTER::BH => vec![0x10, 0xC7],
                    REGISTER::BL => vec![0x10, 0xC3],
                    REGISTER::CH => vec![0x10, 0xC5],
                    REGISTER::CL => vec![0x10, 0xC1],
                    REGISTER::DH => vec![0x10, 0xC6],
                    REGISTER::DL => vec![0x10, 0xC2],
                }
                self.generated = gen;
            }, REGISTER::BH => {
                let gen = match src {
                    REGISTER::AH => vec![0x10, 0xFC],
                    REGISTER::AL => vec![0x10, 0xF8],
                    REGISTER::BH => vec![0x10, 0xFF],
                    REGISTER::BL => vec![0x10, 0xFB],
                    REGISTER::CH => vec![0x10, 0xFD],
                    REGISTER::CL => vec![0x10, 0xF9],
                    REGISTER::DH => vec![0x10, 0xFE],
                    REGISTER::DL => vec![0x10, 0xFA],
                }
                self.generated = gen;
            }, REGISTER::BL => {
                let gen = match src {
                    REGISTER::AH => vec![0x10, 0xDC],
                    REGISTER::AL => vec![0x10, 0xD8],
                    REGISTER::BH => vec![0x10, 0xDF],
                    REGISTER::BL => vec![0x10, 0xDB],
                    REGISTER::CH => vec![0x10, 0xDD],
                    REGISTER::CL => vec![0x10, 0xD9],
                    REGISTER::DH => vec![0x10, 0xDE],
                    REGISTER::DL => vec![0x10, 0xDA],
                }
                self.generated = gen;
            }, REGISTER::CH => {
                let gen = match src {
                    REGISTER::AH => vec![0x10, 0xEC],
                    REGISTER::AL => vec![0x10, 0xE8],
                    REGISTER::BH => vec![0x10, 0xEF],
                    REGISTER::BL => vec![0x10, 0xEB],
                    REGISTER::CH => vec![0x10, 0xED],
                    REGISTER::CL => vec![0x10, 0xE9],
                    REGISTER::DH => vec![0x10, 0xEE],
                    REGISTER::DL => vec![0x10, 0xEA],
                }
                self.generated = gen;
            }, REGISTER::CL => {
                let gen = match src {
                    REGISTER::AH => vec![0x10, 0xCC],
                    REGISTER::AL => vec![0x10, 0xC8],
                    REGISTER::BH => vec![0x10, 0xCF],
                    REGISTER::BL => vec![0x10, 0xCB],
                    REGISTER::CH => vec![0x10, 0xCD],
                    REGISTER::CL => vec![0x10, 0xC9],
                    REGISTER::DH => vec![0x10, 0xCE],
                    REGISTER::DL => vec![0x10, 0xCA],
                }
                self.generated = gen;
            }, REGISTER::DH => {
                let gen = match src {
                    REGISTER::AH => vec![0x10, 0xF4],
                    REGISTER::AL => vec![0x10, 0xF0],
                    REGISTER::BH => vec![0x10, 0xF7],
                    REGISTER::BL => vec![0x10, 0xF3],
                    REGISTER::CH => vec![0x10, 0xF5],
                    REGISTER::CL => vec![0x10, 0xF1],
                    REGISTER::DH => vec![0x10, 0xF6],
                    REGISTER::DL => vec![0x10, 0xF2],
                }
                self.generated = gen;
            }, REGISTER::DL => {
                let gen = match src {
                    REGISTER::AH => vec![0x10, 0xD4],
                    REGISTER::AL => vec![0x10, 0xD0],
                    REGISTER::BH => vec![0x10, 0xD7],
                    REGISTER::BL => vec![0x10, 0xD3],
                    REGISTER::CH => vec![0x10, 0xD5],
                    REGISTER::CL => vec![0x10, 0xD1],
                    REGISTER::DH => vec![0x10, 0xD6],
                    REGISTER::DL => vec![0x10, 0xD2],
                }
                self.generated = gen;
            }, 

            _ => {},
        }
    }
}