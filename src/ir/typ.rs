#[allow(non_camel_case_types)]
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
/// Stores information about the type
pub enum Type {
    u64(u64),
    u32(u32),
    i64(i64),
    i32(i32),

    Bytes(Vec<u8>),
    Str(Vec<u8>), // char* -> so 8 byte pointer
}

impl Type {
    /// Returns if the type can be stored in registers
    pub fn in_reg(&self) -> bool {
        match self {
            Type::u64(_) => true,
            Type::u32(_) => true,
            Type::i64(_) => true,
            Type::i32(_) => true,
            Type::Bytes(_) => false,
            Type::Str(_) => true,
        }
    }

    /// Returns the size of the type in bytes
    pub fn size(&self) -> u64 {
        match self {
            Type::u64(_) => 8,
            Type::u32(_) => 4,
            Type::i64(_) => 8,
            Type::i32(_) => 4,
            Type::Bytes(vec) => (vec.len() - 1) as u64,
            Type::Str(_) => 8,
        }
    }

    pub fn bytes(&self) -> Vec<u8> {
        match self {
            Type::u64(val) => val.to_be_bytes().into(),
            Type::u32(val) => val.to_be_bytes().into(),
            Type::i64(val) => val.to_be_bytes().into(),
            Type::i32(val) => val.to_be_bytes().into(),
            Type::Bytes(b) => b.to_vec(),
            Type::Str(b) => b.to_vec(),
        }
    }
}