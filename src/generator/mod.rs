mod basic;

pub use basic::*;

use crate::problem::{Problem, ProblemType};
use crate::Result;

pub trait Generator {
    fn generate(
        &mut self,
        count: i32,
        min_value: i32,
        max_value: i32,
        allowed_types: &Vec<ProblemType>,
    ) -> Result<()>;

    fn problems(&self) -> &Vec<Box<dyn Problem>>;
}
