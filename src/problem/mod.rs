mod addition;
mod division;
mod multiplication;
mod subtraction;

pub use addition::*;
pub use division::*;
pub use multiplication::*;
pub use subtraction::*;

use std::fmt::{Display, Formatter, Result};

#[derive(Clone, Copy)]
pub enum ProblemType {
    Addition,
    Subtraction,
    Multiplication,
    Division,
}

pub trait Problem {
    fn get_solution(&self) -> i32;

    fn get_question(&self) -> String;

    fn get_question_with_solution(&self) -> String;
}

impl Display for dyn Problem {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}", self.get_question())
    }
}
