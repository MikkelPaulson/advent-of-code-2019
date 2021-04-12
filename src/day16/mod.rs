use std::collections::HashMap;
use std::ops::Range;

pub fn part1(input: &str) -> Result<u64, String> {
    Ok(slice_digits(&parse(input)?, 1, 100, 0..8))
}

pub fn part2(input: &str) -> Result<u64, String> {
    let digits = parse(input)?;
    let offset = slice_digits(&digits, 1, 0, 0..7) as usize;
    println!("Offset: {}", offset);
    Ok(slice_digits(&digits, 10_000, 100, offset..offset + 8))
}

fn slice_digits(digits: &Vec<u8>, repetitions: usize, cycles: u8, range: Range<usize>) -> u64 {
    let range_end = range.end;
    let mut cache = HashMap::new();

    range
        .map(|i| {
            digit(&mut cache, &digits, repetitions, cycles, i) as u64
                * 10u64.pow((range_end - i - 1) as u32)
        })
        .sum()
}

fn digit(
    cache: &mut HashMap<(u8, usize), u8>,
    input: &[u8],
    repetitions: usize,
    cycle: u8,
    calc_pos: usize,
) -> i64 {
    if cycle == 0 {
        if calc_pos < input.len() * repetitions {
            input[calc_pos % input.len()] as i64
        } else {
            0
        }
    } else {
        //println!("digit(_, _, {}, {}, {})", repetitions, cycle, calc_pos);
        match cache.get(&(cycle, calc_pos)) {
            Some(result) => *result as i64,
            None => {
                let mut acc: i64 = 0;
                let mut i = 0;
                let half_step = (calc_pos + 1) * 2;
                let step = (calc_pos + 1) * 4;
                let repeated_len = input.len() * repetitions;

                while i + calc_pos < repeated_len {
                    let mut start = i + calc_pos;
                    let mut end = (start + calc_pos).min(repeated_len - 1);
                    let sum: i64 = (start..=end)
                        .map(|i| digit(cache, input, repetitions, cycle - 1, i))
                        .sum();
                    acc = acc + sum;

                    start += half_step;
                    if start < repeated_len {
                        end = (end + half_step).min(repeated_len - 1);
                        let sum: i64 = (start..=end)
                            .map(|i| digit(cache, input, repetitions, cycle - 1, i))
                            .sum();
                        acc -= sum;
                    }

                    i += step;
                }

                let result = acc.abs() % 10;
                cache.insert((cycle, calc_pos), result as u8);
                result
            }
        }
    }
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
    use super::{part1, part2, slice_digits};

    #[test]
    fn part1_examples() {
        let digits = vec![1, 2, 3, 4, 5, 6, 7, 8];

        assert_eq!(48226158, slice_digits(&digits, 1, 1, 0..8));
        assert_eq!(34040438, slice_digits(&digits, 1, 2, 0..8));
        assert_eq!(03415518, slice_digits(&digits, 1, 3, 0..8));
        assert_eq!(01029498, slice_digits(&digits, 1, 4, 0..8));

        assert_eq!(Ok(24176176), part1("80871224585914546619083218645595"));
        assert_eq!(Ok(73745418), part1("19617804207202209144916044189917"));
        assert_eq!(Ok(52432133), part1("69317163492948606335995924319873"));
    }

    #[test]
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
