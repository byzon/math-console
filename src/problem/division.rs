use crate::problem::Problem;

#[derive(Clone)]
pub struct Division {
    first_digit: i32,
    second_digit: i32,
}

impl Division {
    pub fn new(first_digit: i32, second_digit: i32) -> Self {
        // First digit will be adjusted to be the solution.
        // So if we pass in 8 and 6, the first digit will be 48, and the second will be 6.
        // So the answer is back to 8.
        Self {
            first_digit: first_digit * second_digit,
            second_digit,
        }
    }
}

impl Problem for Division {
    fn get_question(&self) -> String {
        format!("{} / {}", self.first_digit, self.second_digit)
    }

    fn get_question_with_solution(&self) -> String {
        format!(
            "{} / {} = {}",
            self.first_digit,
            self.second_digit,
            self.get_solution()
        )
    }

    fn get_solution(&self) -> i32 {
        // Passing in 8 and 6 will result in formula: (8 * 6) / 6, so 48 * 6 = 8.
        self.first_digit / self.second_digit
    }
}
