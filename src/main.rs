use std::error;
use std::fmt;
use std::io;

struct AppError;

type Result<T> = std::result::Result<T, Box<dyn error::Error>>;

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
        format!("{} * {} = ", self.first_digit, self.second_digit)
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
    println!("Hello, world!");

    let mut problems = vec![];
    let _ = generate_problems(&mut problems).map_err(|e| {
        println!("Error generating problems: {e:?}");
    });

    let mut buf = String::new();
    for problem in problems.clone() {
        let solution = problem.get_solution().to_string();

        println!("{}", problem.get_question());

        io::stdin()
            .read_line(&mut buf)
            .expect("Failed to read line.");

        let input = buf.trim().to_lowercase();

        match input.as_str() {
            "exit" => break,
            _ => {
                if solution == input {
                    println!("SOLVED IT!");
                } else {
                    println!("INCORRECT!");
                }
            }
        }

        buf.clear();
    }
}

fn generate_problems(problems: &mut Vec<Multiplication>) -> Result<()> {
    let mut new_problems = vec![
        Multiplication::new(2, 3),
        Multiplication::new(4, 2),
        Multiplication::new(8, 7),
    ];

    problems.append(&mut new_problems);

    Ok(())
}
