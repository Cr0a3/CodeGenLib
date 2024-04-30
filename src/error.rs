//! Sublib for errors

use std::fmt;

#[allow(non_snake_case)]

/// An error type which can ocure during any CodeGenLib functions
/// which stores the error information
#[derive(Debug, Clone)]
pub enum CodeGenLibError {
    VarNotExist(String),
    FuncNotExist(String),
}

/// Result which stores T + CodeGenLibError
pub type Result<T> = std::result::Result<T, CodeGenLibError>;

impl fmt::Display for CodeGenLibError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let msg = match self {
            CodeGenLibError::VarNotExist(x) => format!("var {x} doesn't exits"),
            CodeGenLibError::FuncNotExist(x) => format!("func {x} doesn't exits"),
        };

        write!(f, "{}", msg)
    }
}

impl std::error::Error for CodeGenLibError {}
