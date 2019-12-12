use std::collections::HashMap;

use anyhow::Error;
use ndarray::Array2;
use num::{FromPrimitive, ToPrimitive};

use crate::intcode_compiler::IntCode;
use crate::utility::plot;

type Point = (i64, i64);

#[derive(Copy, Clone, Debug)]
enum Direction {
    Up,
    Right,
    Left,
    Down,
}

#[derive(FromPrimitive, Copy, Clone, Debug, Eq, PartialEq)]
enum Turn {
    Left = 0,
    Right = 1,
}

#[derive(FromPrimitive, ToPrimitive, Copy, Clone, Debug, Eq, PartialEq)]
enum Color {
    Black = 0,
    White = 1,
}

impl Default for Color {
    fn default() -> Self {
        Color::Black
    }
}

impl Direction {
    fn step(self, location: &mut Point) {
        match self {
            Direction::Up => location.1 += 1,
            Direction::Right => location.0 += 1,
            Direction::Left => location.0 -= 1,
            Direction::Down => location.1 -= 1,
        }
    }

    fn turn(self, turn: Turn) -> Self {
        match turn {
            Turn::Left => match self {
                Direction::Up => Direction::Left,
                Direction::Right => Direction::Up,
                Direction::Left => Direction::Down,
                Direction::Down => Direction::Right,
            },
            Turn::Right => match self {
                Direction::Up => Direction::Right,
                Direction::Right => Direction::Down,
                Direction::Left => Direction::Up,
                Direction::Down => Direction::Left,
            },
        }
    }
}

struct PaintingRobot {
    program: IntCode,
    grid: HashMap<Point, Color>,
    direction: Direction,
    location: Point,
}

impl PaintingRobot {
    fn new(input: &str, start_color: Color) -> Result<Self, Error> {
        let mut grid = HashMap::new();
        grid.insert((0, 0), start_color);
        Ok(PaintingRobot {
            program: IntCode::new(
                input
                    .split(',')
                    .map(|i| i.parse::<i64>())
                    .collect::<Result<Vec<_>, _>>()?,
                0,
            ),
            grid,
            direction: Direction::Up,
            location: (0, 0),
        })
    }

    fn run(&mut self) -> Result<(), Error> {
        while !self.program.halted {
            self.paint()?;
        }
        Ok(())
    }

    fn paint(&mut self) -> Result<(), Error> {
        self.program.input_value = self.get_color_at(&self.location).to_i64().unwrap();
        loop {
            let (opcode, mut parameters) = self.program.make_instruction()?;
            if self.program.process_instruction(opcode, &mut parameters)? {
                break;
            }
            if self.program.outputs.len() == 2 {
                let color = FromPrimitive::from_i64(self.program.outputs[0]).unwrap();
                self.set_color_at(self.location, color);
                let turn = FromPrimitive::from_i64(self.program.outputs[1]).unwrap();
                self.direction = self.direction.turn(turn);
                self.direction.step(&mut self.location);
                self.program.outputs = Vec::new();
                break;
            }
        }
        Ok(())
    }

    fn get_color_at(&self, location: &Point) -> Color {
        *self.grid.get(location).unwrap_or(&Color::default())
    }

    fn set_color_at(&mut self, location: Point, color: Color) {
        self.grid.insert(location, color);
    }
}

pub fn solve_day_11_1(input: &str) -> Result<usize, Error> {
    let mut robot = PaintingRobot::new(input, Color::Black)?;
    robot.run()?;
    Ok(robot.grid.len())
}

pub fn solve_day_11_2(input: &str) -> Result<String, Error> {
    let mut robot = PaintingRobot::new(input, Color::White)?;
    robot.run()?;
    let (mut min_x, mut min_y, mut max_x, mut max_y) = (
        ::std::i64::MAX,
        ::std::i64::MAX,
        ::std::i64::MIN,
        ::std::i64::MIN,
    );
    for location in robot.grid.keys() {
        if location.0 < min_x {
            min_x = location.0;
        }
        if location.0 > max_x {
            max_x = location.0;
        }
        if location.1 < min_y {
            min_y = location.1;
        }
        if location.1 > max_y {
            max_y = location.1;
        }
    }
    let num_x = (max_x - min_x + 1) as usize;
    let num_y = (max_y - min_y + 1) as usize;
    let x_to_index: HashMap<i64, usize> = (min_x..=max_x).zip(0..num_x).collect();
    let y_to_index: HashMap<i64, usize> = (min_y..=max_y).zip(0..num_y).collect();
    let mut grid = Array2::zeros((num_x, num_y));
    for (location, color) in robot.grid {
        grid[(x_to_index[&location.0], y_to_index[&location.1])] = color.to_u32().unwrap();
    }
    Ok(plot(&grid))
}
