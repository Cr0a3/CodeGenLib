use iced_x86::{Register, MemoryOperand};

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

    Push(Register),
    Pop(Register),
}

pub fn adr(adress: i64) -> MemoryOperand {
    MemoryOperand::new(Register::None, Register::None, 1, adress, 1, false, Register::None)
}

pub fn arg(nr: i64) -> MemoryOperand {
    MemoryOperand::new(Register::RBP, Register::None, 1, -(nr * 4), 1, false, Register::None)
}
