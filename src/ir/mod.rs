use std::collections::HashMap;

use formatic::{Decl, Link, Scope};

use crate::{
    arch::{ext::AMD64::*, AsmCall::AsmCall},
    asm::{AsmInstructionEnum, REGISTER},
};

pub fn resolve(
    funcs: Vec<String>,
    code: &Vec<AsmInstructionEnum>,
) -> (Vec<u8>, Vec<Link>, HashMap<String, Decl>) {
    let mut decls: HashMap<String, Decl> = HashMap::new();
    let mut links = vec![];
    let mut generated = vec![];

    for byte in AsmCall::endbr64() {
        generated.push(byte);
    }

    generated.push(AsmCall::push(REGISTER::RBP)[0]);

    for byte in AsmCall::mov_reg(REGISTER::RBP, REGISTER::RSP) {
        generated.push(byte);
    }

    for instruction in code {
        let gen = match *instruction {
            AsmInstructionEnum::Ret => AsmCall::ret(),
            AsmInstructionEnum::MovVal(reg, value) => {
                if reg == REGISTER::RAX
                    || reg == REGISTER::RBP
                    || reg == REGISTER::RBX
                    || reg == REGISTER::RCX
                    || reg == REGISTER::RDI
                    || reg == REGISTER::RDX
                    || reg == REGISTER::RIP
                    || reg == REGISTER::RSI
                    || reg == REGISTER::RSP
                {
                    AsmCall::mov_64(reg, value)
                } else if reg == REGISTER::EAX
                    || reg == REGISTER::EBX
                    || reg == REGISTER::ECX
                    || reg == REGISTER::EDX
                {
                    AsmCall::mov_32(reg, value as u32)
                } else if reg == REGISTER::AX
                    || reg == REGISTER::BX
                    || reg == REGISTER::CX
                    || reg == REGISTER::DX
                {
                    AsmCall::mov_16(reg, value as u16)
                } else {
                    AsmCall::mov_8(reg, value as u8)
                }
            }

            AsmInstructionEnum::MovReg(to, from) => AsmCall::mov_reg(to, from),
            AsmInstructionEnum::Call(dest) => {
                let target = dest.to_string();

                if !decls.contains_key(&target) && !funcs.contains(&target) {
                    decls.insert(target.clone(), Decl::Function(Scope::Import));
                };

                links.push(Link {
                    from: String::new(),
                    to: target,
                    at: generated.len() + 1,
                });

                AsmCall::call(0)
            }
        };

        for byte in gen {
            generated.push(byte)
        }
    }

    generated.push(AsmCall::nop()[0]);
    generated.push(AsmCall::pop(REGISTER::RBP)[0]);
    generated.push(AsmCall::ret()[0]);

    (generated, links, decls)
}
