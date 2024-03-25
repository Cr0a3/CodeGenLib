use std::vec;

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

#[derive(Clone)]
pub struct ASMCall {
    pub generated: Vec<u8>,
}

impl ASMCall {
    pub fn new() -> Self {
        Self {
            generated: vec![]
        }
    }

    pub fn mov_64(&mut self, register: REGISTER, value: u64) {
        match register {
            REGISTER::RAX =>  {
                let (x1, x2, x3, x4, x5, x6, x7, x8) = to_bytes_64(value); 
                self.generated = vec![0x48, 0xb8, x1, x2, x3, x4, x5, x6, x7, x8];
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

    pub fn mov_reg(&mut self, from: REGISTER, to: REGISTER) {
        match from {
            REGISTER::EAX => {
                let gen = match to {
                    REGISTER::EAX => vec![0x89, 0xC0],
                    REGISTER::EBX => vec![0x89, 0xD8],
                    REGISTER::ECX => vec![0x89, 0xC8],
                    REGISTER::EDX => vec![0x89, 0xD0],
                    REGISTER::ESI => vec![0x89, 0xF0],
                    REGISTER::EDI => vec![0x89, 0xF8],
                    REGISTER::EBP => vec![0x89, 0xE8],
                    REGISTER::ESP => vec![0x89, 0xE0],
                    _ => vec![0x00],
                };
                self.generated = gen;
            },
            REGISTER::EBX => {
                let gen = match to {
                    REGISTER::EAX => vec![0x89, 0xC3],
                    REGISTER::EBX => vec![0x89, 0xDB],
                    REGISTER::ECX => vec![0x89, 0xCB],
                    REGISTER::EDX => vec![0x89, 0xD3],
                    REGISTER::ESI => vec![0x89, 0xF3],
                    REGISTER::EDI => vec![0x89, 0xFB],
                    REGISTER::EBP => vec![0x89, 0xEB],
                    REGISTER::ESP => vec![0x89, 0xE3],
                    _ => vec![0x00],
                };
                self.generated = gen;
            },
            REGISTER::ECX => {
                let gen = match to {
                    REGISTER::EAX => vec![0x89, 0xC1],
                    REGISTER::EBX => vec![0x89, 0xD9],
                    REGISTER::ECX => vec![0x89, 0xC9],
                    REGISTER::EDX => vec![0x89, 0xD1],
                    REGISTER::ESI => vec![0x89, 0xF1],
                    REGISTER::EDI => vec![0x89, 0xF9],
                    REGISTER::EBP => vec![0x89, 0xE9],
                    REGISTER::ESP => vec![0x89, 0xE1],
                    _ => vec![0x00],
                };
                self.generated = gen;
            },
            REGISTER::EDX => {
                let gen = match to {
                    REGISTER::EAX => vec![0x89, 0xC2],
                    REGISTER::EBX => vec![0x89, 0xDA],
                    REGISTER::ECX => vec![0x89, 0xCA],
                    REGISTER::EDX => vec![0x89, 0xD2],
                    REGISTER::ESI => vec![0x89, 0xF2],
                    REGISTER::EDI => vec![0x89, 0xFA],
                    REGISTER::EBP => vec![0x89, 0xEA],
                    REGISTER::ESP => vec![0x89, 0xE2],
                    _ => vec![0x00],
                };
                self.generated = gen;
            },
            REGISTER::ESI => {
                let gen = match to {
                    REGISTER::EAX => vec![0x89, 0xC2],
                    REGISTER::EBX => vec![0x89, 0xDA],
                    REGISTER::ECX => vec![0x89, 0xCA],
                    REGISTER::EDX => vec![0x89, 0xD2],
                    REGISTER::ESI => vec![0x89, 0xF2],
                    REGISTER::EDI => vec![0x89, 0xFA],
                    REGISTER::EBP => vec![0x89, 0xEA],
                    REGISTER::ESP => vec![0x89, 0xE2],
                    _ => vec![0x00],
                };
                self.generated = gen;
            },
            REGISTER::EDI => {
                let gen = match to {
                    REGISTER::EAX => vec![0x89, 0xC2],
                    REGISTER::EBX => vec![0x89, 0xDA],
                    REGISTER::ECX => vec![0x89, 0xCA],
                    REGISTER::EDX => vec![0x89, 0xD2],
                    REGISTER::ESI => vec![0x89, 0xF2],
                    REGISTER::EDI => vec![0x89, 0xFA],
                    REGISTER::EBP => vec![0x89, 0xEA],
                    REGISTER::ESP => vec![0x89, 0xE2],
                    _ => vec![0x00],
                };
                self.generated = gen;
            },
            REGISTER::EBP => {
                let gen = match to {
                    REGISTER::EAX => vec![0x89, 0xC5],
                    REGISTER::EBX => vec![0x89, 0xDD],
                    REGISTER::ECX => vec![0x89, 0xCD],
                    REGISTER::EDX => vec![0x89, 0xD5],
                    REGISTER::ESI => vec![0x89, 0xF5],
                    REGISTER::EDI => vec![0x89, 0xFD],
                    REGISTER::EBP => vec![0x89, 0xED],
                    REGISTER::ESP => vec![0x89, 0xE5],
                    _ => vec![0x00],
                };
                self.generated = gen;
            },
            REGISTER::ESP => {
                let gen = match to {
                    REGISTER::EAX => vec![0x89, 0xC4],
                    REGISTER::EBX => vec![0x89, 0xDC],
                    REGISTER::ECX => vec![0x89, 0xCC],
                    REGISTER::EDX => vec![0x89, 0xD4],
                    REGISTER::ESI => vec![0x89, 0xFC],
                    REGISTER::EDI => vec![0x89, 0xFC],
                    REGISTER::EBP => vec![0x89, 0xEC],
                    REGISTER::ESP => vec![0x89, 0xE4],
                    _ => vec![0x00],
                };
                self.generated = gen;
            },
            REGISTER::AX => {
                let gen = match to {
                    REGISTER::AX => vec![0x66, 0x89, 0xc0],
                    REGISTER::BX => vec![0x66, 0x89, 0xD8],
                    REGISTER::CX => vec![0x66, 0x89, 0xC8],
                    REGISTER::DX => vec![0x66, 0x89, 0xD0],
                    _ => vec![0x00]
                };
                self.generated = gen;
            },
            REGISTER::BX => {
                let gen = match to {
                    REGISTER::AX => vec![0x66, 0x89, 0xc3],
                    REGISTER::BX => vec![0x66, 0x89, 0xDB],
                    REGISTER::CX => vec![0x66, 0x89, 0xCB],
                    REGISTER::DX => vec![0x66, 0x89, 0xD3],
                    _ => vec![0x00]
                };
                self.generated = gen;
            },
            REGISTER::CX => {
                let gen = match to {
                    REGISTER::AX => vec![0x66, 0x89, 0xc1],
                    REGISTER::BX => vec![0x66, 0x89, 0xD9],
                    REGISTER::CX => vec![0x66, 0x89, 0xC9],
                    REGISTER::DX => vec![0x66, 0x89, 0xD1],
                    _ => vec![0x00]
                };
                self.generated = gen;
            },
            REGISTER::DX => {
                let gen = match to {
                    REGISTER::AX => vec![0x66, 0x89, 0xC2],
                    REGISTER::BX => vec![0x66, 0x89, 0xDA],
                    REGISTER::CX => vec![0x66, 0x89, 0xCA],
                    REGISTER::DX => vec![0x66, 0x89, 0xD2],
                    _ => vec![0x00]
                };
                self.generated = gen;
            },
            REGISTER::AH => {
                let gen = match to {
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
                let gen = match to {
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
                let gen = match to {
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
                let gen = match to {
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
                let gen = match to {
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
                let gen = match to {
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
                let gen = match to {
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
                let gen = match to {
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
            REGISTER::EIP => {},
        }
    }

    pub fn to_memory(&mut self, adress: u32, target: REGISTER) {
        match target {
            REGISTER::EAX => {
                let (x1, x2, x3, x4) = to_bytes_32(adress);
                self.generated = vec![0xA3, x1, x2, x3, x4];
            },
            REGISTER::EBX => {
                let (x1, x2, x3, x4) = to_bytes_32(adress);
                self.generated = vec![0x89, 0x1D, x1, x2, x3, x4];
            },
            REGISTER::ECX => {
                let (x1, x2, x3, x4) = to_bytes_32(adress);
                self.generated = vec![0x89, 0x0D, x1, x2, x3, x4];
            },
            REGISTER::EDX => {
                let (x1, x2, x3, x4) = to_bytes_32(adress);
                self.generated = vec![0x89, 0x15, x1, x2, x3, x4];
            },
            REGISTER::ESI => {
                let (x1, x2, x3, x4) = to_bytes_32(adress);
                self.generated = vec![0x89, 0x35, x1, x2, x3, x4];
            },
            REGISTER::EDI => {
                let (x1, x2, x3, x4) = to_bytes_32(adress);
                self.generated = vec![0x89, 0x3d, x1, x2, x3, x4];
            },
            REGISTER::EBP => {
                let (x1, x2, x3, x4) = to_bytes_32(adress);
                self.generated = vec![0x89, 0x2d, x1, x2, x3, x4];
            },
            REGISTER::ESP => {
                let (x1, x2, x3, x4) = to_bytes_32(adress);
                self.generated = vec![0x89, 0x25, x1, x2, x3, x4];
            },
            REGISTER::AX => {
                let (x1, x2, x3, x4) = to_bytes_32(adress);
                self.generated = vec![0x66, 0xa3, x1, x2, x3, x4];
            },
            REGISTER::BX => {
                let (x1, x2, x3, x4) = to_bytes_32(adress);
                self.generated = vec![0x66, 0x89, 0x1D, x1, x2, x3, x4];
            },
            REGISTER::CX => {
                let (x1, x2, x3, x4) = to_bytes_32(adress);
                self.generated = vec![0x66, 0x89, 0x0D, x1, x2, x3, x4];
            },
            REGISTER::DX => {
                let (x1, x2, x3, x4) = to_bytes_32(adress);
                self.generated = vec![0x66, 0x89, 0x15, x1, x2, x3, x4];
            },
            REGISTER::AH => {
                let (x1, x2, x3, x4) = to_bytes_32(adress);
                self.generated = vec![0x88, 0x25, x1, x2, x3, x4];
            },
            REGISTER::AL => {
                let (x1, x2, x3, x4) = to_bytes_32(adress);
                self.generated = vec![0xa2, x1, x2, x3, x4];
            },
            REGISTER::BH => {
                let (x1, x2, x3, x4) = to_bytes_32(adress);
                self.generated = vec![0x88, 0x3d, x1, x2, x3, x4];
            },
            REGISTER::BL => {
                let (x1, x2, x3, x4) = to_bytes_32(adress);
                self.generated = vec![0x88, 0x1D, x1, x2, x3, x4];
            },
            REGISTER::CH => {
                let (x1, x2, x3, x4) = to_bytes_32(adress);
                self.generated = vec![0x88, 0x2D, x1, x2, x3, x4];
            },
            REGISTER::CL => {
                let (x1, x2, x3, x4) = to_bytes_32(adress);
                self.generated = vec![0x88, 0x0D, x1, x2, x3, x4];
            },
            REGISTER::DH => {
                let (x1, x2, x3, x4) = to_bytes_32(adress);
                self.generated = vec![0x88, 0x35, x1, x2, x3, x4];
            },
            REGISTER::DL => {
                let (x1, x2, x3, x4) = to_bytes_32(adress);
                self.generated = vec![0x88, 0x15, x1, x2, x3, x4];
            },
            _ => {},
        }
    }

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
            REGISTER::ESI => {
                let (x1, x2, x3, x4) = to_bytes_32(adress);
                self.generated = vec![0x8b, 0x35, x1, x2, x3, x4];
            },
            REGISTER::EDI => {
                let (x1, x2, x3, x4) = to_bytes_32(adress);
                self.generated = vec![0x8b, 0x3d, x1, x2, x3, x4];
            },
            REGISTER::EBP => {
                let (x1, x2, x3, x4) = to_bytes_32(adress);
                self.generated = vec![0x8b, 0x3d, x1, x2, x3, x4];
            },
            REGISTER::ESP => {
                let (x1, x2, x3, x4) = to_bytes_32(adress);
                self.generated = vec![0x8b, 0x25, x1, x2, x3, x4];
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

    pub fn push(&mut self, reg: REGISTER) {
        match reg {
            REGISTER::AX =>     {   self.generated = vec![0x66, 0x50]; }
            REGISTER::CX =>     {   self.generated = vec![0x66, 0x51]; }
            REGISTER::DX =>     {   self.generated = vec![0x66, 0x52]; }
            REGISTER::BX =>     {   self.generated = vec![0x66, 0x53]; }
            REGISTER::EAX =>    {   self.generated = vec![0x50]; }
            REGISTER::ECX =>    {   self.generated = vec![0x51]; }
            REGISTER::EDX =>    {   self.generated = vec![0x52]; }
            REGISTER::EBX =>    {   self.generated = vec![0x53]; }
            REGISTER::ESP =>    {   self.generated = vec![0x54]; }
            REGISTER::EBP =>    {   self.generated = vec![0x55]; }
            REGISTER::ESI =>    {   self.generated = vec![0x56]; }
            REGISTER::EDI =>    {   self.generated = vec![0x57]; }
            REGISTER::EIP =>    {   self.generated = vec![0xff, 0x45, 0x00, 0x00, 0x00, 0x00]; }
            _ => {}
        }
    }

    pub fn pop(&mut self, reg: REGISTER) {
        match reg {
            REGISTER::AX =>     {   self.generated = vec![0x66, 0x58]; }
            REGISTER::CX =>     {   self.generated = vec![0x66, 0x59]; }
            REGISTER::DX =>     {   self.generated = vec![0x66, 0x5a]; }
            REGISTER::BX =>     {   self.generated = vec![0x66, 0x5b]; }
            REGISTER::EAX =>    {   self.generated = vec![0x58]; }
            REGISTER::ECX =>    {   self.generated = vec![0x59]; }
            REGISTER::EDX =>    {   self.generated = vec![0x5a]; }
            REGISTER::EBX =>    {   self.generated = vec![0x5b]; }
            REGISTER::ESP =>    {   self.generated = vec![0x5c]; }
            REGISTER::EBP =>    {   self.generated = vec![0x5d]; }
            REGISTER::ESI =>    {   self.generated = vec![0x5e]; }
            REGISTER::EDI =>    {   self.generated = vec![0x5f]; }
            REGISTER::EIP =>    {   self.generated = vec![0x8F, 0x05, 0x00, 0x00, 0x00, 0x00]; }
            _ => {},
        }
    }

    pub fn nop(&mut self) {
        self.generated = vec![0x90];
    }

    pub fn jmp(&mut self, adress: u32) {
        let (x1, x2, x3, x4) = to_bytes_32(adress);
        self.generated = vec![0xe9, x1, x2, x3, x4];
    }

    pub fn call(&mut self, adress: u32) {
        let (x1, x2, x3, x4) = to_bytes_32(adress);
        self.generated = vec![0xe8, x1, x2, x3, x4];
    }

    pub fn ret(&mut self) {
        self.generated = vec![0xc3];
    }

    pub fn int(&mut self, nr: u8) {
        self.generated = vec![0xCD, nr];
    }

    pub fn endbr32(&mut self) {
        self.generated = vec![0x00];
    }
}