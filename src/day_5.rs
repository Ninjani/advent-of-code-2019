use anyhow::Error;

use crate::intcode_compiler::IntCode;

pub fn solve_day_5_1(input: &str) -> Result<i64, Error> {
    let program = input
        .split(',')
        .map(|i| i.parse::<i64>())
        .collect::<Result<Vec<_>, _>>()?;
    let mut intcode = IntCode::new(program, 1);
    Ok(intcode.process()?.unwrap())
}

pub fn solve_day_5_2(input: &str) -> Result<i64, Error> {
    let program = input
        .split(',')
        .map(|i| i.parse::<i64>())
        .collect::<Result<Vec<_>, _>>()?;
    let mut intcode = IntCode::new(program, 5);
    Ok(intcode.process()?.unwrap())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() -> Result<(), Error> {
        let program = "1002,4,3,4,33"
            .split(',')
            .map(|i| i.parse::<i64>())
            .collect::<Result<Vec<_>, _>>()?;
        let mut intcode = IntCode::new(program, 1);
        intcode.process()?;
        assert_eq!(intcode.program[4], 99);

        let program = "1101,100,-1,4,0"
            .split(',')
            .map(|i| i.parse::<i64>())
            .collect::<Result<Vec<_>, _>>()?;
        let mut intcode = IntCode::new(program, 1);
        intcode.process()?;
        assert_eq!(intcode.program[4], 99);
        Ok(())
    }
}
