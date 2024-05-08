use std::collections::HashMap;

use crate::{error::CodeGenLibError, ir::AsmInstructionEnum, Builder};

use super::engine::ExecEngine;

fn intpr(ir: &Vec<AsmInstructionEnum>, funcs: &HashMap<String, (bool, Vec<AsmInstructionEnum>)>, engine: &mut ExecEngine, builder: &Builder) -> Result<(), CodeGenLibError>{
    for instr in ir.clone() {
        match instr {
            AsmInstructionEnum::Ret => engine.ret(),
            AsmInstructionEnum::Nop => {}, // do nothing
            AsmInstructionEnum::Endbr64 => {}, // do nothing (Not important here)
            AsmInstructionEnum::MovVal(reg, value) => {
                match reg {
                    iced_x86::Register::AL => engine.AL = value as u8,
                    iced_x86::Register::CL => engine.CL = value as u8,
                    iced_x86::Register::DL => engine.DL = value as u8,
                    iced_x86::Register::BL => engine.BL = value as u8,
                    iced_x86::Register::AH => engine.AH = value as u8,
                    iced_x86::Register::CH => engine.CH = value as u8,
                    iced_x86::Register::DH => engine.DH = value as u8,
                    iced_x86::Register::BH => engine.BH = value as u8,
                    iced_x86::Register::R8L => engine.R8B = value as u8,
                    iced_x86::Register::R9L => engine.R9B = value as u8,
                    iced_x86::Register::R10L => engine.R10B = value as u8,
                    iced_x86::Register::R11L => engine.R11B = value as u8,
                    iced_x86::Register::R12L => engine.R12B = value as u8,
                    iced_x86::Register::R13L => engine.R13B = value as u8,
                    iced_x86::Register::R14L => engine.R14B = value as u8,
                    iced_x86::Register::R15L => engine.R15B = value as u8,
                    iced_x86::Register::AX => engine.AX = value as u16,
                    iced_x86::Register::CX => engine.CX = value as u16,
                    iced_x86::Register::DX => engine.DX = value as u16,
                    iced_x86::Register::BX => engine.BX = value as u16,
                    iced_x86::Register::SP => engine.SP = value as u16,
                    iced_x86::Register::BP => engine.BP = value as u16,
                    iced_x86::Register::SI => engine.SI = value as u16,
                    iced_x86::Register::DI => engine.DI = value as u16,
                    iced_x86::Register::R8W => engine.R8W = value as u16,
                    iced_x86::Register::R9W => engine.R9W = value as u16,
                    iced_x86::Register::R10W => engine.R10W = value as u16,
                    iced_x86::Register::R11W => engine.R11W = value as u16,
                    iced_x86::Register::R12W => engine.R12W = value as u16,
                    iced_x86::Register::R13W => engine.R13W = value as u16,
                    iced_x86::Register::R14W => engine.R14W = value as u16,
                    iced_x86::Register::R15W => engine.R15W = value as u16,
                    iced_x86::Register::EAX => todo!(),
                    iced_x86::Register::ECX => todo!(),
                    iced_x86::Register::EDX => todo!(),
                    iced_x86::Register::EBX => todo!(),
                    iced_x86::Register::ESP => todo!(),
                    iced_x86::Register::EBP => todo!(),
                    iced_x86::Register::ESI => todo!(),
                    iced_x86::Register::EDI => todo!(),
                    iced_x86::Register::R8D => todo!(),
                    iced_x86::Register::R9D => todo!(),
                    iced_x86::Register::R10D => todo!(),
                    iced_x86::Register::R11D => todo!(),
                    iced_x86::Register::R12D => todo!(),
                    iced_x86::Register::R13D => todo!(),
                    iced_x86::Register::R14D => todo!(),
                    iced_x86::Register::R15D => todo!(),
                    iced_x86::Register::RAX => todo!(),
                    iced_x86::Register::RCX => todo!(),
                    iced_x86::Register::RDX => todo!(),
                    iced_x86::Register::RBX => todo!(),
                    iced_x86::Register::RSP => todo!(),
                    iced_x86::Register::RBP => todo!(),
                    iced_x86::Register::RSI => todo!(),
                    iced_x86::Register::RDI => todo!(),
                    iced_x86::Register::R8 => todo!(),
                    iced_x86::Register::R9 => todo!(),
                    iced_x86::Register::R10 => todo!(),
                    iced_x86::Register::R11 => todo!(),
                    iced_x86::Register::R12 => todo!(),
                    iced_x86::Register::R13 => todo!(),
                    iced_x86::Register::R14 => todo!(),
                    iced_x86::Register::R15 => todo!(),
                    iced_x86::Register::EIP => todo!(),
                    iced_x86::Register::RIP => todo!(),
                    _ => todo!(),
                }
            },
            AsmInstructionEnum::MovReg(_, _) => todo!(),
            AsmInstructionEnum::MovPtr(_, _) => todo!(),
            AsmInstructionEnum::Store(_, _) => todo!(),
            AsmInstructionEnum::Load(_, _) => todo!(),
            AsmInstructionEnum::Call(_) => todo!(),
            AsmInstructionEnum::Jmp(_) => todo!(),
            AsmInstructionEnum::Inc(_) => todo!(),
            AsmInstructionEnum::Dec(_) => todo!(),
            AsmInstructionEnum::IncMem(_) => todo!(),
            AsmInstructionEnum::DecMem(_) => todo!(),
            AsmInstructionEnum::AddVal(_, _) => todo!(),
            AsmInstructionEnum::AddReg(_, _) => todo!(),
            AsmInstructionEnum::AddMem(_, _) => todo!(),
            AsmInstructionEnum::SubVal(_, _) => todo!(),
            AsmInstructionEnum::SubReg(_, _) => todo!(),
            AsmInstructionEnum::SubMem(_, _) => todo!(),
            AsmInstructionEnum::MulVal(_, _) => todo!(),
            AsmInstructionEnum::MulReg(_, _) => todo!(),
            AsmInstructionEnum::MulMem(_, _) => todo!(),
            AsmInstructionEnum::DivVal(_, _) => todo!(),
            AsmInstructionEnum::DivReg(_, _) => todo!(),
            AsmInstructionEnum::DivMem(_, _) => todo!(),
            AsmInstructionEnum::Push(_) => todo!(),
            AsmInstructionEnum::PushVal(_) => todo!(),
            AsmInstructionEnum::PushLabel(_) => todo!(),
            AsmInstructionEnum::PushPtr(_) => todo!(),
            AsmInstructionEnum::Pop(_) => todo!(),
        }
    }
    
    Ok(())
}

pub trait Intepr {
    fn run(&self, entry: &str) -> Result<(), CodeGenLibError>;
}

impl Intepr for Builder {
    fn run(&self, entry: &str) -> Result<(), CodeGenLibError> {
        let entry = entry.to_string();

        if !self.funcs.contains_key(&entry) { 
            return Err(CodeGenLibError::FuncNotExist(entry));
        }

        let entry_func = self.funcs.get(&entry).unwrap();
        let mut exec_engine = ExecEngine::new();

        intpr(&entry_func.1, &self.funcs, &mut exec_engine, &self)?;


        Ok(())
    }
}