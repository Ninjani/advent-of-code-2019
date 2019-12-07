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
mod day_7;
pub mod utility;

fn main() -> Result<()> {
    let input = utility::input_from_file("data/day_7.input")?;
    println!("{}", day_7::solve_day_7_1(&input)?);
    println!("{}", day_7::solve_day_7_2(&input)?);
    Ok(())
}
