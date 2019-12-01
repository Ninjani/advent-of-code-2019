#![allow(dead_code)]

use anyhow::Result;

mod day_1;
pub mod utility;

fn main() -> Result<()> {
    let input = utility::input_from_file("data/day_1.input")?;
    println!("{}", day_1::solve_day_1_1(&input)?);
    println!("{}", day_1::solve_day_1_2(&input)?);
    Ok(())
}
