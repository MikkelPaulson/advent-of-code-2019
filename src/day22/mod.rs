use std::fmt;
use std::str::FromStr;

pub fn part1(input: &str) -> Result<u64, String> {
    let deck_len = 10007i128;

    let instructions = fold(parse(input)?, deck_len);

    Ok(card_position(&instructions, 2019, deck_len) as u64)
}

pub fn part2(input: &str) -> Result<u64, String> {
    let deck_len = 119_315_717_514_047i128;
    let shuffles = 101_741_582_076_661i128;
    let reversed_shuffles = deck_len - shuffles - 1;

    let instructions = fold(parse(input)?, deck_len);

    let mut card_index = 2020;

    for shuffle in 0..reversed_shuffles {
        if shuffle % 1_000_000 == 0 {
            eprintln!(
                "Shuffle {shuffle} of {reversed_shuffles} ({}%): {card_index} ({}%)",
                shuffle * 100 / reversed_shuffles,
                card_index * 100 / deck_len,
            );
        }

        card_index = card_position(&instructions, card_index, deck_len);
    }

    Ok(card_index as u64)
}

fn card_position(instructions: &[Instruction], card: i128, deck_len: i128) -> i128 {
    assert!(card < deck_len, "No card of value {} in deck", card);
    instructions.into_iter().fold(card, |card, instruction| {
        instruction.card_position(card, deck_len)
    })
}

/*
fn card_at(instructions: &[Instruction], position: i128, deck_size: i128) -> i128 {
    assert!(position < deck_size, "No card at position {}", position);
    instructions
        .into_iter()
        .rev()
        .fold(position, |position, instruction| {
            instruction.card_at(position, deck_size)
        })
}
*/

fn fold(mut instructions: Vec<Instruction>, len: i128) -> Vec<Instruction> {
    if instructions.is_empty() {
        return Vec::new();
    }

    loop {
        let folded_instructions: Vec<_> = instructions[1..].iter().cloned().fold(
            vec![instructions[0].clone()],
            |mut acc, instruction| {
                let prev = acc.pop().unwrap();
                let (a, b) = prev.fold(instruction, len);
                acc.push(a);
                if let Some(b) = b {
                    acc.push(b);
                }
                acc
            },
        );

        if instructions == folded_instructions {
            return instructions;
        } else {
            instructions = folded_instructions;
        }
    }
}

fn parse(input: &str) -> Result<Vec<Instruction>, String> {
    input.lines().map(|line| line.parse()).collect()
}

#[derive(Clone, Debug, Eq, PartialEq)]
enum Instruction {
    Noop,
    Cut(i128),
    DealWithIncrement(i128),
    DealAndCut(i128, i128),
    DealIntoNewStack,
}

impl Instruction {
    fn card_position(&self, card: i128, len: i128) -> i128 {
        match self {
            Self::Cut(cut_size) => (card - cut_size).rem_euclid(len),
            Self::DealWithIncrement(period) => (card * period).rem_euclid(len),
            Self::DealAndCut(period, cut_size) => (card * period - cut_size).rem_euclid(len),
            Self::DealIntoNewStack => len - card - 1,
            Self::Noop => card,
        }
    }

    /*
    fn card_at(&self, index: i128, len: i128) -> i128 {
        match self {
            &Self::Cut(cut_size) => (index + cut_size) % len,
            &Self::Cut(-cut_size) => (index + len - cut_size) % len,
            &Self::DealWithIncrement(period) => {
                // brute-force solution for card given index, period, and len
                // index == card * period % len
                (0..period)
                    .map(|i| i * len + index)
                    .find(|i| i % period == 0)
                    .unwrap()
                    / period
            }
            Self::DealIntoNewStack => len - index - 1,
            Self::Noop => index,
        }
    }
    */

    fn fold(self, other: Self, len: i128) -> (Self, Option<Self>) {
        (
            match (self, other) {
                (Self::Cut(a), Self::Cut(b)) if a == b => Self::Noop,
                (Self::Cut(a), Self::Cut(b)) => Self::Cut(a + b),
                (Self::Cut(a), Self::DealAndCut(b, c)) => Self::DealAndCut(b, (a * b) % len + c),
                (Self::DealWithIncrement(a), Self::Cut(b)) => Self::DealAndCut(a, b),
                (Self::DealWithIncrement(a), Self::DealWithIncrement(b)) => {
                    Self::DealWithIncrement(a * b)
                }
                (Self::DealWithIncrement(a), Self::DealAndCut(b, c)) => Self::DealAndCut(a * b, c),
                (Self::DealAndCut(a, b), Self::Cut(c)) => Self::DealAndCut(a, b + c),
                (Self::DealAndCut(a, b), Self::DealAndCut(c, d)) => {
                    Self::DealAndCut((a * c) % len, (b * c) % len + d)
                }
                (Self::DealIntoNewStack, Self::Cut(a)) => {
                    return (Self::Cut(-a), Some(Self::DealIntoNewStack));
                }
                (Self::DealIntoNewStack, Self::DealWithIncrement(a)) => {
                    return (Self::DealAndCut(-a, a), None);
                }
                (Self::DealIntoNewStack, Self::DealIntoNewStack) => Self::Noop,
                (Self::Noop, a) | (a, Self::Noop) => a,
                (a, b) => return (a, Some(b)),
            },
            None,
        )
    }
}

