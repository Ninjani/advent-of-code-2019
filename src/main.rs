#![allow(dead_code)]
#[macro_use]
extern crate anyhow;
#[macro_use]
extern crate ndarray;
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
mod day_8;
mod day_9;
pub mod intcode_compiler;
pub mod utility;

fn main() -> Result<()> {
    let input = utility::input_from_file("data/day_9.input")?;
    println!("{}", day_9::solve_day_9_1(&input)?);
    println!("{}", day_9::solve_day_9_2(&input)?);
    Ok(())
}
