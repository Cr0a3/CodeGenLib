use iced_x86::Register;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum AsmInstructionEnum {
    Ret,
    Nop,
    Endbr64,

    MovVal(Register, u64),
    MovReg(Register, Register),
    //Store(Register, u64),
    //Load(Register, u64),
    Call(&'static str),
    /*Jmp(&'static str),

    Inc(Register),
    Dec(Register),

    AdcVal(Register, u64),
    AdcReg(Register, Register),*/
    Push(Register),
    Pop(Register),
}
