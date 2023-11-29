use crate::{
    generator::Generator,
    problem::{Addition, Division, Multiplication, Problem, ProblemType, Subtraction},
    Result,
};
use rand::distributions::{Distribution, Uniform};

pub struct BasicGenerator {
    pub problems: Vec<Box<dyn Problem>>,
}

impl BasicGenerator {
    pub fn new() -> Self {
        Self { problems: vec![] }
    }
}

impl Generator for BasicGenerator {
    fn generate(
        &mut self,
        count: i32,
        min_value: i32,
        max_value: i32,
        allowed_types: &Vec<ProblemType>,
    ) -> Result<()> {
        self.problems = vec![];

        println!("min value: {min_value}");
        println!("max value: {max_value}");
        let mut digit_rng = rand::thread_rng();
        let digit_uniform = Uniform::from(min_value..=max_value);

        let mut problem_type_rng = rand::thread_rng();
        let problem_type_uniform = Uniform::from(0..allowed_types.len());

        for _ in 0..count {
            let problem_type_index = problem_type_uniform.sample(&mut problem_type_rng);
            let problem_type = allowed_types[problem_type_index];
            let problem: Box<dyn Problem> = match problem_type {
                ProblemType::Addition => {
                    let first_digit = digit_uniform.sample(&mut digit_rng);
                    let second_digit = digit_uniform.sample(&mut digit_rng);
                    Box::new(Addition::new(first_digit, second_digit))
                }
                ProblemType::Subtraction => {
                    let first_digit = digit_uniform.sample(&mut digit_rng);
                    let second_digit = digit_uniform.sample(&mut digit_rng);
                    Box::new(Subtraction::new(first_digit, second_digit))
                }
                ProblemType::Multiplication => {
                    let first_digit = digit_uniform.sample(&mut digit_rng);
                    let second_digit = digit_uniform.sample(&mut digit_rng);
                    Box::new(Multiplication::new(first_digit, second_digit))
                }
                ProblemType::Division => {
                    let first_digit = digit_uniform.sample(&mut digit_rng);
                    let second_digit = digit_uniform.sample(&mut digit_rng);
                    Box::new(Division::new(first_digit, second_digit))
                }
            };

            self.problems.push(problem);
        }

        Ok(())
    }

    fn problems(&self) -> &Vec<Box<dyn Problem>> {
        &self.problems
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::problem::ProblemType;

    #[test]
    fn test_generates_with_correct_number() {
        let mut generator = BasicGenerator::new();
        generator
            .generate(5, 1, 10, &vec![ProblemType::Multiplication])
            .unwrap();

        assert_eq!(generator.problems.len(), 5);
    }
}
