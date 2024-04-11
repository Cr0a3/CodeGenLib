use std::{collections::HashMap, error::Error};

use formatic::{Decl, Link, Scope};

use iced_x86::{Code, Encoder, Instruction};

use crate::asm::AsmInstructionEnum;

pub fn resolve(
    funcs: Vec<String>,
    code: &Vec<AsmInstructionEnum>,
) -> Result<(Vec<u8>, Vec<Link>, HashMap<String, Decl>), Box<dyn Error>> {
    let mut decls: HashMap<String, Decl> = HashMap::new();
    let mut links = vec![];
    let mut generated = vec![];

    let mut asm = Encoder::new(64);

    for instruction in code {
        let instr = match *instruction {
            AsmInstructionEnum::Ret => Instruction::with(Code::Retnq),
            AsmInstructionEnum::Push(reg) => {
                if reg.size() == 8 {
                    Instruction::with1(Code::Push_r64, reg)?
                } else if reg.size() == 4 {
                    Instruction::with1(Code::Push_r32, reg)?
                } else if reg.size() == 2 {
                    Instruction::with1(Code::Push_r16, reg)?
                } else {
                    Instruction::with(Code::Nopq)
                }
            }

            AsmInstructionEnum::Pop(reg) => {
                if reg.size() == 8 {
                    Instruction::with1(Code::Pop_r64, reg)?
                } else if reg.size() == 4 {
                    Instruction::with1(Code::Pop_r32, reg)?
                } else if reg.size() == 2 {
                    Instruction::with1(Code::Pop_r16, reg)?
                } else {
                    Instruction::with(Code::Nopd)
                }
            }

            AsmInstructionEnum::Call(target) => {
                let target = target.to_string();

                if !decls.contains_key(&target) && !funcs.contains(&target) {
                    decls.insert(target.clone(), Decl::Function(Scope::Import));
                };

                links.push(Link {
                    from: String::new(),
                    to: target,
                    at: generated.len() + 1,
                });

                Instruction::with_declare_byte_5(0xE8, 0, 0, 0, 0) // call 0
            }

            AsmInstructionEnum::MovVal(reg, value) => {
                if reg.size() == 8 {
                    Instruction::with2(Code::Mov_r64_imm64, reg, value)?
                } else if reg.size() == 4 {
                    Instruction::with2(Code::Mov_r32_imm32, reg, value)?
                } else if reg.size() == 2 {
                    Instruction::with2(Code::Mov_r16_imm16, reg, value)?
                } else if reg.size() == 1 {
                    Instruction::with2(Code::Mov_r8_imm8, reg, value)?
                } else {
                    Instruction::with(Code::Nopd)
                }
            }
        };

        match asm.encode(&instr, 0xfff) {
            Ok(_) => {}
            Err(e) => return Err(Box::from(e)),
        };

        for byte in asm.take_buffer() {
            generated.push(byte)
        }
    }

    Ok((generated, links, decls))
}
