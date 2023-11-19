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
        max_value: i32,
        allowed_types: &Vec<ProblemType>,
    ) -> Result<()> {
        self.problems = vec![];

        let mut rng = rand::thread_rng();
        let uniform = Uniform::from(1..=max_value);

        let mut problem_type_rng = rand::thread_rng();
        let problem_type_uniform = Uniform::from(0..allowed_types.len());

        for _ in 0..count {
            let problem_type_index = problem_type_uniform.sample(&mut problem_type_rng);
            let problem_type = allowed_types[problem_type_index];
            let problem: Box<dyn Problem> = match problem_type {
                ProblemType::Addition => {
                    let first_digit = uniform.sample(&mut rng);
                    let second_digit = uniform.sample(&mut rng);
                    Box::new(Addition::new(first_digit, second_digit))
                }
                ProblemType::Subtraction => {
                    let first_digit = uniform.sample(&mut rng);
                    let second_digit = uniform.sample(&mut rng);
                    Box::new(Subtraction::new(first_digit, second_digit))
                }
                ProblemType::Multiplication => {
                    let first_digit = uniform.sample(&mut rng);
                    let second_digit = uniform.sample(&mut rng);
                    Box::new(Multiplication::new(first_digit, second_digit))
                }
                ProblemType::Division => {
                    let first_digit = uniform.sample(&mut rng);
                    let second_digit = uniform.sample(&mut rng);
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
