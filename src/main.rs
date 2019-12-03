#![allow(dead_code)]
#[macro_use]
extern crate anyhow;

use anyhow::Result;

mod day_1;
mod day_2;
mod day_3;
pub mod utility;

fn main() -> Result<()> {
    let input = utility::input_from_file("data/day_3.input")?;
    println!("{}", day_3::solve_day_3_1(&input)?);
    println!("{}", day_3::solve_day_3_2(&input)?);
    Ok(())
}
