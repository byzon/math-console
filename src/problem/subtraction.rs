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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gets_question() {
        let problem = Subtraction::new(3, 4);
        assert_eq!(problem.get_question(), "4 - 3");
    }

    #[test]
    fn test_gets_question_with_solution() {
        let problem = Subtraction::new(3, 4);
        assert_eq!(problem.get_question_with_solution(), "4 - 3 = 1");
    }
}
