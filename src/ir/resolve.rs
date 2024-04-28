use std::{collections::HashMap, error::Error};

use formatic::{Decl, Link, Scope};

use iced_x86::{MemoryOperand, Register};
use iced_x86::{BlockEncoder, BlockEncoderOptions, Code, Instruction, InstructionBlock};

use crate::abi::Abi;
use super::AsmInstructionEnum;

pub fn resolve(
    funcs: Vec<String>,
    labels: Vec<String>,
    code: &Vec<AsmInstructionEnum>,
) -> Result<(Vec<u8>, Vec<Link>, HashMap<String, Decl>), Box<dyn Error>> {
    let mut decls: HashMap<String, Decl> = HashMap::new();
    let mut links = vec![];
    let mut generated = vec![];

    for instruction in code {
        let instr: Vec<Instruction> = match instruction.to_owned() {
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
                    vec![Instruction::with(Code::Nopd)]
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
                    vec![Instruction::with(Code::Nopd)]
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

                vec![Instruction::with_declare_byte_5(0xE9, 0, 0, 0, 0)]
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
                    vec![Instruction::with(Code::Nopd)]
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
                    vec![Instruction::with(Code::Nopd)]
                }
            }

            AsmInstructionEnum::MovPtr(src, target) => {
                let target = target.to_string();

                if !decls.contains_key(&target) && !funcs.contains(&target) {
                    decls.insert(target.clone(), Decl::Data(Scope::Import));
                };

                links.push(Link {
                    from: String::new(),
                    to: target,
                    at: generated.len() + 2,
                });

                if src.size() == 8 {
                    vec![Instruction::with2(Code::Lea_r64_m, src, MemoryOperand::new(
                        Register::RIP, Register::None, 1, 0, 0, true, Register::None
                    ))?]
                } else if src.size() == 4 {
                    vec![Instruction::with2(Code::Lea_r32_m, src, MemoryOperand::new(
                        Register::RIP, Register::None, 1, 0, 0, true, Register::None
                    ))?]
                } else {
                    vec![Instruction::with(Code::Nopd)]
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
                    vec![Instruction::with(Code::Nopd)]
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
                    vec![Instruction::with(Code::Nopd)]
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
                    vec![Instruction::with(Code::Nopd)]
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
                    vec![Instruction::with(Code::Nopd)]
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
                    vec![Instruction::with(Code::Nopd)]
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
                    vec![Instruction::with(Code::Nopd)]
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
                    vec![Instruction::with(Code::Nopd)]
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
                    vec![Instruction::with(Code::Nopd)]
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
                    vec![Instruction::with(Code::Nopd)]
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
                    vec![Instruction::with(Code::Nopd)]
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
                    vec![Instruction::with(Code::Nopd)]
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
                    vec![Instruction::with(Code::Nopd)]
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
                    vec![Instruction::with(Code::Nopd)]
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
                    vec![Instruction::with(Code::Nopd)]
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
                    vec![Instruction::with(Code::Nopd)]
                }
            }

            AsmInstructionEnum::PushVal(value) => {
                if value <= i32::MAX.into() {
                    vec![Instruction::with1(Code::Pushq_imm32, value as i32)?]
                } else if value <= i16::MAX.into() {
                    vec![Instruction::with1(Code::Push_imm16, value as i32)?]
                } else if value <= i8::MAX.into() {
                    vec![Instruction::with1(Code::Pushw_imm8, value as i32)?]
                } else {
                    vec![
                        Instruction::with1(Code::Pushq_imm32, (value & 0xFFFFFFFF) as i32)?,    // Push low bytes
                        Instruction::with1(Code::Pushq_imm32, (value >> 32) as i32)?,           // Push high bytes
                        ]
                }
            }

            AsmInstructionEnum::DivVal(_, _) => {
                vec![Instruction::with(Code::Nopd)]
            }
            AsmInstructionEnum::DivReg(_, _) => {
                vec![Instruction::with(Code::Nopd)]
            }
            AsmInstructionEnum::DivMem(_, _) => {
                vec![Instruction::with(Code::Nopd)]
            }
            
            AsmInstructionEnum::PushLabel(name) => {
                let name = name.to_string();

                if !decls.contains_key(&name) && !labels.contains(&name) {
                    decls.insert(name.clone(), Decl::Data(Scope::Import));
                };

                links.push(Link {
                    from: String::new(),
                    to: name,
                    at: generated.len() + 1,
                });

                vec![Instruction::with1(Code::Push_rm64, Abi::host().mem(0))?]
            }

            AsmInstructionEnum::PushPtr(target) => {
                let target = target.to_string();

                if !decls.contains_key(&target) && !labels.contains(&target) && !funcs.contains(&target) {
                    decls.insert(target.clone(), Decl::Data(Scope::Import));
                };

                links.push(Link {
                    from: String::new(),
                    to: target,
                    at: generated.len() + 1,
                });

                vec![Instruction::with1(Code::Pushq_imm32, 0)?]
            }
        };

        let block = InstructionBlock::new(&instr, 0x0000_1248_FC84_0000);

        let asm = BlockEncoder::encode(64, block, BlockEncoderOptions::NONE)?;

        for byte in asm.code_buffer {
            generated.push(byte)
        }
    }

    Ok((generated, links, decls))
}