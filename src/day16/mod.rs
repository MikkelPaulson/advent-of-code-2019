use std::collections::HashMap;
use std::ops::Range;

pub fn part1(input: &str) -> Result<u64, String> {
    let mut cache = HashMap::new();
    get_digits(&mut cache, &parse(input)?, 1, 100, 0..8)
}

pub fn part2(input: &str) -> Result<u64, String> {
    let digits = parse(input)?;
    let mut cache = HashMap::new();

    let offset = get_digits(&mut cache, &digits, 10000, 100, 0..8)? as usize;
    println!("Calculating offset {}", offset);

    get_digits(&mut cache, &digits, 10000, 100, offset..offset + 8)
}

fn get_digits(
    cache: &mut HashMap<(usize, usize), i16>,
    digits: &Vec<i16>,
    repetitions: usize,
    cycles: usize,
    range: Range<usize>,
) -> Result<u64, String> {
    cache.reserve(
        (digits.len() * repetitions * cycles)
            .checked_sub(cache.capacity())
            .unwrap_or_default(),
    );

    let range_end = range.end;

    Ok(range
        .map(|i| {
            get_digit(cache, digits, repetitions, cycles, i) as u64
                * 10u64.pow((range_end - i - 1) as u32)
        })
        .sum())
}

fn get_digit(
    cache: &mut HashMap<(usize, usize), i16>,
    digits: &Vec<i16>,
    repetitions: usize,
    cycle: usize,
    position: usize,
) -> i16 {
    if position > digits.len() * repetitions {
        0
    } else if cycle == 0 {
        digits.get(position % digits.len()).copied().unwrap()
    } else if let Some(i) = cache.get(&(cycle - 1, position)) {
        *i
    } else {
        let result = ((0..(digits.len() * repetitions)).fold(0i16, |acc, i| {
            match (i + 1) / (position + 1) % 4 {
                1 => acc + get_digit(cache, digits, repetitions, cycle - 1, i),
                3 => acc - get_digit(cache, digits, repetitions, cycle - 1, i),
                _ => acc,
            }
        }) % 10)
            .abs();

        cache.insert((cycle - 1, position), result);
        result
    }
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
    use super::{get_digits, part1, part2, HashMap};

    #[test]
    fn part1_examples() {
        let digits = vec![1, 2, 3, 4, 5, 6, 7, 8];

        let mut cache = HashMap::new();
        assert_eq!(Ok(48226158), get_digits(&mut cache, &digits, 1, 1, 0..8));
        cache.clear();
        assert_eq!(Ok(34040438), get_digits(&mut cache, &digits, 1, 2, 0..8));
        cache.clear();
        assert_eq!(Ok(03415518), get_digits(&mut cache, &digits, 1, 3, 0..8));
        cache.clear();
        assert_eq!(Ok(01029498), get_digits(&mut cache, &digits, 1, 4, 0..8));

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
