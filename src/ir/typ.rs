#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
/// Stores information about the type
pub enum Type {
    uInt64(u64),
    uInt32(u32),
    iInt64(i64),
    iInt32(i32),
}

impl Type {
    /// Returns if the type can be stored in registers
    pub fn in_reg(&self) -> bool {
        match self {
            Type::uInt64(_) => true,
            Type::uInt32(_) => true,
            Type::iInt64(_) => true,
            Type::iInt32(_) => true,
        }
    }

    /// Returns the size of the type in bytes
    pub fn size(&self) -> u64 {
        match self {
            Type::uInt64(_) => 8,
            Type::uInt32(_) => 4,
            Type::iInt64(_) => 8,
            Type::iInt32(_) => 4,
        }
    }
}