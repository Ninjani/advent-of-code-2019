use std::collections::{HashMap, HashSet};
use std::hash::Hash;

use anyhow::Error;
use itertools::Itertools;
use num_rational::Ratio;

type Asteroid = (i64, i64);
type Line = (Ratio<i64>, Ratio<i64>);

#[derive(Hash, Debug, PartialEq, Eq, Copy, Clone, PartialOrd, Ord)]
enum Quadrant {
    TopRight,
    BottomRight,
    BottomLeft,
    TopLeft,
}

impl Quadrant {
    fn get_direction(center: &Asteroid, to: &Asteroid) -> Quadrant {
        if center.0 <= to.0 {
            if center.1 <= to.1 {
                Quadrant::BottomRight
            } else {
                Quadrant::TopRight
            }
        } else if center.1 <= to.1 {
            Quadrant::BottomLeft
        } else {
            Quadrant::TopLeft
        }
    }
}

fn parse_asteroid_map(input: &str) -> Result<Vec<Asteroid>, Error> {
    Ok(input
        .split('\n')
        .enumerate()
        .flat_map(|(j, line)| {
            line.chars()
                .enumerate()
                .filter(|(_, character)| *character == '#')
                .map(move |(i, _)| (i as i64, j as i64))
        })
        .collect())
}

fn get_line(point_1: &Asteroid, point_2: &Asteroid, quadrant: Quadrant) -> Line {
    if point_1.0 == point_2.0 {
        let slope = match quadrant {
            Quadrant::TopRight => Ratio::from_integer(::std::i64::MAX),
            Quadrant::BottomRight => Ratio::from_integer(::std::i64::MIN),
            Quadrant::BottomLeft => Ratio::from_integer(::std::i64::MAX),
            Quadrant::TopLeft => Ratio::from_integer(::std::i64::MIN),
        };
        (slope, Ratio::from_integer(point_1.0))
    } else {
        let slope = Ratio::new(point_1.1 - point_2.1, point_2.0 - point_1.0);
        (slope, Ratio::from_integer(point_2.1) + slope * point_2.0)
    }
}

fn put_asteroids_on_lines(
    index: usize,
    asteroids: &[Asteroid],
) -> HashMap<(Quadrant, Line), HashSet<Asteroid>> {
    let mut line_to_asteroids = HashMap::new();
    for i in 0..asteroids.len() {
        if i != index {
            let quadrant = Quadrant::get_direction(&asteroids[index], &asteroids[i]);
            let line = get_line(&asteroids[index], &asteroids[i], quadrant);
            line_to_asteroids
                .entry((quadrant, line))
                .or_insert_with(HashSet::new)
                .insert(asteroids[i].clone());
        }
    }
    line_to_asteroids
}

fn get_asteroid_with_most_visible(asteroids: &[Asteroid]) -> Result<(Asteroid, usize), Error> {
    let (best_asteroid, num_visible) = (0..asteroids.len())
        .map(|a| {
            let visible_asteroids = put_asteroids_on_lines(a, asteroids);
            (asteroids[a], visible_asteroids.len())
        })
        .max_by(|a, b| a.1.cmp(&b.1))
        .unwrap();
    Ok((best_asteroid, num_visible))
}

fn get_distance(asteroid_1: &Asteroid, asteroid_2: &Asteroid) -> i64 {
    (asteroid_2.0 - asteroid_1.0).pow(2) + (asteroid_2.1 - asteroid_1.1).pow(2)
}

fn shoot_asteroids_in_order(index: usize, asteroids: &[Asteroid]) -> HashMap<usize, Asteroid> {
    let mut line_to_asteroids = put_asteroids_on_lines(index, asteroids);
    let mut asteroid_to_index = HashMap::new();
    let mut shoot_index = 1;
    let sorted_keys = line_to_asteroids
        .keys()
        .cloned()
        .sorted_by(|a, b| (a.0, (b.1).0).cmp(&(b.0, (a.1).0)))
        .collect::<Vec<_>>();
    while sorted_keys.iter().any(|k| !line_to_asteroids[k].is_empty()) {
        for key in &sorted_keys {
            if !line_to_asteroids[key].is_empty() {
                let closest = *line_to_asteroids[key]
                    .iter()
                    .map(|a| (get_distance(&asteroids[index], a), a))
                    .min_by(|a, b| a.0.cmp(&b.0))
                    .unwrap()
                    .1;
                line_to_asteroids.get_mut(key).unwrap().remove(&closest);
                asteroid_to_index.insert(shoot_index, closest);
                shoot_index += 1;
            }
        }
    }
    asteroid_to_index
}

