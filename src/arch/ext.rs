//! Extensions for architectures

use super::def::Arch;

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
        Arch::AMD64(_) => vec![ Extension::I8086, Extension::I80186, Extension::I80286, Extension::I80386, Extension::I80486, 
                                Extension::P5, Extension::X64, Extension::SSE, Extension::SSE2, Extension::CLFSH, 
                                Extension::MONITOR],
    }
}

pub mod I8086;
pub mod I80386;
pub mod shared;
pub mod x64;

pub mod AMD64 {
    pub use {super::I8086::I8086, super::I80386::I80386, super::shared::IShared, super::x64::X64};
}