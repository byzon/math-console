use log::error;
use std::fmt::{Display, Formatter, Result};

pub struct AppError;

impl Display for AppError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "App error")
    }
}

pub fn handle_error(e: anyhow::Error, message: &str) -> anyhow::Error {
    error!("Error: {message}: {e}");
    e
}
