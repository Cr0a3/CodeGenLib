use std::{collections::HashMap, error::Error};

use formatic::{Decl, Link, Scope};

use iced_x86::{BlockEncoder, BlockEncoderOptions, Code, Instruction, InstructionBlock};

use crate::asm::AsmInstructionEnum;

pub mod safe;
mod ir_builder;

pub use ir_builder::IrBuilder;
pub use safe::SafeCode;

pub fn resolve(
    funcs: Vec<String>,
    code: &Vec<AsmInstructionEnum>,
) -> Result<(Vec<u8>, Vec<Link>, HashMap<String, Decl>), Box<dyn Error>> {
    let mut decls: HashMap<String, Decl> = HashMap::new();
    let mut links = vec![];
    let mut generated = vec![];

    for instruction in code {
        let instr: Vec<Instruction> = match *instruction {
            AsmInstructionEnum::Ret => vec![Instruction::with(Code::Retnq)],

            AsmInstructionEnum::Endbr64 => vec![Instruction::with(Code::Endbr64)],

            AsmInstructionEnum::Nop => vec![Instruction::with(Code::Nopw)],

            AsmInstructionEnum::Push(reg) => {
                if reg.size() == 8 {
                    vec![Instruction::with1(Code::Push_r64, reg)?]
                } else if reg.size() == 4 {
                    vec![Instruction::with1(Code::Push_r32, reg)?]
                } else if reg.size() == 2 {
                    vec![Instruction::with1(Code::Push_r16, reg)?]
                } else {
                    vec![Instruction::with(Code::Nopq)]
                }
            }

            AsmInstructionEnum::Pop(reg) => {
                if reg.size() == 8 {
                    vec![Instruction::with1(Code::Pop_r64, reg)?]
                } else if reg.size() == 4 {
                    vec![Instruction::with1(Code::Pop_r32, reg)?]
                } else if reg.size() == 2 {
                    vec![Instruction::with1(Code::Pop_r16, reg)?]
                } else {
                    vec![Instruction::with(Code::Nopq)]
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

                vec![Instruction::with_declare_byte_5(0xE8, 0, 0, 0, 0)]
            }

            AsmInstructionEnum::Jmp(target) => {
                let target = target.to_string();

                if !decls.contains_key(&target) && !funcs.contains(&target) {
                    decls.insert(target.clone(), Decl::Function(Scope::Import));
                };

                links.push(Link {
                    from: String::new(),
                    to: target,
                    at: generated.len() + 1,
                });

                vec![Instruction::with_declare_byte_5(0xEB, 0, 0, 0, 0)]
            }

            AsmInstructionEnum::MovVal(reg, value) => {
                if reg.size() == 8 {
                    vec![Instruction::with2(Code::Mov_r64_imm64, reg, value)?]
                } else if reg.size() == 4 {
                    vec![Instruction::with2(Code::Mov_r32_imm32, reg, value)?]
                } else if reg.size() == 2 {
                    vec![Instruction::with2(Code::Mov_r16_imm16, reg, value)?]
                } else if reg.size() == 1 {
                    vec![Instruction::with2(Code::Mov_r8_imm8, reg, value)?]
                } else {
                    vec![Instruction::with(Code::Nopq)]
                }
            }

            AsmInstructionEnum::MovReg(src, target) => {
                if (src.size() == 8) && (target.size() == 8) {
                    vec![Instruction::with2(Code::Mov_r64_rm64, src, target)?]
                } else if (src.size() == 4) && (target.size() == 4) {
                    vec![Instruction::with2(Code::Mov_r32_rm32, src, target)?]
                } else if (src.size() == 2) && (target.size() == 2) {
                    vec![Instruction::with2(Code::Mov_r16_rm16, src, target)?]
                } else if (src.size() == 1) && (target.size() == 1) {
                    vec![Instruction::with2(Code::Mov_r8_rm8, src, target)?]
                } else {
                    vec![Instruction::with(Code::Nopq)]
                }
            }

            AsmInstructionEnum::Load(reg, mem) => {
                if reg.size() == 8 {
                    vec![Instruction::with2(Code::Mov_r64_rm64, reg, mem)?]
                } else if reg.size() == 4 {
                    vec![Instruction::with2(Code::Mov_r32_rm32, reg, mem)?]
                } else if reg.size() == 2 {
                    vec![Instruction::with2(Code::Mov_r16_rm16, reg, mem)?]
                } else if reg.size() == 1 {
                    vec![Instruction::with2(Code::Mov_r8_rm8, reg, mem)?]
                } else {
                    vec![Instruction::with(Code::Nopq)]
                }
            }

            AsmInstructionEnum::Store(reg, mem) => {
                if reg.size() == 8 {
                    vec![Instruction::with2(Code::Mov_rm64_r64, mem, reg)?]
                } else if reg.size() == 4 {
                    vec![Instruction::with2(Code::Mov_rm32_r32, mem, reg)?]
                } else if reg.size() == 2 {
                    vec![Instruction::with2(Code::Mov_rm16_r16, mem, reg)?]
                } else if reg.size() == 1 {
                    vec![Instruction::with2(Code::Mov_rm8_r8, mem, reg)?]
                } else {
                    vec![Instruction::with(Code::Nopq)]
                }
            }

            AsmInstructionEnum::Inc(reg) => {
                if reg.size() == 8 {
                    vec![Instruction::with1(Code::Inc_rm64, reg)?]
                } else if reg.size() == 4 {
                    vec![Instruction::with1(Code::Inc_r32, reg)?]
                } else if reg.size() == 2 {
                    vec![Instruction::with1(Code::Inc_r16, reg)?]
                } else if reg.size() == 1 {
                    vec![Instruction::with1(Code::Inc_rm8, reg)?]
                } else {
                    vec![Instruction::with(Code::Nopq)]
                }
            }

            AsmInstructionEnum::Dec(reg) => {
                if reg.size() == 8 {
                    vec![Instruction::with1(Code::Dec_rm64, reg)?]
                } else if reg.size() == 4 {
                    vec![Instruction::with1(Code::Dec_r32, reg)?]
                } else if reg.size() == 2 {
                    vec![Instruction::with1(Code::Dec_r16, reg)?]
                } else if reg.size() == 1 {
                    vec![Instruction::with1(Code::Dec_rm8, reg)?]
                } else {
                    vec![Instruction::with(Code::Nopq)]
                }
            }

            AsmInstructionEnum::IncMem(mem) => {
                if mem.scale == 8 {
                    vec![Instruction::with1(Code::Inc_rm64, mem)?]
                } else if mem.scale == 4 {
                    vec![Instruction::with1(Code::Inc_rm32, mem)?]
                } else if mem.scale == 2 {
                    vec![Instruction::with1(Code::Inc_rm16, mem)?]
                } else if mem.scale == 1 {
                    vec![Instruction::with1(Code::Inc_rm8, mem)?]
                } else {
                    vec![Instruction::with(Code::Nopq)]
                }
            }

            AsmInstructionEnum::DecMem(mem) => {
                if mem.scale == 8 {
                    vec![Instruction::with1(Code::Dec_rm64, mem)?]
                } else if mem.scale == 4 {
                    vec![Instruction::with1(Code::Dec_rm32, mem)?]
                } else if mem.scale == 2 {
                    vec![Instruction::with1(Code::Dec_rm16, mem)?]
                } else if mem.scale == 1 {
                    vec![Instruction::with1(Code::Dec_rm8, mem)?]
                } else {
                    vec![Instruction::with(Code::Nopq)]
                }
            }

            AsmInstructionEnum::AddVal(reg, value) => {
                if reg.size() == 8 {
                    vec![Instruction::with2(Code::Add_rm64_imm32, reg, value)?]
                } else if reg.size() == 4 {
                    vec![Instruction::with2(Code::Add_rm32_imm32, reg, value)?]
                } else if reg.size() == 2 {
                    vec![Instruction::with2(Code::Add_rm16_imm16, reg, value)?]
                } else if reg.size() == 1 {
                    vec![Instruction::with2(Code::Add_rm8_imm8, reg, value)?]
                } else {
                    vec![Instruction::with(Code::Nopq)]
                }
            }

            AsmInstructionEnum::AddReg(src, target) => {
                if (src.size() == 8) && (target.size() == 8) {
                    vec![Instruction::with2(Code::Add_r64_rm64, src, target)?]
                } else if (src.size() == 4) && (target.size() == 4) {
                    vec![Instruction::with2(Code::Add_r32_rm32, src, target)?]
                } else if (src.size() == 2) && (target.size() == 2) {
                    vec![Instruction::with2(Code::Add_r16_rm16, src, target)?]
                } else if (src.size() == 1) && (target.size() == 1) {
                    vec![Instruction::with2(Code::Add_r8_rm8, src, target)?]
                } else {
                    vec![Instruction::with(Code::Nopq)]
                }
            }

            AsmInstructionEnum::AddMem(reg, mem) => {
                if reg.size() == 8 {
                    vec![Instruction::with2(Code::Add_r64_rm64, reg, mem)?]
                } else if reg.size() == 4 {
                    vec![Instruction::with2(Code::Add_r32_rm32, reg, mem)?]
                } else if reg.size() == 2 {
                    vec![Instruction::with2(Code::Add_r16_rm16, reg, mem)?]
                } else if reg.size() == 1 {
                    vec![Instruction::with2(Code::Add_r8_rm8, reg, mem)?]
                } else {
                    vec![Instruction::with(Code::Nopq)]
                }
            }

            AsmInstructionEnum::SubVal(reg, value) => {
                if reg.size() == 8 {
                    vec![Instruction::with2(Code::Sub_rm64_imm32, reg, value)?]
                } else if reg.size() == 4 {
                    vec![Instruction::with2(Code::Sub_rm32_imm32, reg, value)?]
                } else if reg.size() == 2 {
                    vec![Instruction::with2(Code::Sub_rm16_imm16, reg, value)?]
                } else if reg.size() == 1 {
                    vec![Instruction::with2(Code::Sub_rm8_imm8, reg, value)?]
                } else {
                    vec![Instruction::with(Code::Nopq)]
                }
            }

            AsmInstructionEnum::SubReg(src, target) => {
                if (src.size() == 8) && (target.size() == 8) {
                    vec![Instruction::with2(Code::Sub_r64_rm64, src, target)?]
                } else if (src.size() == 4) && (target.size() == 4) {
                    vec![Instruction::with2(Code::Sub_r32_rm32, src, target)?]
                } else if (src.size() == 2) && (target.size() == 2) {
                    vec![Instruction::with2(Code::Sub_r16_rm16, src, target)?]
                } else if (src.size() == 1) && (target.size() == 1) {
                    vec![Instruction::with2(Code::Sub_r8_rm8, src, target)?]
                } else {
                    vec![Instruction::with(Code::Nopq)]
                }
            }

            AsmInstructionEnum::SubMem(reg, mem) => {
                if reg.size() == 8 {
                    vec![Instruction::with2(Code::Sub_r64_rm64, reg, mem)?]
                } else if reg.size() == 4 {
                    vec![Instruction::with2(Code::Sub_r32_rm32, reg, mem)?]
                } else if reg.size() == 2 {
                    vec![Instruction::with2(Code::Sub_r16_rm16, reg, mem)?]
                } else if reg.size() == 1 {
                    vec![Instruction::with2(Code::Sub_r8_rm8, reg, mem)?]
                } else {
                    vec![Instruction::with(Code::Nopq)]
                }
            }

            AsmInstructionEnum::MulVal(reg, value) => {
                if reg.size() == 8 {
                    vec![Instruction::with2(Code::Imul_r64_rm64_imm32, reg, value)?]
                } else if reg.size() == 4 {
                    vec![Instruction::with2(Code::Imul_r32_rm32_imm32, reg, value)?]
                } else if reg.size() == 2 {
                    vec![Instruction::with2(Code::Imul_r16_rm16_imm16, reg, value)?]
                } else {
                    vec![Instruction::with(Code::Nopq)]
                }
            }

            AsmInstructionEnum::MulReg(src, target) => {
                if (src.size() == 8) && (target.size() == 8) {
                    vec![Instruction::with2(Code::Imul_r64_rm64, src, target)?]
                } else if (src.size() == 4) && (target.size() == 4) {
                    vec![Instruction::with2(Code::Imul_r32_rm32, src, target)?]
                } else if (src.size() == 2) && (target.size() == 2) {
                    vec![Instruction::with2(Code::Imul_r16_rm16, src, target)?]
                } else {
                    vec![Instruction::with(Code::Nopq)]
                }
            }

            AsmInstructionEnum::MulMem(reg, mem) => {
                if reg.size() == 8 {
                    vec![Instruction::with2(Code::Imul_r64_rm64, reg, mem)?]
                } else if reg.size() == 4 {
                    vec![Instruction::with2(Code::Imul_r32_rm32, reg, mem)?]
                } else if reg.size() == 2 {
                    vec![Instruction::with2(Code::Imul_r16_rm16, reg, mem)?]
                } else {
                    vec![Instruction::with(Code::Nopq)]
                }
            }

            AsmInstructionEnum::PushVal(value) => { vec![Instruction::with1(Code::Pushd_imm32, value)?] }

            AsmInstructionEnum::DivVal(_, _) => {vec![Instruction::with(Code::Nopq)]}
            AsmInstructionEnum::DivReg(_, _) => {vec![Instruction::with(Code::Nopq)]},
            AsmInstructionEnum::DivMem(_, _) => {vec![Instruction::with(Code::Nopq)]},
        };

        
        let block = InstructionBlock::new(&instr, 0x0000_1248_FC84_0000);

        let asm = BlockEncoder::encode(64, block, BlockEncoderOptions::NONE)?;

        for byte in asm.code_buffer {
            generated.push(byte)
        }
    }

    Ok((generated, links, decls))
}