impl FromStr for Instruction {
    type Err = String;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let input = input.trim_end();

        let (prefix, number) = input
            .find(|c: char| c.is_ascii_digit() || c == '-')
            .map_or_else(|| (input, ""), |i| input.split_at(i));
        let number: Option<i128> = if number == "" {
            None
        } else {
            number.parse().ok()
        };

        match (prefix, number) {
            ("cut ", Some(i)) => Ok(Self::Cut(i)),
            ("deal with increment ", Some(i)) => Ok(Self::DealWithIncrement(i)),
            ("deal into new stack", None) => Ok(Self::DealIntoNewStack),
            _ => Err(format!("Invalid input: {:?}", input)),
        }
    }
}

impl fmt::Display for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        match self {
            Self::Cut(i) | Self::DealAndCut(1, i) => write!(f, "cut {i}"),
            Self::DealWithIncrement(i) | Self::DealAndCut(i, 0) => {
                write!(f, "deal with increment {i}")
            }
            Self::DealIntoNewStack => write!(f, "deal into new stack"),
            Self::DealAndCut(deal, cut) => write!(f, "deal with increment {deal} and cut {cut}"),
            Self::Noop => write!(f, "noop"),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    fn deck_from_positions(instructions: &[Instruction]) -> Vec<i128> {
        let mut deck: Vec<i128> = std::iter::repeat(0).take(10).collect();
        (0..10).for_each(|i| deck[card_position(instructions, i, 10) as usize] = i);
        deck
    }

    #[test]
    fn instructions() {
        assert_eq!(
            vec![9, 8, 7, 6, 5, 4, 3, 2, 1, 0],
            deck_from_positions(&[Instruction::DealIntoNewStack])
        );

        assert_eq!(
            vec![3, 4, 5, 6, 7, 8, 9, 0, 1, 2],
            deck_from_positions(&[Instruction::Cut(3)])
        );

        assert_eq!(
            vec![6, 7, 8, 9, 0, 1, 2, 3, 4, 5],
            deck_from_positions(&[Instruction::Cut(-4)])
        );

        assert_eq!(
            vec![0, 7, 4, 1, 8, 5, 2, 9, 6, 3],
            deck_from_positions(&[Instruction::DealWithIncrement(3)])
        );

        assert_eq!(
            vec![9, 8, 7, 6, 5, 4, 3, 2, 1, 0],
            deck_from_positions(&[Instruction::DealAndCut(-1, 1)])
        );
    }

    #[test]
    fn part1_examples() {
        assert_eq!(
            vec![0, 3, 6, 9, 2, 5, 8, 1, 4, 7],
            deck_from_positions(&parse(include_str!("test1.txt")).unwrap())
        );

        assert_eq!(
            vec![3, 0, 7, 4, 1, 8, 5, 2, 9, 6],
            deck_from_positions(&parse(include_str!("test2.txt")).unwrap())
        );

        assert_eq!(
            vec![6, 3, 0, 7, 4, 1, 8, 5, 2, 9],
            deck_from_positions(&parse(include_str!("test3.txt")).unwrap())
        );

        assert_eq!(
            vec![9, 2, 5, 8, 1, 4, 7, 0, 3, 6],
            deck_from_positions(&parse(include_str!("test4.txt")).unwrap())
        );
    }

    /*
    #[test]
    fn part2_instructions() {
        let run_instruction = |instruction: &str| {
            (0..10)
                .map(|i| parse(instruction).unwrap()[0].card_at(i, 10))
                .collect::<Vec<_>>()
        };

        assert_eq!(
            vec![9, 8, 7, 6, 5, 4, 3, 2, 1, 0],
            run_instruction("deal into new stack"),
        );

        assert_eq!(vec![3, 4, 5, 6, 7, 8, 9, 0, 1, 2], run_instruction("cut 3"));

        assert_eq!(
            vec![6, 7, 8, 9, 0, 1, 2, 3, 4, 5],
            run_instruction("cut -4"),
        );

        assert_eq!(
            vec![0, 7, 4, 1, 8, 5, 2, 9, 6, 3],
            run_instruction("deal with increment 3"),
        );

        assert_eq!(
            vec![0, 5, 10, 1, 6, 11, 2, 7, 12, 3, 8, 13, 4, 9],
            (0..14)
                .map(|i| Instruction::DealWithIncrement(3).card_at(i, 14))
                .collect::<Vec<_>>(),
        );
    }
    */

    #[test]
    fn part1_solution() {
        assert_eq!(Ok(6638), part1(include_str!("input.txt")));
        /*
        assert_eq!(
            2019,
            card_at(&parse(include_str!("input.txt")).unwrap(), 6638, 10007)
        );
        */
    }

