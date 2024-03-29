//! Save and store Architectures

use crate::arch::ext::Extension;

/// Enum of supported architectures with extensions to select
/// 
/// `AMD64` - x86-64Bit; x64
pub enum Arch { // enum of supported archs
    AMD64(Vec<Extension>),
}