use iced_x86::{MemoryOperand, Register};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AsmInstructionEnum {
    Ret,
    Nop,
    Endbr64,

    MovVal(Register, i64),
    MovReg(Register, Register),
    MovPtr(Register, String),

    Store(Register, MemoryOperand),
    Load(Register, MemoryOperand),

    Call(String),
    Jmp(String),

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
    PushVal(i64),
    PushLabel(String),
    PushPtr(String),

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
    let windows = vec![Register::ECX, Register::EDX, Register::R8D,  Register::R9D];
    let linux   = vec![Register::EDI, Register::ESI, Register::EDX, Register::ECX];

    let regs = {
        if cfg!(target_os = "windows") {
            windows
        } else {
            linux
        }
    };

    match nr {
        1 => regs[0],
        2 => regs[1],
        3 => regs[2],
        4 => regs[3],

        _ => Register::None,
    }
}

pub fn arg64(nr: i64) -> Register {
    let windows = vec![Register::RCX, Register::RDX, Register::R8,  Register::R9];
    let linux   = vec![Register::RDI, Register::RSI, Register::RDX, Register::RCX];

    let regs = {
        if cfg!(target_os = "windows") {
            windows
        } else {
            linux
        }
    };

    match nr {
        1 => regs[0],
        2 => regs[1],
        3 => regs[2],
        4 => regs[3],

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
