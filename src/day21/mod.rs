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
        _ => Err(format!("Failed after {} steps.", intcode.steps)),
    }
}

pub fn part2(input: &str) -> Result<u64, String> {
    let mut intcode: Intcode = input.parse()?;

    // AB.D.FG.. -- no jump (prev: jump)
    // A.C.EF... -- no jump
    // .B.DE...I -- jump
    //
    // AB.D.F.H. -- jump
    // abcDefgH
    //
    // Is there a point to jumping now? (hole in A/B/C, no hole in D)
    // Is there a reason next round won't do? (A is a hole or E is a hole)
    // Is there a reason the round after won't do? (B is a hole or F is a hole)
    // Or the round after that? (C is a hole or G is a hole)

    intcode.input_str(
        "NOT A J
NOT B T
OR T J
NOT C T
OR T J
AND D J
RUN
",
    );

    intcode.run();

    println!("{}", intcode.output_string());

    match intcode.output.last() {
        Some(&i) if i > 255 => Ok(i as u64),
        _ => Err(format!("Failed after {} steps.", intcode.steps)),
    }
}

#[cfg(test)]
mod test {
    use super::{part1, part2};

    #[test]
    fn part1_solution() {
        assert_eq!(Ok(19348359), part1(include_str!("input.txt")));
    }

    #[test]
    #[ignore]
    fn part2_solution() {
        assert_eq!(Ok(0), part2(include_str!("input.txt")));
    }
}
