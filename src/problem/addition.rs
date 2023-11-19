use crate::problem::Problem;

#[derive(Clone)]
pub struct Addition {
    first_digit: i32,
    second_digit: i32,
}

impl Addition {
    pub fn new(first_digit: i32, second_digit: i32) -> Self {
        Self {
            first_digit,
            second_digit,
        }
    }
}

impl Problem for Addition {
    fn get_question(&self) -> String {
        format!("{} + {}", self.first_digit, self.second_digit)
    }

    fn get_question_with_solution(&self) -> String {
        format!(
            "{} + {} = {}",
            self.first_digit,
            self.second_digit,
            self.get_solution()
        )
    }

    fn get_solution(&self) -> i32 {
        self.first_digit + self.second_digit
    }
}
