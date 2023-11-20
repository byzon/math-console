use crate::problem::Problem;

#[derive(Clone)]
pub struct Subtraction {
    first_digit: i32,
    second_digit: i32,
}

impl Subtraction {
    pub fn new(first_digit: i32, second_digit: i32) -> Self {
        // Make sure first is greater than second, so we avoid negative numbers.
        Self {
            first_digit: first_digit.max(second_digit),
            second_digit: first_digit.min(second_digit),
        }
    }
}

impl Problem for Subtraction {
    fn get_question(&self) -> String {
        format!("{} - {}", self.first_digit, self.second_digit)
    }

    fn get_question_with_solution(&self) -> String {
        format!(
            "{} - {} = {}",
            self.first_digit,
            self.second_digit,
            self.get_solution()
        )
    }

    fn get_solution(&self) -> i32 {
        self.first_digit - self.second_digit
    }
}
