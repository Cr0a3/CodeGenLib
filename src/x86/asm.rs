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

pub fn to_bytes_64(value: u64) -> (u8, u8, u8, u8, u8, u8, u8, u8) {
    let arr = value.to_le_bytes();
    (arr[0], arr[1], arr[2], arr[3], arr[4], arr[5], arr[6], arr[7])
}

pub fn to_bytes_32(value: u32) -> (u8, u8, u8, u8) {
    let arr = value.to_le_bytes();
    (arr[0], arr[1], arr[2], arr[3])
}

pub fn to_bytes_16(value: u16) -> (u8, u8) {
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

    /// Just endbr64
    pub fn endbr64(&mut self) {
        self.generated = vec![0xF3, 0x0F, 0x1E, 0xFA];
    }

    

    

    

    
    /// Add with carry value to 64Bit register
    pub fn adc_64(&mut self, register: REGISTER, value: u64) {
        if value > u32::MAX as u64 {
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
                let gen = match src {
                    REGISTER::AX => vec![0x66, 0x11, 0xC0],
                    REGISTER::BX => vec![0x66, 0x11, 0xC3],
                    REGISTER::CX => vec![0x66, 0x11, 0xC1],
                    REGISTER::DX => vec![0x66, 0x11, 0xC2],
                    _ => vec![0]
                };
                self.generated = gen;
            }, REGISTER::BX => {
                let gen = match src {
                    REGISTER::AX => vec![0x66, 0x11, 0xD8],
                    REGISTER::BX => vec![0x66, 0x11, 0xDB],
                    REGISTER::CX => vec![0x66, 0x11, 0xD9],
                    REGISTER::DX => vec![0x66, 0x11, 0xDA],
                    _ => vec![0]
                };
                self.generated = gen;
            }, REGISTER::CX => {
                let gen = match src {
                    REGISTER::AX => vec![0x66, 0x11, 0xC8],
                    REGISTER::BX => vec![0x66, 0x11, 0xCB],
                    REGISTER::CX => vec![0x66, 0x11, 0xC9],
                    REGISTER::DX => vec![0x66, 0x11, 0xCA],
                    _ => vec![0]
                };
                self.generated = gen;
            }, REGISTER::DX => {
                let gen = match src {
                    REGISTER::AX => vec![0x66, 0x11, 0xD0],
                    REGISTER::BX => vec![0x66, 0x11, 0xD3],
                    REGISTER::CX => vec![0x66, 0x11, 0xD1],
                    REGISTER::DX => vec![0x66, 0x11, 0xD2],
                    _ => vec![0]
                };
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
                    _ => vec![0],
                };
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
                    _ => vec![0],
                };
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
                    _ => vec![0],
                };
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
                    _ => vec![0],
                };
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
                    _ => vec![0],
                };
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
                    _ => vec![0],
                };
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
                    _ => vec![0],
                };
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
                    _ => vec![0],
                };
                self.generated = gen;
            }, 

            _ => {},
        }
    }
}