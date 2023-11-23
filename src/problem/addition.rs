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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gets_question() {
        let problem = Addition::new(3, 4);
        assert_eq!(problem.get_question(), "3 + 4");
    }

    #[test]
    fn test_gets_question_with_solution() {
        let problem = Addition::new(3, 4);
        assert_eq!(problem.get_question_with_solution(), "3 + 4 = 7");
    }
}