pub fn solve_day_10_1(input: &str) -> Result<usize, Error> {
    let asteroids = parse_asteroid_map(input)?;
    Ok(get_asteroid_with_most_visible(&asteroids)?.1)
}

pub fn solve_day_10_2(input: &str) -> Result<usize, Error> {
    let asteroids = parse_asteroid_map(input)?;
    let center = get_asteroid_with_most_visible(&asteroids)?.0;
    let center_index = (0..asteroids.len())
        .find(|i| asteroids[*i] == center)
        .unwrap();
    let asteroid_200 = shoot_asteroids_in_order(center_index, &asteroids)[&200];
    Ok((asteroid_200.0 * 100 + asteroid_200.1) as usize)
}

#[cfg(test)]
mod tests {
    use crate::utility::input_from_file;

    use super::*;

    #[test]
    fn test_1() -> Result<(), Error> {
        let asteroids = parse_asteroid_map(".#..#\n.....\n#####\n....#\n...##")?;
        let (best_asteroid, num_visible) = get_asteroid_with_most_visible(&asteroids)?;
        assert_eq!(best_asteroid, (3, 4));
        assert_eq!(num_visible, 8);
        let asteroids = parse_asteroid_map("......#.#.\n#..#.#....\n..#######.\n.#.#.###..\n.#..#.....\n..#....#.#\n#..#....#.\n.##.#..###\n##...#..#.\n.#....####")?;
        let (best_asteroid, num_visible) = get_asteroid_with_most_visible(&asteroids)?;
        assert_eq!(best_asteroid, (5, 8));
        assert_eq!(num_visible, 33);
        let asteroids = parse_asteroid_map("#.#...#.#.\n.###....#.\n.#....#...\n##.#.#.#.#\n....#.#.#.\n.##..###.#\n..#...##..\n..##....##\n......#...\n.####.###.")?;
        let (best_asteroid, num_visible) = get_asteroid_with_most_visible(&asteroids)?;
        assert_eq!(best_asteroid, (1, 2));
        assert_eq!(num_visible, 35);
        let asteroids = parse_asteroid_map(".#..#..###\n####.###.#\n....###.#.\n..###.##.#\n##.##.#.#.\n....###..#\n..#.#..#.#\n#..#.#.###\n.##...##.#\n.....#.#..")?;
        let (best_asteroid, num_visible) = get_asteroid_with_most_visible(&asteroids)?;
        assert_eq!(best_asteroid, (6, 3));
        assert_eq!(num_visible, 41);
        let asteroids = parse_asteroid_map(&input_from_file("data/day_10_test.input")?)?;
        let (best_asteroid, num_visible) = get_asteroid_with_most_visible(&asteroids)?;
        assert_eq!(best_asteroid, (11, 13));
        assert_eq!(num_visible, 210);
        Ok(())
    }

    #[test]
    fn test_2() -> Result<(), Error> {
        let asteroids = parse_asteroid_map(&input_from_file("data/day_10_test.input")?)?;
        let a = (0..asteroids.len())
            .find(|i| asteroids[*i] == (11, 13))
            .unwrap();
        let order = shoot_asteroids_in_order(a, &asteroids);
        assert_eq!((11, 12), order[&1]);
        assert_eq!((12, 1), order[&2]);
        assert_eq!((12, 2), order[&3]);
        assert_eq!((12, 8), order[&10]);
        assert_eq!((16, 0), order[&20]);
        assert_eq!((16, 9), order[&50]);
        assert_eq!((10, 16), order[&100]);
        assert_eq!((9, 6), order[&199]);
        assert_eq!((8, 2), order[&200]);
        assert_eq!((10, 9), order[&201]);
        assert_eq!((11, 1), order[&299]);
        Ok(())
    }
}
