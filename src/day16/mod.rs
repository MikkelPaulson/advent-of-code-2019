use std::mem;

pub fn part1(input: &str) -> Result<u64, String> {
    part1_phases(input, 100)
}

pub fn part2(_input: &str) -> Result<u64, String> {
    Err("Incomplete".to_string())
}

fn part1_phases(input: &str, phases: u8) -> Result<u64, String> {
    let mut digits = parse(input)?;
    let mut temp: Vec<u8> = Vec::with_capacity(digits.len());

    for _ in 0..phases {
        (0..digits.len()).for_each(|i| temp.push(get_digit(&digits, i)));

        mem::swap(&mut digits, &mut temp);
        temp.clear();
    }

    Ok(digits
        .iter()
        .take(8)
        .enumerate()
        .map(|(i, d)| (*d as u64) * 10u64.pow(7 - i as u32))
        .sum())
}

fn get_digit(digits: &Vec<u8>, position: usize) -> u8 {
    let mut iter = digits.iter();
    let mut result: i64 = 0;

    for _ in 0..(digits.len() / (position + 1) * 4) {
        result += *iter.nth(position).unwrap_or(&0) as i64;
        (0..position).for_each(|_| result += *iter.next().unwrap_or(&0) as i64);

        result -= *iter.nth(position + 1).unwrap_or(&0) as i64;
        (0..position).for_each(|_| result -= *iter.next().unwrap_or(&0) as i64);

        iter.next();
    }

    (result % 10).abs() as u8
}

fn parse(input: &str) -> Result<Vec<u8>, String> {
    input
        .trim()
        .chars()
        .map(|c| {
            c.to_digit(10)
                .map(|c| c as u8)
                .ok_or_else(|| format!("Invalid digit: {}", c))
        })
        .collect::<Result<_, _>>()
}

#[cfg(test)]
mod test {
    use super::{part1, part1_phases, part2};

    #[test]
    fn part1_examples() {
        assert_eq!(Ok(48226158), part1_phases("12345678", 1));
        assert_eq!(Ok(34040438), part1_phases("12345678", 2));
        assert_eq!(Ok(03415518), part1_phases("12345678", 3));
        assert_eq!(Ok(01029498), part1_phases("12345678", 4));
        assert_eq!(Ok(24176176), part1("80871224585914546619083218645595"));
        assert_eq!(Ok(73745418), part1("19617804207202209144916044189917"));
        assert_eq!(Ok(52432133), part1("69317163492948606335995924319873"));
    }

    #[test]
    #[ignore]
    fn part1_solution() {
        assert_eq!(Ok(96136976), part1(include_str!("input.txt")));
    }

    #[test]
    #[ignore]
    fn part2_examples() {
        assert_eq!(Ok(84462026), part2("03036732577212944063491565474664"));
        assert_eq!(Ok(78725270), part2("02935109699940807407585447034323"));
        assert_eq!(Ok(53553731), part2("03081770884921959731165446850517"));
    }

    #[test]
    #[ignore]
    fn part2_solution() {
        assert_eq!(Ok(0), part2(include_str!("input.txt")));
    }
}
