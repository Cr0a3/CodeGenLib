//! The home of code optimization

use crate::Function;

/// Implement this trait so the builder can optimize your struct
pub trait Optimize {
    fn optimize(&mut self);
}

impl<'a> Optimize for Function<'a> {
    fn optimize(&mut self) {

    }
}