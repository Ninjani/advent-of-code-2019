#![allow(dead_code)]
#[macro_use]
extern crate anyhow;
#[macro_use]
extern crate num_derive;

use anyhow::Result;

mod day_1;
mod day_2;
mod day_3;
mod day_4;
mod day_5;
mod day_6;
pub mod utility;

fn main() -> Result<()> {
    let input = utility::input_from_file("data/day_6.input")?;
    println!("{}", day_6::solve_day_6_1(&input)?);
    println!("{}", day_6::solve_day_6_2(&input)?);
    Ok(())
}
