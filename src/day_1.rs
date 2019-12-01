use crate::utility::input_from_file;
use anyhow::Error;

fn get_fuel_for_mass(mass: f64) -> f64 {
    (mass / 3.).floor() - 2.
}

fn get_fuel_for_mass_and_fuel(mass: f64) -> f64 {
    let mut remaining_mass = mass;
    let mut fuel = 0.;
    let mut mass_fuel;
    while remaining_mass > 0. {
        mass_fuel = get_fuel_for_mass(remaining_mass);
        if mass_fuel > 0. {
            fuel += mass_fuel;
            remaining_mass = mass_fuel;
        } else {
            break;
        }
    }
    fuel
}

fn get_fuel_for_spacecraft_1(masses: &[f64]) -> f64 {
    masses.iter().map(|mass| get_fuel_for_mass(*mass)).sum()
}

fn get_fuel_for_spacecraft_2(masses: &[f64]) -> f64 {
    masses.iter().map(|mass| get_fuel_for_mass_and_fuel(*mass)).sum()
}

pub fn solve_day_1_1() -> Result<f64, Error> {
    let masses = input_from_file("data/day_1_1.input")?
        .split('\n')
        .map(|line| line.parse::<f64>()).collect::<Result<Vec<_>, _>>()?;
    Ok(get_fuel_for_spacecraft_1(&masses))
}

pub fn solve_day_1_2() -> Result<f64, Error> {
    let masses = input_from_file("data/day_1_2.input")?
        .split('\n')
        .map(|line| line.parse::<f64>()).collect::<Result<Vec<_>, _>>()?;
    Ok(get_fuel_for_spacecraft_2(&masses))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() -> Result<(), Error> {
        assert_eq!(2., get_fuel_for_mass(12.));
        assert_eq!(2., get_fuel_for_mass(14.));
        assert_eq!(654., get_fuel_for_mass(1969.));
        assert_eq!(33583., get_fuel_for_mass(100756.));
        Ok(())
    }

    #[test]
    fn test_2() -> Result<(), Error> {
        assert_eq!(2., get_fuel_for_mass_and_fuel(14.));
        assert_eq!(966., get_fuel_for_mass_and_fuel(1969.));
        assert_eq!(50346., get_fuel_for_mass_and_fuel(100756.));
        Ok(())
    }
}