use crate::ir::AsmInstructionEnum::{self, *};
use iced_x86::Register;
use std::{collections::VecDeque, error::Error};

macro_rules! instr {
    ($instr_var:ident => $instr:ident; $last_instr_var:ident => $instr2:ident => $($arg:tt)*) => {
        matches!($instr_var, $instr($($arg)*) if $last_instr_var == $instr2( $($arg)* ))
    };
}

/// Optimizes and makes the incoming ir safe
pub fn Optimize(code: Vec<AsmInstructionEnum>) -> Result<Vec<AsmInstructionEnum>, Box<dyn Error>> {
    let mut opt: VecDeque<AsmInstructionEnum> = VecDeque::new();

    let mut instr = Nop; // Last instruction

    let mut skip = false; // skipes the next element

    for _instr in code {
        if skip {
            skip = false;
        } else if matches!(instr, AddVal(reg, 1) if instr == AddVal(reg, 1)) {
            opt.push_back(Inc(match instr {
                AddVal(reg, _) => reg,
                _ => Register::None,
            }));
        } else if matches!(instr, AddVal(reg, -1) if instr == AddVal(reg, -1)) {
            opt.push_back(Dec(match instr {
                AddVal(reg, _) => reg,
                _ => Register::None,
            }));
        } else if _instr == Ret {
            break;
        } else {
            opt.push_back(instr);
        }

        instr = _instr.to_owned();
    }

    opt.push_back(instr); // last element gets skipped so

    // Setup the stack and add
    opt.push_front(SubVal(Register::RSP, 32));
    opt.push_front(MovReg(Register::RBP, Register::RSP));
    opt.push_front(Push(Register::RBP));
    opt.push_front(Endbr64);

    opt.push_back(AddVal(Register::RSP, 32));
    opt.push_back(AsmInstructionEnum::Pop(Register::RBP)); // for stack safty
    opt.push_back(AsmInstructionEnum::Ret);

    Ok(opt.into())
}
