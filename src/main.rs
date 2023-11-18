use colored::Colorize;
use rand::distributions::{Distribution, Uniform};
use rand::Rng;
use std::error;
use std::fmt;
use std::io;
use std::time::{Duration, Instant};

struct AppError;

type Result<T> = std::result::Result<T, Box<dyn error::Error>>;

const UNICODE_CHECKMARK: &str = "\u{2714}";
const UNICODE_X: &str = "\u{0078}";

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "App error")
    }
}

pub enum Operation {
    Multiply,
}

pub trait Problem {
    fn get_solution(&self) -> i32;

    fn get_question(&self) -> String;
    fn get_question_with_solution(&self) -> String;
}

#[derive(Clone)]
struct Multiplication {
    first_digit: i32,
    second_digit: i32,
}

impl Multiplication {
    fn new(first_digit: i32, second_digit: i32) -> Self {
        Self {
            first_digit,
            second_digit,
        }
    }
}

impl Problem for Multiplication {
    fn get_question(&self) -> String {
        format!("{} * {}", self.first_digit, self.second_digit)
    }

    fn get_question_with_solution(&self) -> String {
        format!(
            "{} * {} = {}",
            self.first_digit,
            self.second_digit,
            self.get_solution()
        )
    }

    fn get_solution(&self) -> i32 {
        self.first_digit * self.second_digit
    }
}

impl fmt::Display for Multiplication {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.get_question())
    }
}

fn main() {
    println!("Let's do some math!");

    loop {
        println!("How may problems do you want to do?");
        let mut num_problems = String::new();
        io::stdin()
            .read_line(&mut num_problems)
            .expect("Invalid input");

        let num_problems = num_problems
            .trim()
            .parse::<i32>()
            .expect("invalid number of problems entered");

        let mut problems = vec![];
        let _ = generate_problems(&mut problems, num_problems).map_err(|e| {
            println!("Error generating problems: {e:?}");
        });

        let mut answers = vec![];
        let start_time = std::time::Instant::now();

        let mut num_correct = 0;
        let mut num_attempted = 0;
        let mut buf = String::new();
        for (index, problem) in problems.iter().enumerate() {
            let solution = problem.get_solution().to_string();

            println!("\nQuestion {} out of {}", (index + 1), num_problems);
            println!("{} = ", problem.get_question());

            io::stdin()
                .read_line(&mut buf)
                .expect("Failed to read line.");

            let input = buf.trim().to_lowercase();

            match input.trim().to_lowercase().as_str() {
                "exit" => break,
                _ => {
                    // TODO: validate bad input
                    if solution == input {
                        num_correct += 1;
                    }

                    answers.push(input);

                    num_attempted += 1;
                }
            }

            buf.clear();
        }

        let elapsed = (Instant::now() - start_time).as_secs_f32();

        for i in 0..num_attempted {
            let problem = problems.get(i).expect("invalid problem");
            let solution = problem.get_solution().to_string();
            let answer = &answers[i];
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

        let percent = ((num_correct as f32 / num_attempted as f32) * 100.0).floor();
        println!(
            "\nYou got {} out of {} ({}%) in {:.2} seconds",
            num_correct, num_attempted, percent, elapsed
        );

        let mut continue_buf = String::new();
        println!("Play again? (Y/N)");
        io::stdin()
            .read_line(&mut continue_buf)
            .expect("Invalid input");

        match continue_buf.trim().to_lowercase().as_str() {
            "y" => {}
            _ => break,
        };
    }
}

fn generate_problems(problems: &mut Vec<Multiplication>, count: i32) -> Result<()> {
    let mut rng = rand::thread_rng();
    let uniform = Uniform::from(1..=12);

    for n in 0..count {
        let first_digit = uniform.sample(&mut rng);
        let second_digit = uniform.sample(&mut rng);
        let problem = Multiplication::new(first_digit, second_digit);

        problems.push(problem);
    }

    Ok(())
}
