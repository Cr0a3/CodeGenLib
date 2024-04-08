use std::fmt;

#[allow(non_snake_case)]

/// An error type which can ocure during any CodeGenLib functions
/// which stores the error information
#[derive(Debug, Clone)]
pub enum CodeGenLibError {
    JitFunctionsNoExtern,
    AdrOutOfMem,
}

pub type Result<T> = std::result::Result<T, CodeGenLibError>;

impl fmt::Display for CodeGenLibError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let msg = match self {
            CodeGenLibError::JitFunctionsNoExtern => "JitError: externs aren't allowed",
            CodeGenLibError::AdrOutOfMem => "VarManager: you allocated to much memory",
        };

        write!(f, "{}", msg)
    }
}
