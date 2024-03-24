#[derive(Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum REGISTER {
    EAX, EBX, ECX, EDX,                 // 32bit
    AX, BX, CX, DX,                     // 16bit
    AH, AL, BH, BL, CH, CL, DH, DL,     // 8bit

    ESI, EDI, EBP, EIP, ESP,            // Indexs + Pointers (32bit)
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

    pub fn mov_32(&mut self, register: REGISTER, value: u32) {
        match register {
            REGISTER::EAX =>  {
                let (x1, x2, x3, x4) = to_bytes_32(value); 
                self.generated = vec![0xb8, x1, x2, x3, x4];
            },
            REGISTER::ECX => {
                let (x1, x2, x3, x4) = to_bytes_32(value);
                self.generated = vec![0xb9, x1, x2, x3, x4];
            },
            REGISTER::EDX => {
                let (x1, x2, x3, x4) = to_bytes_32(value);
                self.generated = vec![0xba, x1, x2, x3, x4];
            },
            REGISTER::EBX => {
                let (x1, x2, x3, x4) = to_bytes_32(value);
                self.generated = vec![0xbb, x1, x2, x3, x4];
            },
            REGISTER::ESP => {
                let (x1, x2, x3, x4) = to_bytes_32(value);
                self.generated = vec![0xbc, x1, x2, x3, x4];
            },
            REGISTER::EBP => {
                let (x1, x2, x3, x4) = to_bytes_32(value);
                self.generated = vec![0xbd, x1, x2, x3, x4];
            }
            REGISTER::ESI => {
                let (x1, x2, x3, x4) = to_bytes_32(value);
                self.generated = vec![0xbe, x1, x2, x3, x4];
            }
            REGISTER::EDI => {
                let (x1, x2, x3, x4) = to_bytes_32(value);
                self.generated = vec![0xbf, x1, x2, x3, x4];
            }
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
                match to {
                    REGISTER::EAX => todo!(),
                    REGISTER::EBX => todo!(),
                    REGISTER::ECX => todo!(),
                    REGISTER::EDX => todo!(),
                    REGISTER::AX => todo!(),
                    REGISTER::BX => todo!(),
                    REGISTER::CX => todo!(),
                    REGISTER::DX => todo!(),
                    REGISTER::AH => todo!(),
                    REGISTER::AL => todo!(),
                    REGISTER::BH => todo!(),
                    REGISTER::BL => todo!(),
                    REGISTER::CH => todo!(),
                    REGISTER::CL => todo!(),
                    REGISTER::DH => todo!(),
                    REGISTER::DL => todo!(),
                    REGISTER::ESI => todo!(),
                    REGISTER::EDI => todo!(),
                    REGISTER::EBP => todo!(),
                    REGISTER::EIP => todo!(),
                    REGISTER::ESP => todo!(),
                };
            REGISTER::EBX => {
                match to {
                    
                };
            REGISTER::ECX => {
                match to {
                    
                };
            REGISTER::EDX => {
                match to {
                    
                };
            REGISTER::AX => {
                match to {
                    
                };
            REGISTER::BX => {
                match to {
                    
                };
            REGISTER::CX => {
                match to {
                    
                };
            REGISTER::DX => {
                match to {
                    
                };
            REGISTER::AH => {
                match to {
                    
                };
            REGISTER::AL => {
                match to {
                    
                };
            REGISTER::BH => {
                match to {
                    
                };
            REGISTER::BL => {
                match to {
                    
                };
            REGISTER::CH => {
                match to {
                    
                };
            REGISTER::CL => {
                match to {
                    
                };
            REGISTER::DH => {
                match to {
                    
                };
            REGISTER::DL => {
                match to {
                    
                };
            REGISTER::ESI => {
                match to {
                    
                };
            REGISTER::EDI => {
                match to {
                    
                };
            REGISTER::EBP => {
                match to {
                    
                };
            REGISTER::EIP => {
                match to {
                    
                };
            REGISTER::ESP => {
                match to {
                    
                };
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
}