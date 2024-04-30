#![allow(unused_imports)]

use iced_x86::{MemoryOperand, Register};

use super::windows::WindowsAbi;
use super::linux::LinuxAbi;

/// Struct which saves the target ABI
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Abi {
    pub reg_args: usize,
    
    pub regs_64: Vec<Register>,
    pub regs_32: Vec<Register>,

    pub return_reg: Register,

    pub stack_base: i64,
}

impl Abi {
    /// Returns the host ABI
    /// 
    /// ! Only works for **Linux and Windows**
    pub fn host() -> Self {
        #[cfg(target_os = "windows")]
        return Abi::windows();

        #[cfg(target_os = "linux")]
        return Abi::linux();
    }

    /// Returns how many arguments are stored in registers
    pub fn reg_args(&self) -> usize {
        self.reg_args
    }

    /// Returns a 64bit register in which the argument is stored
    /// else the Register is Register::None
    pub fn arg64(&self, nr: usize) -> Register {
        if self.reg_args >= nr {
            let opt = self.regs_64.get(nr);

            if opt.is_some() {
                *opt.unwrap()
            } else {
                Register::None
            }
        } else {
            Register::None
        }
    }

    /// Returns a 32bit register in which the argument is stored
    /// else the register is `Register::None`
    pub fn arg32(&self, nr: usize) -> Register {
        if self.reg_args >= nr {
            let opt = self.regs_32.get(nr);

            if opt.is_some() {
                *opt.unwrap()
            } else {
                Register::None
            }
        } else {
            Register::None
        }
    }

    /// Returns the `MemoryOperand` for the stack position (rbp + pos)
    pub fn stack(&self, pos: i64) -> MemoryOperand {

        let displ = {
            if pos.is_positive() {
                pos + self.stack_base
            } else {
                pos
            }
        };

        MemoryOperand::new(
            Register::RBP,
            Register::None,
            1,
            displ,
            1,
            false,
            Register::None,
        )
    }

    /// Returns the `MemoryOperand` for the memory position
    pub fn mem(&self, adr: i64) -> MemoryOperand {
        MemoryOperand::new(
            Register::None,
            Register::None,
            1,
            adr,
            1,
            false,
            Register::None,
        )
    }

    /// Returns the `Register` in which the return value is stored
    pub fn ret_reg(&self) -> Register {
        self.return_reg
    }
}