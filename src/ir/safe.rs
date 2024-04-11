use iced_x86::Register;

use crate::asm::AsmInstructionEnum;
use std::{collections::VecDeque, error::Error};

pub fn SafeCode(code: Vec<AsmInstructionEnum>) -> Result<Vec<AsmInstructionEnum>, Box<dyn Error>> {

    let mut ret = VecDeque::from(code);

    // endbr64
    // push rbp
    // mov rbp, rsp
    ret.push_front(AsmInstructionEnum::MovReg(Register::RBP, Register::RSP));
    ret.push_front(AsmInstructionEnum::Push(Register::RBP));
    ret.push_front(AsmInstructionEnum::Endbr64);

    // nop
    // pop rbp
    // ret
    ret.push_back(AsmInstructionEnum::Ret);
    ret.push_back(AsmInstructionEnum::Pop(Register::RBP));
    ret.push_back(AsmInstructionEnum::Nop);

    Ok(ret.into())
}