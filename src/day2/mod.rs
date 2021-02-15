use crate::intcode::Intcode;

pub fn part1(input: &str) -> Result<u64, String> {
    let mut intcode: Intcode = input.parse()?;

    intcode.set(1, 12);
    intcode.set(2, 2);

    intcode.run();

    Ok(intcode.get(0) as u64)
}

pub fn part2(input: &str) -> Result<u64, String> {
    let clean_intcode: Intcode = input.parse()?;

    for noun in 0..100 {
        for verb in 0..100 {
            let mut intcode = clean_intcode.clone();

            intcode.set(1, noun);
            intcode.set(2, verb);

            intcode.run();

            if intcode.get(0) == 19690720 {
                return Ok((100 * noun + verb) as u64);
            }
        }
    }
    Err("No matching result was found.".to_string())
}

#[cfg(test)]
mod test {
    use super::{part1, part2};

    #[test]
    fn part1_solution() {
        assert_eq!(Ok(4138658), part1(include_str!("input.txt")));
    }

    #[test]
    fn part2_solution() {
        assert_eq!(Ok(7264), part2(include_str!("input.txt")));
    }
}
