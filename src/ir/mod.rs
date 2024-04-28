use iced_x86::{MemoryOperand, Register};

pub mod ir_builder;
pub mod typ;
pub mod resolve;

pub use ir_builder::IrBuilder;
pub use ir_builder::IrFunctionBuilder;
pub use typ::Type;

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