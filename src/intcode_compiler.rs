use anyhow::Error;
use num::FromPrimitive;

pub struct IntCode {
    pub program: Vec<i64>,
    pointer: usize,
    pub input_value: i64,
    pub first_input_done: bool,
    pub halted: bool,
    pub outputs: Vec<i64>,
    relative_base: i64,
}

impl IntCode {
    pub fn new(program: Vec<i64>, input_value: i64) -> Self {
        IntCode {
            program,
            pointer: 0,
            input_value,
            first_input_done: false,
            halted: false,
            outputs: Vec::new(),
            relative_base: 0,
        }
    }
}

#[derive(FromPrimitive, Copy, Clone, Debug)]
enum Mode {
    Position = 0,
    Immediate = 1,
    Relative = 2,
}

impl Default for Mode {
    fn default() -> Self {
        Mode::Position
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Parameter {
    value: i64,
    mode: Mode,
}

impl Parameter {
    fn _get_address(&self, program: &[i64]) -> Result<i64, Error> {
        if self.value < 0 {
            Err(anyhow!("negative address"))
        } else {
            let address = self.value as usize;
            if address >= program.len() {
                Ok(0)
            } else {
                Ok(program[self.value as usize])
            }
        }
    }

    fn _set_address(&self, value: i64, program: &mut Vec<i64>) -> Result<(), Error> {
        if self.value < 0 {
            Err(anyhow!("negative address"))
        } else {
            let address = self.value as usize;
            if address >= program.len() {
                program.resize(address + 1, 0);
            }
            program[address] = value;
            Ok(())
        }
    }

    fn get(&mut self, program: &[i64], relative_base: i64) -> Result<i64, Error> {
        match self.mode {
            Mode::Position => Ok(self._get_address(program)?),
            Mode::Immediate => Ok(self.value),
            Mode::Relative => {
                self.value += relative_base;
                Ok(self._get_address(program)?)
            }
        }
    }

    fn set(&mut self, value: i64, program: &mut Vec<i64>, relative_base: i64) -> Result<(), Error> {
        match self.mode {
            Mode::Position => {
                self._set_address(value, program)?;
                Ok(())
            }
            Mode::Relative => {
                self.value += relative_base;
                self._set_address(value, program)
            }
            _ => Err(anyhow!("set parameter must be in position/relative mode")),
        }
    }
}

#[derive(FromPrimitive, Copy, Clone, Debug, Eq, PartialEq)]
pub enum OpCode {
    Add = 1,
    Multiply = 2,
    Input = 3,
    Output = 4,
    JumpIfTrue = 5,
    JumpIfFalse = 6,
    LessThan = 7,
    Equals = 8,
    RelativeBaseOffset = 9,
    Halt = 99,
}

impl OpCode {
    fn num_parameters(self) -> usize {
        match self {
            OpCode::Add | OpCode::Multiply | OpCode::LessThan | OpCode::Equals => 3,
            OpCode::JumpIfTrue | OpCode::JumpIfFalse => 2,
            OpCode::Input | OpCode::Output | OpCode::RelativeBaseOffset => 1,
            OpCode::Halt => 0,
        }
    }
}

impl IntCode {
    pub fn get_last_output(&self) -> Option<i64> {
        if !self.outputs.is_empty() {
            Some(self.outputs[self.outputs.len() - 1])
        } else {
            None
        }
    }

    pub fn process(&mut self) -> Result<Option<i64>, Error> {
        while !self.step()? {}
        Ok(self.get_last_output())
    }

    fn step(&mut self) -> Result<bool, Error> {
        let (opcode, mut parameters) = self.make_instruction()?;
        Ok(self.process_instruction(opcode, &mut parameters)?)
    }

    pub fn make_instruction(&mut self) -> Result<(OpCode, Vec<Parameter>), Error> {
        if self.pointer >= self.program.len() {
            self.program.resize(self.pointer + 1, 0);
        }
        let instruction: Vec<_> = self.program[self.pointer].to_string().chars().collect();
        let opcode_length = if instruction.len() >= 2 { 2 } else { 1 };
        let opcode_int = instruction[instruction.len() - opcode_length..]
            .iter()
            .collect::<String>()
            .parse::<usize>()?;
        let opcode: OpCode = FromPrimitive::from_usize(opcode_int)
            .ok_or_else(|| anyhow!("Unknown OpCode {}", opcode_int))?;
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

    pub fn process_instruction(
        &mut self,
        opcode: OpCode,
        parameters: &mut [Parameter],
    ) -> Result<bool, Error> {
        assert_eq!(parameters.len(), opcode.num_parameters());
        match opcode {
            OpCode::Add => {
                let op1 = parameters[0].get(&self.program, self.relative_base)?;
                let op2 = parameters[1].get(&self.program, self.relative_base)?;
                parameters[2].set(
                    op1 + op2,
                    &mut self.program,
                    self.relative_base,
                )?;
                self.pointer += opcode.num_parameters() + 1;
            }
            OpCode::Multiply => {
                let op1 = parameters[0].get(&self.program, self.relative_base)?;
                let op2 = parameters[1].get(&self.program, self.relative_base)?;
                parameters[2].set(
                    op1 * op2,
                    &mut self.program,
                    self.relative_base,
                )?;
                self.pointer += opcode.num_parameters() + 1;
            }
            OpCode::Input => {
                parameters[0].set(self.input_value, &mut self.program, self.relative_base)?;
                self.pointer += opcode.num_parameters() + 1;
            }
            OpCode::Output => {
                self.outputs.push(parameters[0].get(&self.program, self.relative_base)?);
                self.pointer += opcode.num_parameters() + 1;
            }
            OpCode::JumpIfTrue => {
                if parameters[0].get(&self.program, self.relative_base)? != 0 {
                    self.pointer = parameters[1].get(&self.program, self.relative_base)? as usize;
                } else {
                    self.pointer += opcode.num_parameters() + 1;
                }
            }
            OpCode::JumpIfFalse => {
                if parameters[0].get(&self.program, self.relative_base)? == 0 {
                    self.pointer = parameters[1].get(&self.program, self.relative_base)? as usize;
                } else {
                    self.pointer += opcode.num_parameters() + 1;
                }
            }
            OpCode::LessThan => {
                if parameters[0].get(&self.program, self.relative_base)? < parameters[1].get(&self.program, self.relative_base)? {
                    parameters[2].set(1, &mut self.program, self.relative_base)?;
                } else {
                    parameters[2].set(0, &mut self.program, self.relative_base)?;
                }
                self.pointer += opcode.num_parameters() + 1;
            }
            OpCode::Equals => {
                if parameters[0].get(&self.program, self.relative_base)? == parameters[1].get(&self.program, self.relative_base)? {
                    parameters[2].set(1, &mut self.program, self.relative_base)?;
                } else {
                    parameters[2].set(0, &mut self.program, self.relative_base)?;
                }
                self.pointer += opcode.num_parameters() + 1;
            }
            OpCode::RelativeBaseOffset => {
                self.relative_base += parameters[0].get(&self.program, self.relative_base)?;
                self.pointer += opcode.num_parameters() + 1;
            }
            OpCode::Halt => {
                self.halted = true;
                return Ok(true);
            }
        }
        Ok(false)
    }
}