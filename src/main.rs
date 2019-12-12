#![allow(dead_code)]
#[macro_use]
extern crate anyhow;
#[macro_use]
extern crate ndarray;
#[macro_use]
extern crate num_derive;

use anyhow::Result;

mod day_1;
mod day_10;
mod day_2;
mod day_3;
mod day_4;
mod day_5;
mod day_6;
mod day_7;
mod day_8;
mod day_9;
mod day_11;
mod day_12;
pub mod intcode_compiler;
pub mod utility;

fn main() -> Result<()> {
    let input = utility::input_from_file("data/day_12.input")?;
    println!("{}", day_12::solve_day_12_1(&input)?);
    println!("{}", day_12::solve_day_12_2(&input)?);
    Ok(())
}
