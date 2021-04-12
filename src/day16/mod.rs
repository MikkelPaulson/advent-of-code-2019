use std::iter;
use std::ops::Range;

pub fn part1(input: &str) -> Result<u64, String> {
    Ok(cycles_slice(&parse(input)?, 0..8, 1, 100))
}

pub fn part2(input: &str) -> Result<u64, String> {
    let offset = cycles_slice(&parse(input)?, 0..7, 1, 0) as usize;
    Ok(cycles_slice(
        &parse(input)?,
        offset..offset + 8,
        10_000,
        100,
    ))
}

fn cycles_slice(
    input: &Vec<u8>,
    range: Range<usize>,
    repetitions: usize,
    cycle_count: usize,
) -> u64 {
    let digits = cycles(input, range.start, repetitions, cycle_count);
    let range_end = range.end;

    range
        .map(|i| digits[i] as u64 * 10u64.pow((range_end - i - 1) as u32))
        .sum()
}

fn cycles(input: &Vec<u8>, start_pos: usize, repetitions: usize, cycle_count: usize) -> Vec<u8> {
    let mut digits = (0..repetitions)
        .flat_map(|_| input.iter())
        .copied()
        .collect();

    for i in 0..cycle_count {
        println!("Cycle {}", i);
        cycle(&mut digits, start_pos);
    }

    digits
}

fn cycle(input: &mut Vec<u8>, start_pos: usize) {
    *input = if start_pos > input.len() / 2 {
        let mut result: Vec<u8> = iter::repeat(0).take(input.len()).collect();
        let mut acc: u64 = 0;
        for i in (start_pos..input.len()).rev() {
            acc = acc + input[i] as u64;
            result[i] = (acc % 10) as u8;
        }
        result
    } else {
        (start_pos..input.len()).map(|i| digit(&input, i)).collect()
    };
}

fn digit(input: &[u8], calc_pos: usize) -> u8 {
    let mut acc: i64 = 0;
    let mut i = 0;
    let repetitions = calc_pos + 1;
    let half_step = repetitions * 2;
    let step = repetitions * 4;

    while i + calc_pos < input.len() {
        let mut start = i + calc_pos;
        let mut end = (start + calc_pos).min(input.len() - 1);
        let sum: i64 = input[start..=end].iter().map(|i| *i as i64).sum();
        acc = acc + sum;

        start += half_step;
        if start < input.len() {
            end = (end + half_step).min(input.len() - 1);
            let sum: i64 = input[start..=end].iter().map(|i| *i as i64).sum();
            acc -= sum;
        }

        i += step;
    }

    (acc.abs() % 10) as u8
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
    use super::{cycles_slice, part1, part2};

    #[test]
    fn part1_examples() {
        let digits = vec![1, 2, 3, 4, 5, 6, 7, 8];

        assert_eq!(48226158, cycles_slice(&digits, 0..8, 1, 1));
        assert_eq!(34040438, cycles_slice(&digits, 0..8, 1, 2));
        assert_eq!(03415518, cycles_slice(&digits, 0..8, 1, 3));
        assert_eq!(01029498, cycles_slice(&digits, 0..8, 1, 4));

        assert_eq!(Ok(24176176), part1("80871224585914546619083218645595"));
        assert_eq!(Ok(73745418), part1("19617804207202209144916044189917"));
        assert_eq!(Ok(52432133), part1("69317163492948606335995924319873"));
    }

    #[test]
    fn part1_solution() {
        assert_eq!(Ok(96136976), part1(include_str!("input.txt")));
    }

    #[test]
    fn part2_examples() {
        assert_eq!(Ok(84462026), part2("03036732577212944063491565474664"));
        assert_eq!(Ok(78725270), part2("02935109699940807407585447034323"));
        assert_eq!(Ok(53553731), part2("03081770884921959731165446850517"));
    }

    #[test]
    #[ignore]
    fn part2_solution() {
        assert_eq!(Ok(85600369), part2(include_str!("input.txt")));
    }
}
