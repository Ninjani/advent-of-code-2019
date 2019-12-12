use std::cmp::Ordering;

use anyhow::Error;
use num::integer::lcm;

type Vec3D = [i64; 3];

#[derive(Clone, Debug, Eq, PartialEq)]
struct Moon {
    position: Vec3D,
    velocity: Vec3D,
}

impl Moon {
    fn new(position: Vec3D) -> Moon {
        Moon {
            position,
            velocity: [0, 0, 0],
        }
    }

    fn update_position(&mut self) {
        for i in 0..3 {
            self.position[i] += self.velocity[i];
        }
    }

    fn get_potential_energy(&self) -> i64 {
        (0..3).map(|i| self.position[i].abs()).sum()
    }

    fn get_kinetic_energy(&self) -> i64 {
        (0..3).map(|i| self.velocity[i].abs()).sum()
    }

    fn get_energy(&self) -> i64 {
        self.get_potential_energy() * self.get_kinetic_energy()
    }
}

fn parse_input(input: &str) -> Result<Vec<Moon>, Error> {
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
    Ok(moons)
}

fn get_total_energy(moons: &[Moon]) -> i64 {
    moons.iter().map(|moon| moon.get_energy()).sum::<i64>()
}

fn change_velocity(moons: &mut [Moon], index_1: usize, index_2: usize) {
    for i in 0..3 {
        match moons[index_1].position[i].cmp(&moons[index_2].position[i]) {
            Ordering::Greater => {
                moons[index_1].velocity[i] -= 1;
                moons[index_2].velocity[i] += 1;
            }
            Ordering::Less => {
                moons[index_1].velocity[i] += 1;
                moons[index_2].velocity[i] -= 1;
            }
            _ => ()
        }
    }
}

fn simulate_moons(moons: &mut [Moon], num_steps: usize) {
    for _ in 0..num_steps {
        for index_1 in 0..moons.len() {
            for index_2 in index_1 + 1..moons.len() {
                change_velocity(moons, index_1, index_2);
            }
            moons[index_1].update_position();
        }
    }
}

fn find_simulation_period(moons: &mut [Moon]) -> Option<usize> {
    let mut periods = [None, None, None];
    let initial_positions: Vec<_> = moons.iter().map(|m| m.position).collect();
    let mut step = 0;
    while periods.iter().any(|p| p.is_none()) {
        simulate_moons(moons, 1);
        step += 1;
        for (i, p) in periods.iter_mut().enumerate().filter(|(_, p)| p.is_none()) {
            let back_to_first = moons.iter().enumerate().all(|(m, moon)| {
                moon.position[i] == initial_positions[m][i] && moon.velocity[i] == 0
            });
            if back_to_first {
                *p = Some(step);
            }
        }
    }
    match periods {
        [Some(x), Some(y), Some(z)] => Some(lcm(x, lcm(y, z))),
        _ => None,
    }
}

pub fn solve_day_12_1(input: &str) -> Result<i64, Error> {
    let mut moons = parse_input(input)?;
    simulate_moons(&mut moons, 1000);
    Ok(get_total_energy(&moons))
}

pub fn solve_day_12_2(input: &str) -> Result<usize, Error> {
    let mut moons = parse_input(input)?;
    Ok(find_simulation_period(&mut moons).unwrap())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() -> Result<(), Error> {
        let mut moons = parse_input(
            "<x=-1, y=0, z=2>\n<x=2, y=-10, z=-7>\n<x=4, y=-8, z=8>\n<x=3, y=5, z=-1>",
        )?;
        // 5 steps
        simulate_moons(&mut moons, 5);
        assert_eq!(moons[0].position, [-1, -9, 2]);
        assert_eq!(moons[0].velocity, [-3, -1, 2]);
        // 10 steps
        simulate_moons(&mut moons, 5);
        assert_eq!(moons[0].position, [2, 1, -3]);
        assert_eq!(moons[0].velocity, [-3, -2, 1]);
        assert_eq!(179, get_total_energy(&moons));

        let mut moons = parse_input(
            "<x=-8, y=-10, z=0>\n<x=5, y=5, z=10>\n<x=2, y=-7, z=3>\n<x=9, y=-8, z=-3>",
        )?;
        simulate_moons(&mut moons, 100);
        assert_eq!(1940, get_total_energy(&moons));
        Ok(())
    }

    #[test]
    fn test_2() -> Result<(), Error> {
        let mut moons = parse_input(
            "<x=-1, y=0, z=2>\n<x=2, y=-10, z=-7>\n<x=4, y=-8, z=8>\n<x=3, y=5, z=-1>",
        )?;
        assert_eq!(2772, find_simulation_period(&mut moons).unwrap());

        let mut moons = parse_input(
            "<x=-8, y=-10, z=0>\n<x=5, y=5, z=10>\n<x=2, y=-7, z=3>\n<x=9, y=-8, z=-3>",
        )?;
        assert_eq!(4686774924, find_simulation_period(&mut moons).unwrap());
        Ok(())
    }
}
