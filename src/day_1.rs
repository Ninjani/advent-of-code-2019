use anyhow::Error;

fn get_fuel_for_mass(mass: f64) -> f64 {
    (mass / 3.).floor() - 2.
}

/// iterative version
fn get_fuel_for_mass_and_fuel_i(mass: f64) -> f64 {
    let mut new_mass = mass;
    let mut fuel = 0.;
    while new_mass > 0. {
        new_mass = get_fuel_for_mass(new_mass);
        if new_mass > 0. {
            fuel += new_mass;
        } else {
            break;
        }
    }
    fuel
}

/// recursive version
fn get_fuel_for_mass_and_fuel_r(mass: f64) -> f64 {
    let new_mass = get_fuel_for_mass(mass);
    if new_mass > 0. {
        new_mass + get_fuel_for_mass_and_fuel_r(new_mass)
    } else {
        0.
    }
}

pub fn solve_day_1_1(input: &str) -> Result<f64, Error> {
    let masses: Vec<_> = input
        .split('\n')
        .map(|line| line.parse::<f64>())
        .collect::<Result<_, _>>()?;
    Ok(masses.into_iter().map(get_fuel_for_mass).sum())
}

pub fn solve_day_1_2(input: &str) -> Result<f64, Error> {
    let masses: Vec<_> = input
        .split('\n')
        .map(|line| line.parse::<f64>())
        .collect::<Result<_, _>>()?;
    Ok(masses.into_iter().map(get_fuel_for_mass_and_fuel_r).sum())
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
    fn test_2_i() -> Result<(), Error> {
        assert_eq!(2., get_fuel_for_mass_and_fuel_i(14.));
        assert_eq!(966., get_fuel_for_mass_and_fuel_i(1969.));
        assert_eq!(50346., get_fuel_for_mass_and_fuel_i(100756.));
        Ok(())
    }

    #[test]
    fn test_2_r() -> Result<(), Error> {
        assert_eq!(2., get_fuel_for_mass_and_fuel_r(14.));
        assert_eq!(966., get_fuel_for_mass_and_fuel_r(1969.));
        assert_eq!(50346., get_fuel_for_mass_and_fuel_r(100756.));
        Ok(())
    }
}
