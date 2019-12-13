use anyhow::Error;

use crate::intcode_compiler::IntCode;

pub fn solve_day_9_1(input: &str) -> Result<i64, Error> {
    let program = input
        .split(',')
        .map(|i| i.parse::<i64>())
        .collect::<Result<Vec<_>, _>>()?;
    let mut intcode = IntCode::new(program, 1);
    Ok(intcode.process()?.unwrap())
}

pub fn solve_day_9_2(input: &str) -> Result<i64, Error> {
    let program = input
        .split(',')
        .map(|i| i.parse::<i64>())
        .collect::<Result<Vec<_>, _>>()?;
    let mut intcode = IntCode::new(program, 2);
    Ok(intcode.process()?.unwrap())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() -> Result<(), Error> {
        let program = "109,1,204,-1,1001,100,1,100,1008,100,16,101,1006,101,0,99"
            .split(',')
            .map(|i| i.parse::<i64>())
            .collect::<Result<Vec<_>, _>>()?;
        let mut intcode = IntCode::new(program.clone(), 1);
        intcode.process()?;
        assert_eq!(intcode.outputs, program);

        let program = "1102,34915192,34915192,7,4,7,99,0"
            .split(',')
            .map(|i| i.parse::<i64>())
            .collect::<Result<Vec<_>, _>>()?;
        let mut intcode = IntCode::new(program, 1);
        intcode.process()?;
        assert_eq!(
            intcode
                .get_last_output()
                .unwrap()
                .to_string()
                .chars()
                .count(),
            16
        );

        let program = "104,1125899906842624,99"
            .split(',')
            .map(|i| i.parse::<i64>())
            .collect::<Result<Vec<_>, _>>()?;
        let mut intcode = IntCode::new(program, 1);
        intcode.process()?;
        assert_eq!(Some(1125899906842624), intcode.get_last_output());
        Ok(())
    }
}
