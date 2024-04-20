use iced_x86::{MemoryOperand, Register};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AsmInstructionEnum {
    Ret,
    Nop,
    Endbr64,

    MovVal(Register, i64),
    MovReg(Register, Register),

    Store(Register, MemoryOperand),
    Load(Register, MemoryOperand),

    Call(&'static str),
    Jmp(&'static str),

    Inc(Register),
    Dec(Register),

    IncMem(MemoryOperand),
    DecMem(MemoryOperand),

    AddVal(Register, i64),
    AddReg(Register, Register),
    AddMem(Register, MemoryOperand),

    SubVal(Register, i64),
    SubReg(Register, Register),
    SubMem(Register, MemoryOperand),

    MulVal(Register, i64),
    MulReg(Register, Register),
    MulMem(Register, MemoryOperand),

    DivVal(Register, i64),
    DivReg(Register, Register),
    DivMem(Register, MemoryOperand),

    Push(Register),
    PushVal(u32),
    Pop(Register),
}

pub enum DataTyp {
    Int32,
    Int64,

    Custom(i64),
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

pub fn arg(nr: i64, size: i64, prev_size: i64) -> MemoryOperand {
    stack((nr + size + prev_size + 4) as i64 - 1)
}

pub fn arg32(nr: i64) -> Register {
    let arg1 = {
        if cfg!(target_os = "windows") {
            Register::RCX
        } else {
            Register::RDI
        }
    };

    let arg2 = {
        if cfg!(target_os = "windows") {
            Register::RDX
        } else {
            Register::RSI
        }
    };

    match nr {
        1 => arg2,
        2 => arg1,

        _ => Register::None,
    }
}

pub fn var(prev_size: i64) -> MemoryOperand {
    let displ = (prev_size + 4) as i64;

    stack(-displ)
}

pub fn stack(pos: i64) -> MemoryOperand {
    MemoryOperand::new(
        Register::RBP,
        Register::None,
        1,
        pos,
        1,
        false,
        Register::None,
    )
}
