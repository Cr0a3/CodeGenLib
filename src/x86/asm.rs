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