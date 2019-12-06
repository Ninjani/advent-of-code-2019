use anyhow::Error;
use num::FromPrimitive;

struct IntCode {
    program: Vec<isize>,
    pointer: usize,
    input_value: isize,
    outputs: Vec<isize>,
}

impl IntCode {
    fn new(program: Vec<isize>, input_value: isize) -> Self {
        IntCode {
            program,
            pointer: 0,
            input_value,
            outputs: Vec::new(),
        }
    }
}

#[derive(FromPrimitive, Copy, Clone, Debug)]
enum Mode {
    Position = 0,
    Immediate = 1,
}

impl Default for Mode {
    fn default() -> Self {
        Mode::Position
    }
}

#[derive(Debug, Copy, Clone)]
struct Parameter {
    value: isize,
    mode: Mode,
}

impl Parameter {
    fn _get_address(&self, program: &[isize]) -> Result<isize, Error> {
        if self.value < 0 {
            Err(anyhow!("negative address"))
        } else {
            Ok(program[self.value as usize])
        }
    }

    fn _set_address(&self, value: isize, program: &mut [isize]) -> Result<(), Error> {
        if self.value < 0 {
            Err(anyhow!("negative address"))
        } else {
            program[self.value as usize] = value;
            Ok(())
        }
    }

    fn get(&self, program: &[isize]) -> Result<isize, Error> {
        match self.mode {
            Mode::Position => Ok(self._get_address(program)?),
            Mode::Immediate => Ok(self.value),
        }
    }

    fn set(&self, value: isize, program: &mut [isize]) -> Result<(), Error> {
        match self.mode {
            Mode::Position => {
                self._set_address(value, program)?;
                Ok(())
            }
            _ => Err(anyhow!("set parameter must be in position mode")),
        }
    }
}

#[derive(FromPrimitive, Copy, Clone, Debug)]
enum OpCode {
    Add = 1,
    Multiply = 2,
    Input = 3,
    Output = 4,
    JumpIfTrue = 5,
    JumpIfFalse = 6,
    LessThan = 7,
    Equals = 8,
    Halt = 99,
}

impl OpCode {
    fn num_parameters(self) -> usize {
        match self {
            OpCode::Add | OpCode::Multiply | OpCode::LessThan | OpCode::Equals => 3,
            OpCode::JumpIfTrue | OpCode::JumpIfFalse => 2,
            OpCode::Input | OpCode::Output => 1,
            OpCode::Halt => 0,
        }
    }
}

impl IntCode {
    fn process(&mut self) -> Result<Option<isize>, Error> {
        while !self.step()? {}
        if !self.outputs.is_empty() {
            Ok(Some(self.outputs[self.outputs.len() - 1]))
        } else {
            Ok(None)
        }
    }

    fn step(&mut self) -> Result<bool, Error> {
        let (opcode, parameters) = self.make_instruction()?;
        Ok(self.process_instruction(opcode, &parameters)?)
    }

    fn make_instruction(&self) -> Result<(OpCode, Vec<Parameter>), Error> {
        let instruction: Vec<_> = self.program[self.pointer].to_string().chars().collect();

        let opcode_length = if instruction.len() >= 2 { 2 } else { 1 };
        let opcode: OpCode = FromPrimitive::from_usize(
            instruction[instruction.len() - opcode_length..]
                .iter()
                .collect::<String>()
                .parse::<usize>()?,
        )
            .ok_or_else(|| anyhow!("Unknown OpCode"))?;

        let mut parameters = Vec::with_capacity(opcode.num_parameters());
        for char in instruction[..instruction.len() - opcode_length]
            .iter()
            .rev()
            {
                parameters.push(Parameter {
                    value: self.program[self.pointer + parameters.len() + 1],
                    mode: FromPrimitive::from_u32(
                        char.to_digit(10)
                            .ok_or_else(|| anyhow!("char not a digit"))?,
                    )
                        .ok_or_else(|| anyhow!("Unknown Mode"))?,
                });
            }
        for _ in 0..opcode.num_parameters() - parameters.len() {
            parameters.push(Parameter {
                value: self.program[self.pointer + parameters.len() + 1],
                mode: Mode::default(),
            });
        }

        Ok((opcode, parameters))
    }

    fn process_instruction(
        &mut self,
        opcode: OpCode,
        parameters: &[Parameter],
    ) -> Result<bool, Error> {
        assert_eq!(parameters.len(), opcode.num_parameters());
        match opcode {
            OpCode::Add => {
                parameters[2].set(
                    parameters[0].get(&self.program)? + parameters[1].get(&self.program)?,
                    &mut self.program,
                )?;
                self.pointer += opcode.num_parameters() + 1;
            }
            OpCode::Multiply => {
                parameters[2].set(
                    parameters[0].get(&self.program)? * parameters[1].get(&self.program)?,
                    &mut self.program,
                )?;
                self.pointer += opcode.num_parameters() + 1;
            }
            OpCode::Input => {
                parameters[0].set(self.input_value, &mut self.program)?;
                self.pointer += opcode.num_parameters() + 1;
            }
            OpCode::Output => {
                self.outputs.push(parameters[0].get(&self.program)?);
                self.pointer += opcode.num_parameters() + 1;
            }
            OpCode::JumpIfTrue => {
                if parameters[0].get(&self.program)? != 0 {
                    self.pointer = parameters[1].get(&self.program)? as usize;
                } else {
                    self.pointer += opcode.num_parameters() + 1;
                }
            }
            OpCode::JumpIfFalse => {
                if parameters[0].get(&self.program)? == 0 {
                    self.pointer = parameters[1].get(&self.program)? as usize;
                } else {
                    self.pointer += opcode.num_parameters() + 1;
                }
            }
            OpCode::LessThan => {
                if parameters[0].get(&self.program)? < parameters[1].get(&self.program)? {
                    parameters[2].set(1, &mut self.program)?;
                } else {
                    parameters[2].set(0, &mut self.program)?;
                }
                self.pointer += opcode.num_parameters() + 1;
            }
            OpCode::Equals => {
                if parameters[0].get(&self.program)? == parameters[1].get(&self.program)? {
                    parameters[2].set(1, &mut self.program)?;
                } else {
                    parameters[2].set(0, &mut self.program)?;
                }
                self.pointer += opcode.num_parameters() + 1;
            }
            OpCode::Halt => return Ok(true),
        }
        Ok(false)
    }
}

pub fn solve_day_5_1(input: &str) -> Result<isize, Error> {
    let program = input
        .split(',')
        .map(|i| i.parse::<isize>())
        .collect::<Result<Vec<_>, _>>()?;
    let mut intcode = IntCode::new(program, 1);
    Ok(intcode.process()?.unwrap())
}

pub fn solve_day_5_2(input: &str) -> Result<isize, Error> {
    let program = input
        .split(',')
        .map(|i| i.parse::<isize>())
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
            .map(|i| i.parse::<isize>())
            .collect::<Result<Vec<_>, _>>()?;
        let mut intcode = IntCode::new(program, 1);
        intcode.process()?;
        assert_eq!(intcode.program[4], 99);

        let program = "1101,100,-1,4,0"
            .split(',')
            .map(|i| i.parse::<isize>())
            .collect::<Result<Vec<_>, _>>()?;
        let mut intcode = IntCode::new(program, 1);
        intcode.process()?;
        assert_eq!(intcode.program[4], 99);
        Ok(())
    }
}
