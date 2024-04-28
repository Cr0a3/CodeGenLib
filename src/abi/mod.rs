use iced_x86::{MemoryOperand, Register};

pub mod windows;
pub mod linux;

pub use windows::ABI as WindowsAbi;
pub use linux::ABI as LinuxAbi;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Abi {
    reg_args: usize,
    
    regs_64: Vec<Register>,
    regs_32: Vec<Register>,
}

impl Abi {
    pub fn reg_args(&self) -> usize {
        self.reg_args
    }

    pub fn arg64(&self, nr: usize) -> Register {
        if self.reg_args <= nr {
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

    pub fn arg32(&self, nr: usize) -> Register {
        if self.reg_args <= nr {
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

    pub fn stack(&self, pos: i64) -> MemoryOperand {
        MemoryOperand::new(
            Register::RBP,
            Register::None,
            1,
            pos,
            1,
            false,
            Register::None,
        )
    }

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
}