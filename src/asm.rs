pub enum REGISTER {
    EAX, EBX, ECX, EDX,                 // 32bit
    AX, BX, CX, DX,                     // 16bit
    AH, AL, BH, BL, CH, CL, DH, DL,     // 8bit
}

fn to_bytes_32(value: u32) -> (u8, u8, u8, u8) {
    let arr = value.to_be_bytes();
    (arr[3], arr[2], arr[1], arr[0])
}

fn to_bytes_16(value: u16) -> (u8, u8) {
    let arr = value.to_be_bytes();

    (arr[1], arr[0])
}

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
            REGISTER::AH => {
                self.generated = vec![0xb4, value];
            },
            REGISTER::AL => {
                self.generated = vec![0xb0, value];
            },
            REGISTER::BH => {
                self.generated = vec![0xb7, value];
            },
            REGISTER::BL => {
                self.generated = vec![0xb3, value];
            },
            REGISTER::CH => {
                self.generated = vec![0xb5, value];
            },
            REGISTER::CL => {
                self.generated = vec![0xb1, value];
            },
            REGISTER::DH => {
                self.generated = vec![0xb6, value];
            },
            REGISTER::DL => {
                self.generated = vec![0xb2, value];
            },
            _ => {},
        }
    }

    pub fn mov_reg(&mut self, from: REGISTER, to: REGISTER) {
        match from {
            REGISTER::EBX => {
                let to_bin = match to {
                    _ => 0x00,
                };
                self.generated = vec![0x00, to_bin];
            },
            REGISTER::EAX => {
                let to_bin = match to {
                    _ => 0x00,
                };
                self.generated = vec![0x00, to_bin];
            },
            REGISTER::ECX => {
                let to_bin = match to {
                    _ => 0x00,
                };
                self.generated = vec![0x00, to_bin];
            },
            REGISTER::EDX => {
                let to_bin = match to {
                    _ => 0x00,
                };
                self.generated = vec![0x00, to_bin];
            },
            REGISTER::AX => {
                let to_bin = match to {
                    _ => 0x00,
                };
                self.generated = vec![0x00, to_bin];
            },
            REGISTER::BX => {
                let to_bin = match to {
                    _ => 0x00,
                };
                self.generated = vec![0x00, to_bin];
            },
            REGISTER::CX => {
                let to_bin = match to {
                    _ => 0x00,
                };
                self.generated = vec![0x00, to_bin];
            },
            REGISTER::DX => {
                let to_bin = match to {
                    _ => 0x00,
                };
                self.generated = vec![0x00, to_bin];
            },
            REGISTER::AH => {
                let to_bin = match to {
                    _ => 0x00,
                };
                self.generated = vec![0x00, to_bin];
            },
            REGISTER::AL => {
                let to_bin = match to {
                    _ => 0x00,
                };
                self.generated = vec![0x00, to_bin];
            },
            REGISTER::BH => {
                let to_bin = match to {
                    _ => 0x00,
                };
                self.generated = vec![0x00, to_bin];
            },
            REGISTER::BL => {
                let to_bin = match to {
                    _ => 0x00,
                };
                self.generated = vec![0x00, to_bin];
            },
            REGISTER::CH => {
                let to_bin = match to {
                    _ => 0x00,
                };
                self.generated = vec![0x00, to_bin];
            },
            REGISTER::CL => {
                let to_bin = match to {
                    _ => 0x00,
                };
                self.generated = vec![0x00, to_bin];
            },
            REGISTER::DH => {
                let to_bin = match to {
                    _ => 0x00,
                };
                self.generated = vec![0x00, to_bin];
            },
            REGISTER::DL => {
                let to_bin = match to {
                    _ => 0x00,
                };
                self.generated = vec![0x00, to_bin];
            },
        }
    }
}