mod basic;

pub use basic::*;

use anyhow::Result;

pub trait Game {
    fn start(&mut self) -> Result<()>;
    fn prepare(&mut self) -> Result<()>;
    fn play(&mut self) -> Result<()>;
    fn end(&mut self) -> Result<()>;
}
