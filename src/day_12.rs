use std::cmp::Ordering;

use anyhow::Error;
use num::integer::lcm;

type Vec3D = [i64; 3];

#[derive(Clone, Debug, Eq, PartialEq)]
struct Moon {
    position: Vec3D,
    velocity: Vec3D,
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct Moons {
    moons: Vec<Moon>,
    length: usize,
    initial_positions: Vec<Vec3D>,
}

impl Moon {
    fn new(position: Vec3D) -> Moon {
        Moon {
            position,
            velocity: [0, 0, 0],
        }
    }

    fn update_position_i(&mut self, i: usize) {
        self.position[i] += self.velocity[i];
    }

    fn get_potential_energy(&self) -> i64 {
        self.position.iter().map(|p| p.abs()).sum()
    }

    fn get_kinetic_energy(&self) -> i64 {
        self.velocity.iter().map(|p| p.abs()).sum()
    }

    fn get_energy(&self) -> i64 {
        self.get_potential_energy() * self.get_kinetic_energy()
    }
}

impl Moons {
    fn new(input: &str) -> Result<Self, Error> {
        fn get_value(part: &str) -> Result<i64, Error> {
            Ok(part.split('=').last().unwrap().parse::<i64>()?)
        }
        let mut moons = Vec::new();
        for line in input.split('\n') {
            let parts: Vec<_> = line.split(", ").collect();
            moons.push(Moon::new([
                get_value(parts[0])?,
                get_value(parts[1])?,
                get_value(&parts[2][..parts[2].len() - 1])?,
            ]));
        }
        let length = moons.len();
        let initial_positions = moons.iter().map(|m| m.position).collect();
        Ok(Moons {
            moons,
            length,
            initial_positions,
        })
    }

    fn get_total_energy(&self) -> i64 {
        self.moons.iter().map(|moon| moon.get_energy()).sum()
    }

    fn change_velocity_i(&mut self, index_1: usize, index_2: usize, i: usize) {
        match self.moons[index_1].position[i].cmp(&self.moons[index_2].position[i]) {
            Ordering::Greater => {
                self.moons[index_1].velocity[i] -= 1;
                self.moons[index_2].velocity[i] += 1;
            }
            Ordering::Less => {
                self.moons[index_1].velocity[i] += 1;
                self.moons[index_2].velocity[i] -= 1;
            }
            _ => (),
        }
    }

    fn time_step_i(&mut self, i: usize) {
        for index_1 in 0..self.length {
            for index_2 in index_1 + 1..self.length {
                self.change_velocity_i(index_1, index_2, i);
            }
            self.moons[index_1].update_position_i(i);
        }
    }

    fn time_step(&mut self) {
        for index_1 in 0..self.length {
            for index_2 in index_1 + 1..self.length {
                for i in 0..3 {
                    self.change_velocity_i(index_1, index_2, i);
                }
            }
            for i in 0..3 {
                self.moons[index_1].update_position_i(i);
            }
        }
    }

    fn find_period_i(&mut self, i: usize) -> usize {
        let mut step = 0;
        loop {
            self.time_step_i(i);
            step += 1;
            let back_to_first = self.moons.iter().enumerate().all(|(m, moon)| {
                moon.position[i] == self.initial_positions[m][i] && moon.velocity[i] == 0
            });
            if back_to_first {
                return step;
            }
        }
    }

    fn find_period(&mut self) -> usize {
        let (x, y, z) = (
            self.find_period_i(0),
            self.find_period_i(1),
            self.find_period_i(2),
        );
        lcm(x, lcm(y, z))
    }
}

pub fn solve_day_12_1(input: &str) -> Result<i64, Error> {
    let mut moons = Moons::new(input)?;
    for _ in 0..1000 {
        moons.time_step();
    }
    Ok(moons.get_total_energy())
}

pub fn solve_day_12_2(input: &str) -> Result<usize, Error> {
    let mut moons = Moons::new(input)?;
    Ok(moons.find_period())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() -> Result<(), Error> {
        let mut moons =
            Moons::new("<x=-1, y=0, z=2>\n<x=2, y=-10, z=-7>\n<x=4, y=-8, z=8>\n<x=3, y=5, z=-1>")?;
        // 5 steps
        for _ in 0..5 {
            moons.time_step();
        }
        assert_eq!(moons.moons[0].position, [-1, -9, 2]);
        assert_eq!(moons.moons[0].velocity, [-3, -1, 2]);
        // 10 steps
        for _ in 0..5 {
            moons.time_step();
        }
        assert_eq!(moons.moons[0].position, [2, 1, -3]);
        assert_eq!(moons.moons[0].velocity, [-3, -2, 1]);
        assert_eq!(179, moons.get_total_energy());

        let mut moons = Moons::new(
            "<x=-8, y=-10, z=0>\n<x=5, y=5, z=10>\n<x=2, y=-7, z=3>\n<x=9, y=-8, z=-3>",
        )?;
        for _ in 0..100 {
            moons.time_step();
        }
        assert_eq!(1940, moons.get_total_energy());
        Ok(())
    }

    #[test]
    fn test_2() -> Result<(), Error> {
        let mut moons =
            Moons::new("<x=-1, y=0, z=2>\n<x=2, y=-10, z=-7>\n<x=4, y=-8, z=8>\n<x=3, y=5, z=-1>")?;
        assert_eq!(2772, moons.find_period());

        let mut moons = Moons::new(
            "<x=-8, y=-10, z=0>\n<x=5, y=5, z=10>\n<x=2, y=-7, z=3>\n<x=9, y=-8, z=-3>",
        )?;
        assert_eq!(4686774924, moons.find_period());
        Ok(())
    }
}
