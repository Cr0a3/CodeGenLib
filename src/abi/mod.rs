use formatic::BinFormat;
use iced_x86::{MemoryOperand, Register};

pub mod windows;
pub mod linux;

#[allow(unused_imports)]
use windows::WindowsAbi;

#[allow(unused_imports)]
use linux::LinuxAbi;

/// Struct which saves the target ABI
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Abi {
    reg_args: usize,
    
    regs_64: Vec<Register>,
    regs_32: Vec<Register>,

    return_reg: Register,

    stack_base: i64,

    bin: BinFormat,
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

    /// Returns the `BinFormat` for the target abi
    pub fn binary_format(&self) -> BinFormat {
        self.bin
    }

    /// Returns the `Register` in which the return value is stored
    pub fn ret_reg(&self) -> Register {
        self.return_reg
    }
}