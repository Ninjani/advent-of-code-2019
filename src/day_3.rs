use std::collections::{HashMap, HashSet};

use anyhow::Error;

type Point = (isize, isize);
type Wire = HashMap<(isize, isize), usize>;

const CENTRAL_PORT: Point = (0, 0);

fn parse_movement(movement: &str) -> Result<Vec<Point>, Error> {
    let direction = movement.chars().next().unwrap();
    let number = movement
        .chars()
        .skip(1)
        .collect::<String>()
        .parse::<isize>()?;
    match direction {
        'R' => Ok((1..=number).map(|i| (i, 0)).collect()),
        'L' => Ok((1..=number).map(|i| (-i, 0)).collect()),
        'U' => Ok((1..=number).map(|i| (0, i)).collect()),
        'D' => Ok((1..=number).map(|i| (0, -i)).collect()),
        c => panic!("Unknown direction {}", c),
    }
}

fn get_wire_points(directions: &[&str]) -> Result<Wire, Error> {
    let mut points = HashMap::new();
    let mut num_steps = 0;
    let (mut current_point, mut new_point) = (CENTRAL_PORT, CENTRAL_PORT);
    for direction in directions {
        for (move_to_x, move_to_y) in parse_movement(direction)? {
            num_steps += 1;
            new_point = (current_point.0 + move_to_x, current_point.1 + move_to_y);
            if new_point != CENTRAL_PORT && !points.contains_key(&new_point) {
                points.insert(new_point, num_steps);
            }
        }
        current_point = new_point;
    }
    Ok(points)
}

fn manhattan_distance(point_1: Point, point_2: Point) -> usize {
    ((point_1.0 - point_2.0).abs() + (point_1.1 - point_2.1).abs()) as usize
}

fn get_closest_intersection_manhattan(wire_1: &Wire, wire_2: &Wire) -> usize {
    wire_1
        .keys()
        .collect::<HashSet<_>>()
        .intersection(&wire_2.keys().collect())
        .map(|point| manhattan_distance(**point, CENTRAL_PORT))
        .min()
        .unwrap()
}

pub fn solve_day_3_1(input: &str) -> Result<usize, Error> {
    let mut lines = input.split('\n');
    let wire_1 = get_wire_points(
        &lines
            .next()
            .ok_or_else(|| anyhow!("Input empty"))?
            .split(',')
            .collect::<Vec<_>>(),
    )?;
    let wire_2 = get_wire_points(
        &lines
            .next()
            .ok_or_else(|| anyhow!("Input needs two lines"))?
            .split(',')
            .collect::<Vec<_>>(),
    )?;
    Ok(get_closest_intersection_manhattan(&wire_1, &wire_2))
}

fn get_closest_intersection_steps(wire_1: &Wire, wire_2: &Wire) -> usize {
    wire_1
        .keys()
        .collect::<HashSet<_>>()
        .intersection(&wire_2.keys().collect())
        .map(|point| wire_1[&point] + wire_2[&point])
        .min()
        .unwrap()
}

pub fn solve_day_3_2(input: &str) -> Result<usize, Error> {
    let mut lines = input.split('\n');
    let wire_1 = get_wire_points(
        &lines
            .next()
            .ok_or_else(|| anyhow!("Input empty"))?
            .split(',')
            .collect::<Vec<_>>(),
    )?;
    let wire_2 = get_wire_points(
        &lines
            .next()
            .ok_or_else(|| anyhow!("Input needs two lines"))?
            .split(',')
            .collect::<Vec<_>>(),
    )?;
    Ok(get_closest_intersection_steps(&wire_1, &wire_2))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() -> Result<(), Error> {
        assert_eq!(6, solve_day_3_1("R8,U5,L5,D3\nU7,R6,D4,L4")?);
        assert_eq!(
            159,
            solve_day_3_1("R75,D30,R83,U83,L12,D49,R71,U7,L72\nU62,R66,U55,R34,D71,R55,D58,R83")?
        );
        assert_eq!(
            135,
            solve_day_3_1(
                "R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51\nU98,R91,D20,R16,D67,R40,U7,R15,U6,R7"
            )?
        );
        Ok(())
    }

    #[test]
    fn test_2() -> Result<(), Error> {
        assert_eq!(30, solve_day_3_2("R8,U5,L5,D3\nU7,R6,D4,L4")?);
        assert_eq!(
            610,
            solve_day_3_2("R75,D30,R83,U83,L12,D49,R71,U7,L72\nU62,R66,U55,R34,D71,R55,D58,R83")?
        );
        assert_eq!(
            410,
            solve_day_3_2(
                "R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51\nU98,R91,D20,R16,D67,R40,U7,R15,U6,R7"
            )?
        );
        Ok(())
    }
}
