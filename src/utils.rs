use anyhow::Result;
use std::{io, str::FromStr};

pub const UNICODE_CHECKMARK: &str = "\u{2714}";
pub const UNICODE_X: &str = "\u{0078}";
pub const UNICODE_DOT: &str = "\u{2022}";

pub fn read_input<T: FromStr>() -> Result<T>
where
    <T as FromStr>::Err: std::error::Error + Send + Sync + 'static,
{
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    let result = input.trim().parse::<T>()?;

    Ok(result)
}
