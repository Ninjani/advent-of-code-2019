use anyhow::Error;
use itertools::Itertools;

use crate::day_5::{IntCode, OpCode};

impl IntCode {
    fn process_with_two_inputs(&mut self, new_input: isize) -> Result<Option<isize>, Error> {
        loop {
            let (opcode, parameters) = self.make_instruction()?;
            if self.process_instruction(opcode, &parameters)? {
                break;
            }
            if !self.first_input_done && opcode == OpCode::Input {
                self.input_value = new_input;
                self.first_input_done = true;
            }
        }
        Ok(self.get_diagnostic())
    }

    fn process_till_next_output(&mut self, new_input: isize) -> Result<Option<isize>, Error> {
        loop {
            let (opcode, parameters) = self.make_instruction()?;
            if self.first_input_done && opcode == OpCode::Input {
                self.input_value = new_input;
            }
            self.process_instruction(opcode, &parameters)?;
            if !self.first_input_done && opcode == OpCode::Input {
                self.input_value = new_input;
                self.first_input_done = true;
            }

            if opcode == OpCode::Output || opcode == OpCode::Halt {
                return Ok(self.get_diagnostic());
            }
        }
    }
}

fn get_thrust(program: &[isize], phase_sequence: &[isize]) -> Result<isize, Error> {
    let mut second_input = 0;
    let amplifiers: Vec<_> = (0..5)
        .map(|i| IntCode::new(program.to_vec(), phase_sequence[i]))
        .collect();
    for mut amplifier in amplifiers {
        second_input = amplifier.process_with_two_inputs(second_input)?.unwrap();
    }
    Ok(second_input)
}

pub fn get_thrust_feedback(program: &[isize], phase_sequence: &[isize]) -> Result<isize, Error> {
    let mut new_input = 0;
    let mut amplifiers: Vec<_> = (0..5)
        .map(|i| IntCode::new(program.to_vec(), phase_sequence[i]))
        .collect();
    let mut index = 0;
    loop {
        new_input = amplifiers[index]
            .process_till_next_output(new_input)?
            .unwrap();
        if amplifiers.iter().all(|a| a.halted) {
            return Ok(new_input);
        }
        index += 1;
        index %= 5;
    }
}

pub fn solve_day_7_1(input: &str) -> Result<isize, Error> {
    let program = input
        .split(',')
        .map(|i| i.parse::<isize>())
        .collect::<Result<Vec<_>, _>>()?;
    let mut output = ::std::isize::MIN;
    for phase_sequence in (0..5).permutations(5) {
        let p_output = get_thrust(&program, &phase_sequence)?;
        if p_output > output {
            output = p_output
        }
    }
    Ok(output)
}

pub fn solve_day_7_2(input: &str) -> Result<isize, Error> {
    let program = input
        .split(',')
        .map(|i| i.parse::<isize>())
        .collect::<Result<Vec<_>, _>>()?;
    let mut output = ::std::isize::MIN;
    for phase_sequence in (5..10).permutations(5) {
        let p_output = get_thrust_feedback(&program, &phase_sequence)?;
        if p_output > output {
            output = p_output
        }
    }
    Ok(output)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() -> Result<(), Error> {
        let program = "3,15,3,16,1002,16,10,16,1,16,15,15,4,15,99,0,0"
            .split(',')
            .map(|i| i.parse::<isize>())
            .collect::<Result<Vec<_>, _>>()?;
        assert_eq!(43210, get_thrust(&program, &[4, 3, 2, 1, 0])?);
        let program = "3,23,3,24,1002,24,10,24,1002,23,-1,23,101,5,23,23,1,24,23,23,4,23,99,0,0"
            .split(',')
            .map(|i| i.parse::<isize>())
            .collect::<Result<Vec<_>, _>>()?;
        assert_eq!(54321, get_thrust(&program, &[0, 1, 2, 3, 4])?);
        let program = "3,31,3,32,1002,32,10,32,1001,31,-2,31,1007,31,0,33,1002,33,7,33,1,33,31,31,1,32,31,31,4,31,99,0,0,0".split(',')
            .map(|i| i.parse::<isize>())
            .collect::<Result<Vec<_>, _>>()?;
        assert_eq!(65210, get_thrust(&program, &[1, 0, 4, 3, 2])?);

        Ok(())
    }

    #[test]
    fn test_2() -> Result<(), Error> {
        let program =
            "3,26,1001,26,-4,26,3,27,1002,27,2,27,1,27,26,27,4,27,1001,28,-1,28,1005,28,6,99,0,0,5"
                .split(',')
                .map(|i| i.parse::<isize>())
                .collect::<Result<Vec<_>, _>>()?;
        assert_eq!(139629729, get_thrust_feedback(&program, &[9, 8, 7, 6, 5])?);
        let program = "3,52,1001,52,-5,52,3,53,1,52,56,54,1007,54,5,55,1005,55,26,1001,54,-5,54,1105,1,12,1,53,54,53,1008,54,0,55,1001,55,1,55,2,53,55,53,4,53,1001,56,-1,56,1005,56,6,99,0,0,0,0,10".split(',')
            .map(|i| i.parse::<isize>())
            .collect::<Result<Vec<_>, _>>()?;
        assert_eq!(18216, get_thrust_feedback(&program, &[9, 7, 8, 5, 6])?);
        Ok(())
    }
}
