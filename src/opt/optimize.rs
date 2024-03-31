//! The home of code optimization

use crate::Function;

/// Implement this trait so the builder can optimize your struct
pub trait Optimize {
    fn optimize(&mut self);
}

impl Optimize for Function {
    fn optimize(&mut self) {
        
    }
}