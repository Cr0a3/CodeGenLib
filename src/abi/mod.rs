use formatic::BinFormat;
use iced_x86::{MemoryOperand, Register};

pub mod windows;
pub mod linux;

#[allow(unused_imports)]
use windows::WindowsAbi;

#[allow(unused_imports)]
use linux::LinuxAbi;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Abi {
    reg_args: usize,
    
    regs_64: Vec<Register>,
    regs_32: Vec<Register>,

    stack_base: i64,

    bin: BinFormat,
}

impl Abi {

    pub fn host() -> Self {
        #[cfg(target_os = "windows")]
        return Abi::windows();

        #[cfg(target_os = "linux")]
        return Abi::linux();
    }

    pub fn reg_args(&self) -> usize {
        self.reg_args
    }

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

    pub fn binary_format(&self) -> BinFormat {
        self.bin
    }
}