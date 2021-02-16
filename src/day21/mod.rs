use super::intcode::Intcode;

pub fn part1(input: &str) -> Result<u64, String> {
    let mut intcode: Intcode = input.parse()?;

    intcode.input_str(
        "NOT A J
NOT B T
OR T J
NOT C T
OR T J
AND D J
WALK
",
    );

    intcode.run();

    println!("{}", intcode.output_string());

    match intcode.output.last() {
        Some(&i) if i > 255 => Ok(i as u64),
        _ => Err("Failed to complete operation.".to_string()),
    }
}

#[cfg(test)]
mod test {
    use super::part1;

    #[test]
    fn part1_solution() {
        assert_eq!(Ok(19348359), part1(include_str!("input.txt")));
    }
}
