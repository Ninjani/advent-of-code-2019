#![allow(dead_code)]
#[macro_use]
extern crate anyhow;

use anyhow::Result;

mod day_1;
mod day_2;
pub mod utility;

fn main() -> Result<()> {
    let input = utility::input_from_file("data/day_2.input")?;
    println!("{}", day_2::solve_day_2_1(&input)?);
    println!("{}", day_2::solve_day_2_2(&input)?);
    Ok(())
}
