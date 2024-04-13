use iced_x86::{MemoryOperand, Register};

#[derive(Clone, Copy)]
pub enum AsmInstructionEnum {
    Ret,
    Nop,
    Endbr64,

    MovVal(Register, u64),
    MovReg(Register, Register),

    Store(Register, MemoryOperand),
    Load(Register, MemoryOperand),

    Call(&'static str),
    Jmp(&'static str),

    Inc(Register),
    Dec(Register),

    IncMem(MemoryOperand),
    DecMem(MemoryOperand),

    AddVal(Register, u64),
    AddReg(Register, Register),
    AddMem(Register, MemoryOperand),

    SubVal(Register, u64),
    SubReg(Register, Register),
    SubMem(Register, MemoryOperand),

    MulVal(Register, u64),
    MulReg(Register, Register),
    MulMem(Register, MemoryOperand),

    Push(Register),
    PushVal(u32),
    Pop(Register),
}

pub enum DataTyp {
    Int32,
    Int64,

    Custom(u64),
}

pub fn adr(adress: i64) -> MemoryOperand {
    MemoryOperand::new(
        Register::None,
        Register::None,
        1,
        adress,
        1,
        false,
        Register::None,
    )
}

pub fn arg(nr: u64, size: u64, prev_size: u64) -> MemoryOperand {
    MemoryOperand::new(
        Register::RBP,
        Register::None,
        1,
        ( nr + size + prev_size + 4 ) as i64 - 1,
        1,
        false,
        Register::None,
    )
}

pub fn arg32(nr: u64) -> Register {
    match nr {
        1 => Register::EDI,
        2 => Register::ESI,
        3 => Register::EDX,
        4 => Register::ECX,
        5 => Register::R8D,
        6 => Register::R9D,
        _ => Register::None,
    }
}

pub fn var(nr: u64, prev_size: u64) -> MemoryOperand {
    let displ = (prev_size + 4) as i64;

    MemoryOperand::new(
        Register::RBP,
        Register::None,
        1,
        -displ,
        1,
        false,
        Register::None,
    )
}
