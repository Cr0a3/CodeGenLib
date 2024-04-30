//! Make easy cross compilation to linux/windows

use formatic::BinFormat;

pub mod windows;
pub mod linux;
pub mod abi;

pub use abi::Abi;

use self::{linux::LinuxAbi, windows::WindowsAbi};


pub struct Target {
    pub bin: BinFormat,
    pub abi: Abi,
}

impl Target {
    /// Returns host target
    pub fn host() -> Self {
        Target { 
            bin: BinFormat::host(),
            abi: Abi::host(),
        }
    }
    
    /// Returns the target struct with windows values
    pub fn windows() -> Self {
        Target { 
            bin: BinFormat::Coff,
            abi: Abi::windows(),
        }
    }

    /// Returns the target struct with linux values
    pub fn linux() -> Self {
        Target { 
            bin: BinFormat::Elf,
            abi: Abi::linux(),
        }
    }
}