use super::Game;
use crate::error::{handle_error, AppError};
use crate::generator::Generator;
use crate::problem::{Problem, ProblemType};
use crate::utils::read_input;
use colored::Colorize;
use std::time::Instant;
use strum::IntoEnumIterator;

use crate::utils::{UNICODE_CHECKMARK, UNICODE_X};

use anyhow::{anyhow, Result};

pub struct SimpleGame {
    generator: Box<dyn Generator>,

    start_time: Instant,

    allowed_types: Vec<ProblemType>,
    max_value: i32,

    num_problems: i32,
    num_attempted: i32,
    num_correct: i32,
    answers: Vec<String>,
}

impl SimpleGame {
    pub fn new(generator: Box<dyn Generator>) -> Self {
        Self {
            generator,
            start_time: Instant::now(),
            allowed_types: vec![],
            max_value: 0,
            num_problems: 0,
            num_attempted: 0,
            num_correct: 0,
            answers: vec![],
        }
    }

    fn prompt_problem_types(&mut self) -> Result<()> {
        println!(
            r#"
What do you want to practice? Enter 1 or more letters:
(example: enter "m" for Multiplication, or "ads" for Addition, Division, and Subtraction)

a = Addition
s = Subtraction
m = Multiplication
d = Division
"#
        );

        let allowed_types_input = read_input::<String>().expect("Error reading input");
        self.allowed_types = vec![];
        for problem_type_variant in ProblemType::iter() {
            match problem_type_variant {
                ProblemType::Addition if allowed_types_input.contains("a") => {
                    self.allowed_types.push(problem_type_variant)
                }
                ProblemType::Subtraction if allowed_types_input.contains("s") => {
                    self.allowed_types.push(problem_type_variant)
                }
                ProblemType::Multiplication if allowed_types_input.contains("m") => {
                    self.allowed_types.push(problem_type_variant)
                }
                ProblemType::Division if allowed_types_input.contains("d") => {
                    self.allowed_types.push(problem_type_variant)
                }
                _ => {}
            }
        }

        if allowed_types_input.is_empty() {
            return Err(anyhow!(AppError::InvalidInput));
        }

        Ok(())
    }

    fn prompt_total_problems(&mut self) -> Result<()> {
        println!("How may problems do you want to do?");

        self.num_problems = read_input::<i32>().map_err(|e| handle_error(e, "Can't read input"))?;

        Ok(())
    }

    fn prompt_max_value(&mut self) -> Result<()> {
        println!("What is the highest number to use the problems??");

        self.max_value = read_input::<i32>().map_err(|e| handle_error(e, "Can't read input"))?;

        Ok(())
    }

    fn prompt_for_answer(&self, problem_num: usize, problem: &Box<dyn Problem>) -> Result<String> {
        println!(
            "\nQuestion {} out of {}",
            (problem_num + 1),
            self.num_problems
        );

        println!("{} = ", problem.get_question());

        let input =
            read_input::<String>().map_err(|e| handle_error(e, "Can't read answer input"))?;

        Ok(input)
    }
}

impl Game for SimpleGame {
    fn prepare(&mut self) -> Result<()> {
        self.prompt_problem_types()?;

        Ok(())
    }

    fn start(&mut self) -> Result<()> {
        self.prompt_total_problems()?;

        self.prompt_max_value()?;

        self.generator
            .generate(self.num_problems, self.max_value, &self.allowed_types)
            .map_err(|e| handle_error(e, "Can't generate problems"))?;

        Ok(())
    }

    fn play(&mut self) -> Result<()> {
        self.start_time = Instant::now();

        let problems_iter = self.generator.problems();
        for (index, problem) in problems_iter.iter().enumerate() {
            let answer = self.prompt_for_answer(index, problem)?;
            let solution = problem.get_solution().to_string();

            if "exit" == answer {
                break;
            }

            // TODO: validate bad input

            if solution == answer {
                self.num_correct += 1;
            }

            self.num_attempted += 1;
            self.answers.push(answer);
        }

        Ok(())
    }

    fn end(&mut self) -> Result<()> {
        let elapsed = (Instant::now() - self.start_time).as_secs_f32();
        let problems = self.generator.problems();

        for i in 0..self.num_attempted {
            let problem = problems.get(i as usize).unwrap();
            let solution = problem.get_solution().to_string();
            let answer = &self.answers[i as usize];
            if &solution == answer {
                println!(
                    "{} {}",
                    UNICODE_CHECKMARK.green(),
                    problem.get_question_with_solution()
                );
            } else {
                println!(
                    "{} {} = {} ({})",
                    UNICODE_X.red(),
                    problem.get_question(),
                    answer.red(),
                    solution.green()
                );
            }
        }

        let percent = ((self.num_correct as f32 / self.num_attempted as f32) * 100.0).floor();

        println!(
            "\nYou got {} out of {} ({}%) in {:.2} seconds",
            self.num_correct, self.num_attempted, percent, elapsed
        );

        Ok(())
    }
}
