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

pub fn arg(nr: i64) -> MemoryOperand {
    MemoryOperand::new(
        Register::RBP,
        Register::None,
        1,
        (nr * 4) + 4,
        1,
        false,
        Register::None,
    )
}

pub fn var(nr: i64) -> MemoryOperand {
    MemoryOperand::new(
        Register::RBP,
        Register::None,
        1,
        -(nr * 4),
        1,
        false,
        Register::None,
    )
}
