//! Extensions for architectures

use super::AsmCall::AsmCall;
use super::def::Arch;
use crate::x86::asm::{REGISTER, to_bytes_16, to_bytes_32, to_bytes_64};

pub enum Extension {
    I8086, I80186, I80286, I80386, I80486,
    P5,
    X64,
    SSE, SSE2,
    CLFSH, MONITOR,
}

/// Returns all extensions for archichtecture arch
pub fn all(arch: Arch) -> Vec<Extension> {
    match arch {
        Arch::AMD64(x) => vec![ Extension::I8086, Extension::I80186, Extension::I80286, Extension::I80386, Extension::I80486, 
                                Extension::P5, Extension::X64, Extension::SSE, Extension::SSE2, Extension::CLFSH, 
                                Extension::MONITOR],
    }
}

pub mod I80186;
pub mod I80386;
pub mod shared;

pub trait AMD64: shared::IShared + I80186::I80186 + I80386::I80386 {}