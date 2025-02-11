use std::collections::HashMap;
use std::fmt;
use std::str::FromStr;

pub fn part1(input: &str) -> Result<u64, String> {
    let instructions = parse(input)?;

    Ok(card_position(&instructions, 2019, 10007))
}

pub fn part2(input: &str) -> Result<u64, String> {
    let instructions = parse(input)?;

    let deck_size = 119315717514047u64;
    let shuffles = 101741582076661u64;
    let reversed_shuffles = deck_size - shuffles;

    let mut card_index = 2020;

    for shuffle in 0..reversed_shuffles {
        card_index = card_position(&instructions, card_index, deck_size);

        if shuffle % 100000 == 0 {
            eprintln!("Shuffle {shuffle} of {reversed_shuffles}: {card_index}",);
        }
    }

    Ok(card_index)

    /*
    Ok(card_position(&instructions, 2020, deck_size))

    let mut known_positions = HashMap::new();
    for shuffle in 0..shuffles {
        if known_positions.insert(card_index, shuffle).is_some() {
            panic!(
                "Periodicity at {} -> {}, {}!",
                prev_card_index, card_index, shuffle
            );
        }
        prev_card_index = card_index;
        card_index = card_at(&instructions, card_index, deck_size);

        if shuffle % 10000 == 0 {
            eprintln!(
                "Shuffle {shuffle}: {card_index} (len: {})",
                known_positions.len(),
            );

            known_positions
                .iter()
                .take(3)
                .for_each(|(k, v)| eprintln!("  {k}: {v}"));
        }
    }
    */
}

fn card_position(instructions: &[Instruction<u64>], card: u64, deck_size: u64) -> u64 {
    assert!(card < deck_size, "No card of value {} in deck", card);
    instructions.into_iter().fold(card, |card, instruction| {
        instruction.card_position(card, deck_size)
    })
}

fn card_at(instructions: &[Instruction<u64>], position: u64, deck_size: u64) -> u64 {
    assert!(position < deck_size, "No card at position {}", position);
    instructions
        .into_iter()
        .rev()
        .fold(position, |position, instruction| {
            instruction.card_at(position, deck_size)
        })
}

fn parse<T: FromStr>(input: &str) -> Result<Vec<Instruction<T>>, String> {
    input.lines().map(|line| line.parse()).collect()
}

#[derive(Debug)]
enum Instruction<T: FromStr> {
    CutLeft(T),
    CutRight(T),
    DealWithIncrement(T),
    DealIntoNewStack,
}

impl Instruction<u64> {
    fn card_position(&self, card: u64, len: u64) -> u64 {
        match self {
            &Instruction::CutLeft(i) if card < i => len - i + card,
            Instruction::CutLeft(i) => card - i,
            &Instruction::CutRight(i) if card >= len - i => card - (len - i),
            Instruction::CutRight(i) => card + i,
            &Instruction::DealWithIncrement(period) => card * period % len,
            Instruction::DealIntoNewStack => len - card - 1,
        }
    }

    fn card_at(&self, index: u64, len: u64) -> u64 {
        match self {
            &Instruction::CutLeft(i) if index >= len - i => index - (len - i),
            Instruction::CutLeft(i) => index + i,
            &Instruction::CutRight(i) if index < i => len - i + index,
            Instruction::CutRight(i) => index - i,
            &Instruction::DealWithIncrement(period) => {
                // brute-force solution for card given index, period, and len
                // index == card * period % len
                (0..period)
                    .map(|i| i * len + index)
                    .find(|i| i % period == 0)
                    .unwrap()
                    / period
            }
            Instruction::DealIntoNewStack => len - index - 1,
        }
    }
}

impl<T: FromStr> FromStr for Instruction<T> {
    type Err = String;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let input = input.trim_end();

        let (prefix, number) = input
            .find(|c: char| c.is_ascii_digit())
            .map_or_else(|| (input, ""), |i| input.split_at(i));
        let number: Option<T> = if number == "" {
            None
        } else {
            number.parse().ok()
        };

        match (prefix, number) {
            ("cut -", Some(i)) => Ok(Self::CutRight(i)),
            ("cut ", Some(i)) => Ok(Self::CutLeft(i)),
            ("deal with increment ", Some(i)) => Ok(Self::DealWithIncrement(i)),
            ("deal into new stack", None) => Ok(Self::DealIntoNewStack),
            _ => Err(format!("Invalid input: {:?}", input)),
        }
    }
}

impl<T: fmt::Display + FromStr> fmt::Display for Instruction<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        match self {
            Self::CutLeft(i) => write!(f, "cut {i}"),
            Self::CutRight(i) => write!(f, "cut -{i}"),
            Self::DealWithIncrement(i) => write!(f, "deal with increment {i}"),
            Self::DealIntoNewStack => write!(f, "deal into new stack"),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    fn deck_from_positions(instructions: &[Instruction<u64>]) -> Vec<u64> {
        let mut deck: Vec<u64> = std::iter::repeat(0).take(10).collect();
        (0..10).for_each(|i| deck[card_position(instructions, i, 10) as usize] = i);
        deck
    }

    #[test]
    fn part1_instructions() {
        assert_eq!(
            vec![9, 8, 7, 6, 5, 4, 3, 2, 1, 0],
            deck_from_positions(&[Instruction::DealIntoNewStack])
        );

        assert_eq!(
            vec![3, 4, 5, 6, 7, 8, 9, 0, 1, 2],
            deck_from_positions(&[Instruction::CutLeft(3)])
        );

        assert_eq!(
            vec![6, 7, 8, 9, 0, 1, 2, 3, 4, 5],
            deck_from_positions(&[Instruction::CutRight(4)])
        );

        assert_eq!(
            vec![0, 7, 4, 1, 8, 5, 2, 9, 6, 3],
            deck_from_positions(&[Instruction::DealWithIncrement(3)])
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

    #[test]
    fn part1_solution() {
        assert_eq!(Ok(6638), part1(include_str!("input.txt")));
        assert_eq!(
            2019,
            card_at(&parse(include_str!("input.txt")).unwrap(), 6638, 10007)
        );
    }

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

    #[test]
    #[ignore]
    fn part2_solution() {
        assert!(part2(include_str!("input.txt")).unwrap() > 53660045266244);
        //assert_eq!(Ok(0), part2(include_str!("input.txt")));
    }
}
