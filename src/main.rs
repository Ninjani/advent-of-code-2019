#![allow(dead_code)]
#[macro_use]
extern crate anyhow;
#[macro_use]
extern crate ndarray;
#[macro_use]
extern crate num_derive;
#[macro_use]
extern crate text_io;

use anyhow::Result;

mod day_1;
mod day_10;
mod day_11;
mod day_12;
mod day_13;
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
    let input = utility::input_from_file("data/day_13.input")?;
    //    println!("{}", day_13::solve_day_13_1(&input)?);
    println!("{}", day_13::solve_day_13_2(&input)?);
    Ok(())
}
