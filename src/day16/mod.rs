use std::collections::HashMap;
use std::mem;
use std::ops::Range;

pub fn part1(input: &str) -> Result<u64, String> {
    let output = cycles(&parse(input)?, 1, 100);
    Ok(slice_digits(&output, 0..8))
}

pub fn part2(input: &str) -> Result<u64, String> {
    Err("Not implemented".to_string())
}

fn slice_digits(digits: &Vec<i16>, range: Range<usize>) -> u64 {
    let range_end = range.end;

    range
        .map(|i| digits[i] as u64 * 10u64.pow((range_end - i - 1) as u32))
        .sum()
}

fn cycles(input: &Vec<i16>, repetitions: usize, cycles: usize) -> Vec<i16> {
    let mut digits = (0..repetitions)
        .flat_map(|_| input.iter())
        .copied()
        .collect();

    for i in 0..cycles {
        cycle(&mut digits);
        println!("Cycle {}: {:?}", i, digits);
    }

    digits
}

fn cycle(input: &mut Vec<i16>) {
    let result = (0..input.len()).map(|i| digit(&input, i)).collect();
    mem::replace(input, result);
}

fn digit(input: &[i16], calc_pos: usize) -> i16 {
    let mut acc: i16 = 0;
    let mut i = 0;
    let repetitions = calc_pos + 1;
    let half_step = repetitions * 2;
    let step = repetitions * 4;

    while i + calc_pos < input.len() {
        let mut start = i + calc_pos;
        let mut end = (start + calc_pos).min(input.len() - 1);
        let sum: i16 = input[start..=end].iter().sum();
        acc = acc + sum;

        start += half_step;
        if start < input.len() {
            end = (end + half_step).min(input.len() - 1);
            let sum: i16 = input[start..=end].iter().sum();
            acc -= sum;
        }

        i += step;
    }

    acc.abs() % 10
}

fn parse(input: &str) -> Result<Vec<i16>, String> {
    input
        .trim()
        .chars()
        .map(|c| {
            c.to_digit(10)
                .map(|c| c as i16)
                .ok_or_else(|| format!("Invalid digit: {}", c))
        })
        .collect::<Result<_, _>>()
}

#[cfg(test)]
mod test {
    use super::{cycles, part1, part2, slice_digits};

    #[test]
    fn part1_examples() {
        let digits = vec![1, 2, 3, 4, 5, 6, 7, 8];

        assert_eq!(48226158, slice_digits(&cycles(&digits, 1, 1), 0..8));
        assert_eq!(34040438, slice_digits(&cycles(&digits, 1, 2), 0..8));
        assert_eq!(03415518, slice_digits(&cycles(&digits, 1, 3), 0..8));
        assert_eq!(01029498, slice_digits(&cycles(&digits, 1, 4), 0..8));

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