    /*
    #[test]
    fn part2_example1() {
        let instructions = parse(include_str!("test1.txt")).unwrap();
        assert_eq!(
            vec![0, 3, 6, 9, 2, 5, 8, 1, 4, 7],
            (0..10)
                .map(|i| card_at(&instructions, i, 10))
                .collect::<Vec<_>>(),
        );
    }

    #[test]
    fn part2_example2() {
        let instructions = parse(include_str!("test2.txt")).unwrap();
        assert_eq!(
            vec![3, 0, 7, 4, 1, 8, 5, 2, 9, 6],
            (0..10)
                .map(|i| card_at(&instructions, i, 10))
                .collect::<Vec<_>>(),
        );
    }

    #[test]
    fn part2_example3() {
        let instructions = parse(include_str!("test3.txt")).unwrap();
        assert_eq!(
            vec![6, 3, 0, 7, 4, 1, 8, 5, 2, 9],
            (0..10)
                .map(|i| card_at(&instructions, i, 10))
                .collect::<Vec<_>>(),
        );
    }

    #[test]
    fn part2_example4() {
        let instructions = parse(include_str!("test4.txt")).unwrap();
        assert_eq!(
            vec![9, 2, 5, 8, 1, 4, 7, 0, 3, 6],
            (0..10)
                .map(|i| card_at(&instructions, i, 10))
                .collect::<Vec<_>>(),
        );
    }
    */

    #[test]
    #[ignore]
    fn part2_solution() {
        assert!(part2(include_str!("input.txt")).unwrap() > 53660045266244);
        //assert_eq!(Ok(0), part2(include_str!("input.txt")));
    }

    #[test]
    fn loops_on_prime_numbers() {
        let instructions = parse(include_str!("input.txt")).unwrap();
        let mut unvisited: std::collections::HashSet<i128> = (0..20021).collect();

        assert_eq!(
            19863,
            (0..4004).fold(19863, |card, i| {
                eprintln!("{i}: {card}");
                unvisited.take(&card).unwrap();
                card_position(&instructions, card, 20021)
            }),
        );

        //assert_eq!(std::collections::HashSet::new(), unvisited);
    }

    /// Fold combinations:
    /// * (Cut, Cut)
    /// * (Cut, DealWithIncrement)
    /// * (Cut, DealIntoNewStack)
    /// * (DealWithIncrement, Cut)
    /// * (DealWithIncrement, DealWithIncrement)
    /// * (DealWithIncrement, DealIntoNewStack)
    /// * (DealIntoNewStack, Cut)
    /// * (DealIntoNewStack, DealWithIncrement)
    /// * (DealIntoNewStack, DealIntoNewStack)
    #[test]
    fn individual_fold_test() {
        const LEN: i128 = 23;

        let instructions = &[
            [Instruction::Cut(2), Instruction::Cut(-5)],
            [Instruction::Cut(2), Instruction::DealWithIncrement(8)],
            [Instruction::Cut(2), Instruction::DealIntoNewStack],
            [Instruction::DealWithIncrement(10), Instruction::Cut(-5)],
            [
                Instruction::DealWithIncrement(10),
                Instruction::DealWithIncrement(8),
            ],
            [
                Instruction::DealWithIncrement(10),
                Instruction::DealIntoNewStack,
            ],
            [Instruction::DealIntoNewStack, Instruction::Cut(-5)],
            [
                Instruction::DealIntoNewStack,
                Instruction::DealWithIncrement(8),
            ],
            [Instruction::DealIntoNewStack, Instruction::DealIntoNewStack],
        ][..];

        for instruction in instructions {
            let sequential_output: Vec<_> = (0..LEN)
                .map(|i| instruction[1].card_position(instruction[0].card_position(i, LEN), LEN))
                .collect();

            let (combined_instruction, combined_output): (String, Vec<_>) =
                match instruction[0].clone().fold(instruction[1].clone(), LEN) {
                    (a, Some(b)) => (
                        format!("({a}, {b})"),
                        (0..LEN)
                            .map(|i| b.card_position(a.card_position(i, LEN), LEN))
                            .collect(),
                    ),
                    (a, None) => (
                        format!("{a}"),
                        (0..LEN).map(|i| a.card_position(i, LEN)).collect(),
                    ),
                };

            assert_eq!(
                sequential_output, combined_output,
                "({}, {}) => {}",
                instruction[0], instruction[1], combined_instruction,
            );
        }
    }

    #[test]
    fn part1_fold_test() {
        const LEN: i128 = 10007;

        let instructions = parse(include_str!("input.txt")).unwrap();
        let folded_instructions = fold(instructions.clone(), LEN);

        eprintln!("Instructions ({}):", instructions.len());
        eprintln!("Folded instructions ({}):", folded_instructions.len());
        for i in 0..instructions.len() {
            let first_column = format!("  {}", instructions[i]);
            eprint!("{first_column}");

            if let Some(folded_instruction) = folded_instructions.get(i) {
                (0..30 - first_column.len()).for_each(|_| eprint!(" "));
                eprintln!("{folded_instruction}");
            } else {
                eprintln!("");
            }
        }

        assert_eq!(6638, card_position(&instructions, 2019, LEN));
        assert_eq!(6638, card_position(&folded_instructions, 2019, LEN));
        panic!();
    }
}
