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

    
}