#![allow(dead_code)]
pub mod utility;
mod day_1;
use anyhow::Result;

fn main() -> Result<()>{
    print!("{}", day_1::solve_day_1_2()?);
    Ok(())
}
