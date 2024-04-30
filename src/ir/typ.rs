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
    Ptr(Box<Type>), // char* -> so 8 byte pointer

    InVar(String),

    /// used for unkown number of arguments (`printf(char* fmt, ...)`)
    Unlim(Vec<Type>),
}

impl Type {
    /// Returns if the type can be stored in registers (false for undetermined)
    pub fn in_reg(&self) -> bool {
        match self {
            Type::u64(_) => true,
            Type::u32(_) => true,
            Type::i64(_) => true,
            Type::i32(_) => true,
            Type::Bytes(_) => false,
            Type::Str(_) => true,
            Type::Ptr(_) => true,
            Type::Unlim(_) => false,
            Type::InVar(_) => false,
        }
    }

    /// Returns the size of the type in bytes (0 for undetermined)
    pub fn size(&self) -> u64 {
        match self {
            Type::u64(_) => 8,
            Type::u32(_) => 4,
            Type::i64(_) => 8,
            Type::i32(_) => 4,
            Type::Bytes(vec) => (vec.len() - 1) as u64,
            Type::Str(_) => 8,
            Type::Ptr(_) => 8,

            Type::Unlim(_) => 0,
            Type::InVar(_) => 0,
        }
    }

    /// Returns the contents of the type as `Vec<u8>`  (empty for undetermined)
    pub fn bytes(&self) -> Vec<u8> {
        match self {
            Type::u64(val) => val.to_be_bytes().into(),
            Type::u32(val) => val.to_be_bytes().into(),
            Type::i64(val) => val.to_be_bytes().into(),
            Type::i32(val) => val.to_be_bytes().into(),
            Type::Bytes(b) => b.to_vec(),
            Type::Str(b) => b.to_vec(),
            Type::Ptr(target) => (*target).bytes(),
            Type::Unlim(_) => vec![],
            Type::InVar(a) => a.as_bytes().into(),
        }
    }

    /// Returns the type of self without value contents
    pub fn empty(&self) -> Self {
        match self {
            Type::u64(_) => Type::u64(0),
            Type::u32(_) => Type::u32(0),
            Type::i64(_) => Type::i64(0),
            Type::i32(_) => Type::i32(0),
            Type::Bytes(_) => Type::Bytes(vec![]),
            Type::Str(_) => Type::Str(vec![]),

            Type::Ptr(_) => Type::Ptr(Box::from( Type::u64(0) )),

            Type::Unlim(_) => Type::Unlim(vec![]),
            Type::InVar(_) => Type::InVar(String::new()),
        }
    }
}