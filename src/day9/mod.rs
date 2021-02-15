use crate::intcode::Intcode;

pub fn part1(input: &str) -> Result<u64, String> {
    let mut intcode = input.parse::<Intcode>()?;

    intcode.input.push(1);
    intcode.run();

    println!("Output: {:?}", intcode.output);

    intcode
        .output
        .pop()
        .ok_or_else(|| "No output.".to_string())
        .map(|n| n as u64)
}

pub fn part2(input: &str) -> Result<u64, String> {
    let mut intcode = input.parse::<Intcode>()?;

    intcode.input.push(2);
    intcode.run();

    println!("Output: {:?}", intcode.output);

    intcode
        .output
        .pop()
        .ok_or_else(|| "No output.".to_string())
        .map(|n| n as u64)
}

#[cfg(test)]
mod test {
    use super::{part1, part2};

    #[test]
    fn part1_solution() {
        assert_eq!(Ok(3507134798), part1(include_str!("input.txt")));
    }

    #[test]
    fn part2_solution() {
        assert_eq!(Ok(84513), part2(include_str!("input.txt")));
    }
}
