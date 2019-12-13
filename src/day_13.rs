use std::collections::HashMap;

use anyhow::Error;
use ndarray::Array2;
use num::{FromPrimitive, ToPrimitive};
use text_io::read;

use crate::intcode_compiler::{IntCode, OpCode};
use crate::utility::{input_from_file, write_to_file};

// 0 is an empty tile. No game object appears in this tile.
// 1 is a wall tile. Walls are indestructible barriers.
// 2 is a block tile. Blocks can be broken by the ball.
// 3 is a horizontal paddle tile. The paddle is indestructible.
// 4 is a ball tile. The ball moves diagonally and bounces off objects.
#[derive(FromPrimitive, ToPrimitive, Copy, Clone, Eq, PartialEq, Debug)]
enum TileID {
    Empty = 0,
    Wall = 1,
    Block = 2,
    Paddle = 3,
    Ball = 4,
}

impl TileID {
    fn to_char(self) -> char {
        match self {
            TileID::Empty => ' ',
            TileID::Wall => '█',
            TileID::Block => 'X',
            TileID::Paddle => '-',
            TileID::Ball => 'o',
        }
    }

    fn plot(grid: &Array2<u32>) -> String {
        (0..grid.shape()[0])
            .flat_map(|i| {
                (0..grid.shape()[1])
                    .map(move |j| {
                        let tile_id: TileID = FromPrimitive::from_u32(grid[(i, j)]).unwrap();
                        tile_id.to_char()
                    })
                    .chain(vec!['\n'].into_iter())
            })
            .collect()
    }
}

struct Tile {
    x: i64,
    y: i64,
    id: TileID,
}

impl Tile {
    fn new(outputs: &[i64]) -> Self {
        Tile {
            x: outputs[0],
            y: outputs[1],
            id: FromPrimitive::from_i64(outputs[2]).unwrap(),
        }
    }
}

#[derive(FromPrimitive, ToPrimitive, Debug, Copy, Clone, PartialEq, Eq)]
enum Joystick {
    Neutral = 0,
    Left = -1,
    Right = 1,
}

impl Default for Joystick {
    fn default() -> Self {
        Joystick::Neutral
    }
}

struct ArcadeCabinet {
    program: IntCode,
    tiles: Vec<Tile>,
    joystick: Joystick,
    joysticks: Vec<Joystick>,
    input_index: usize,
    score: i64,
}

impl ArcadeCabinet {
    fn new(input: Vec<i64>) -> Result<Self, Error> {
        Ok(ArcadeCabinet {
            program: IntCode::new(input, 0),
            tiles: Vec::new(),
            score: 0,
            joystick: Joystick::default(),
            input_index: 0,
            joysticks: Vec::new(),
        })
    }

    fn run(&mut self) -> Result<(), Error> {
        while !self.program.halted {
            self.step()?;
        }
        Ok(())
    }

    fn read_input(&mut self) {
        if self.input_index < self.joysticks.len() {
            self.joystick = self.joysticks[self.input_index];
            self.program.input_value = self.joysticks[self.input_index].to_i64().unwrap_or(0);
        } else {
            println!("Move joystick:");
            let i: String = read!("{}\n");
            let i = match i.chars().next() {
                Some('A') | Some('a') => -1,
                Some('D') | Some('d') => 1,
                Some('S') | Some('s') => 0,
                _ => 2,
            };
            if let Some(joystick) = FromPrimitive::from_i64(i) {
                self.joystick = joystick;
                self.program.input_value = i;
            }
            self.joysticks.push(self.joystick)
        }
        self.input_index += 1;
    }

    fn step(&mut self) -> Result<(), Error> {
        loop {
            let (opcode, mut parameters) = self.program.make_instruction()?;
            if opcode == OpCode::Input {
                self.read_input();
            }
            if self.program.process_instruction(opcode, &mut parameters)? {
                break;
            }
            if self.program.outputs.len() == 3 {
                if self.program.outputs[0] == -1 && self.program.outputs[1] == 0 {
                    self.score = self.program.outputs[2];
                } else {
                    self.tiles.push(Tile::new(&self.program.outputs));
                }
                self.program.outputs = Vec::new();
                println!("SCORE: {}", self.score);
                println!("Joystick: {:?}", self.joystick);
                println!("{}", TileID::plot(&self.make_grid()));
                println!();
                break;
            }
        }
        Ok(())
    }

    fn make_grid(&self) -> Array2<u32> {
        let (mut min_x, mut min_y, mut max_x, mut max_y) = (
            ::std::i64::MAX,
            ::std::i64::MAX,
            ::std::i64::MIN,
            ::std::i64::MIN,
        );
        for tile in self.tiles.iter() {
            if tile.x < min_x {
                min_x = tile.x;
            }
            if tile.x > max_x {
                max_x = tile.x;
            }
            if tile.y < min_y {
                min_y = tile.y;
            }
            if tile.y > max_y {
                max_y = tile.y;
            }
        }
        let num_x = (max_x - min_x + 1) as usize;
        let num_y = (max_y - min_y + 1) as usize;
        let x_to_index: HashMap<i64, usize> = (min_x..=max_x).zip(0..num_x).collect();
        let y_to_index: HashMap<i64, usize> = (min_y..=max_y).zip(0..num_y).collect();
        let mut grid = Array2::zeros((num_y, num_x));
        for tile in self.tiles.iter() {
            grid[(y_to_index[&tile.y], x_to_index[&tile.x])] = tile.id.to_u32().unwrap();
        }
        grid
    }
}

pub fn solve_day_13_1(input: &str) -> Result<usize, Error> {
    let mut arcade = ArcadeCabinet::new(
        input
            .split(',')
            .map(|i| i.parse::<i64>())
            .collect::<Result<Vec<_>, _>>()?,
    )?;
    arcade.run()?;
    Ok(arcade
        .tiles
        .iter()
        .filter(|tile| tile.id == TileID::Block)
        .count())
}

fn play(arcade: &mut ArcadeCabinet, read: bool, write: bool) -> Result<i64, Error> {
    if read {
        let joysticks = input_from_file("data/day_13.output")?
            .split(",")
            .map(|i| i.parse::<i64>())
            .collect::<Result<Vec<_>, _>>()?;
        arcade.joysticks = joysticks
            .into_iter()
            .map(|i| FromPrimitive::from_i64(i).unwrap_or(Joystick::default()))
            .collect();
    }
    arcade.run()?;
    if write {
        write_to_file(
            "data/day_13.output",
            &arcade
                .joysticks
                .iter()
                .map(|j| j.to_i64().unwrap().to_string())
                .collect::<Vec<_>>()
                .join(","),
        )?;
    }
    Ok(arcade.score)
}

pub fn solve_day_13_2(input: &str) -> Result<i64, Error> {
    let mut program = input
        .split(',')
        .map(|i| i.parse::<i64>())
        .collect::<Result<Vec<_>, _>>()?;
    program[0] = 2;
    let mut arcade = ArcadeCabinet::new(program)?;
    Ok(play(&mut arcade, true, true)?)
}
