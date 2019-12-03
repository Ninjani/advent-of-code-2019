use anyhow::Error;

fn process_intcode(intcode: &mut [usize]) {
    for i in (0..intcode.len()).step_by(4) {
        match intcode[i] {
            99 => break,
            1 => intcode[intcode[i + 3]] = intcode[intcode[i + 1]] + intcode[intcode[i + 2]],
            2 => intcode[intcode[i + 3]] = intcode[intcode[i + 1]] * intcode[intcode[i + 2]],
            n => panic!("Unknown opcode {}", n),
        }
    }
}

fn find_noun_verb(intcode: &[usize], output_equals: usize) -> Option<(usize, usize)> {
    for noun in 0..100 {
        for verb in 0..100 {
            let mut intcode_noun_verb = intcode.to_vec();
            intcode_noun_verb[1] = noun;
            intcode_noun_verb[2] = verb;
            process_intcode(&mut intcode_noun_verb);
            if intcode_noun_verb[0] == output_equals {
                return Some((noun, verb));
            }
        }
    }
    None
}

pub fn solve_day_2_1(input: &str) -> Result<usize, Error> {
    let mut intcode: Vec<_> = input
        .split(',')
        .map(|i| i.parse::<usize>())
        .collect::<Result<_, _>>()?;
    intcode[1] = 12;
    intcode[2] = 2;
    process_intcode(&mut intcode);
    Ok(intcode[0])
}

pub fn solve_day_2_2(input: &str) -> Result<usize, Error> {
    let intcode: Vec<_> = input
        .split(',')
        .map(|i| i.parse::<usize>())
        .collect::<Result<_, _>>()?;
    match find_noun_verb(&intcode, 19_690_720) {
        Some((noun, verb)) => Ok(100 * noun + verb),
        None => Err(anyhow!("(noun verb) pair not found")),
    }
}

#[cfg(test)]
mod tests {
    use crate::utility::input_from_file;

    use super::*;

    #[test]
    fn test_1() -> Result<(), Error> {
        let mut intcode = vec![1, 0, 0, 0, 99];
        process_intcode(&mut intcode);
        assert_eq!(vec![2, 0, 0, 0, 99], intcode);
        let mut intcode = vec![2, 3, 0, 3, 99];
        process_intcode(&mut intcode);
        assert_eq!(vec![2, 3, 0, 6, 99], intcode);
        let mut intcode = vec![2, 4, 4, 5, 99, 0];
        process_intcode(&mut intcode);
        assert_eq!(vec![2, 4, 4, 5, 99, 9801], intcode);
        let mut intcode = vec![1, 1, 1, 4, 99, 5, 6, 0, 99];
        process_intcode(&mut intcode);
        assert_eq!(vec![30, 1, 1, 4, 2, 5, 6, 0, 99], intcode);
        Ok(())
    }

    #[test]
    fn test_2() -> Result<(), Error> {
        let intcode: Vec<_> = input_from_file("data/day_2.input")?
            .split(',')
            .map(|i| i.parse::<usize>())
            .collect::<Result<_, _>>()?;
        let (noun, verb) = find_noun_verb(&intcode, 4945026).unwrap();
        assert_eq!(1202, 100 * noun + verb);
        Ok(())
    }
}
