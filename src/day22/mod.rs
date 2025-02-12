use std::cmp::Ordering;
use std::collections::HashMap;
use std::fmt;
use std::str::FromStr;

pub fn part1(input: &str) -> Result<u64, String> {
    let instructions = parse(input)?;

    Ok(card_position(&instructions, 2019, 10007) as u64)
}

pub fn part2(input: &str) -> Result<u64, String> {
    let instructions = parse(input)?;

    //let deck_size = 119315717514047i64;
    let deck_size = 20021i64;
    let shuffles = 20020i64;
    //let reversed_shuffles = deck_size - shuffles;
    let reversed_shuffles = shuffles;

    let mut card_index = 2020;
    let mut visited = HashMap::new();

    for shuffle in 0..reversed_shuffles {
        if let Some(_prev) = visited.insert(card_index, shuffle) {
            panic!("Repeated at {} after {} shuffles", card_index, shuffle);
        }

        card_index = card_position(&instructions, card_index, deck_size);
        //card_index = card_at(&instructions, card_index, deck_size);

        if shuffle % 100000 == 0 {
            eprintln!("Shuffle {shuffle} of {reversed_shuffles}: {card_index}");
        }
    }

    Ok(card_index as u64)
}

fn card_position(instructions: &[Instruction], card: i64, deck_size: i64) -> i64 {
    assert!(card < deck_size, "No card of value {} in deck", card);
    instructions.into_iter().fold(card, |card, instruction| {
        instruction.card_position(card, deck_size)
    })
}

/*
fn card_at(instructions: &[Instruction], position: i64, deck_size: i64) -> i64 {
    assert!(position < deck_size, "No card at position {}", position);
    instructions
        .into_iter()
        .rev()
        .fold(position, |position, instruction| {
            instruction.card_at(position, deck_size)
        })
}
*/

fn fold(mut instructions: Vec<Instruction>) -> Vec<Instruction> {
    if instructions.is_empty() {
        return Vec::new();
    }

    loop {
        let folded_instructions: Vec<_> = instructions[1..].iter().cloned().fold(
            vec![instructions[0].clone()],
            |mut acc, instruction| {
                let prev = acc.pop().unwrap();
                let (a, b) = prev.fold(instruction);
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
    CutLeft(i64),
    CutRight(i64),
    DealWithIncrement(i64),
    DealAndCut(i64, i64),
    DealIntoNewStack,
}

impl Instruction {
    fn card_position(&self, card: i64, len: i64) -> i64 {
        match self {
            Self::CutLeft(cut_size) => (card + len - cut_size) % len,
            Self::CutRight(cut_size) => (card + cut_size) % len,
            Self::DealWithIncrement(period) => card * period % len,
            Self::DealAndCut(period, cut_size) => (card * period + cut_size) % len,
            Self::DealIntoNewStack => len - card - 1,
            Self::Noop => card,
        }
    }

    /*
    fn card_at(&self, index: i64, len: i64) -> i64 {
        match self {
            &Self::CutLeft(cut_size) => (index + cut_size) % len,
            &Self::CutRight(cut_size) => (index + len - cut_size) % len,
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

    /// Fold combinations:
    /// * [x] (CutLeft, CutLeft)
    /// * [x] (CutLeft, CutRight)
    /// * [ ] (CutLeft, DealWithIncrement)
    /// * [ ] (CutLeft, DealIntoNewStack)
    /// * [x] (CutRight, CutLeft)
    /// * [x] (CutRight, CutRight)
    /// * [ ] (CutRight, DealWithIncrement)
    /// * [ ] (CutRight, DealIntoNewStack)
    /// * [ ] (DealWithIncrement, CutLeft)
    /// * [ ] (DealWithIncrement, CutRight)
    /// * [x] (DealWithIncrement, DealWithIncrement)
    /// * [ ] (DealWithIncrement, DealIntoNewStack)
    /// * [ ] (DealIntoNewStack, CutLeft)
    /// * [ ] (DealIntoNewStack, CutRight)
    /// * [ ] (DealIntoNewStack, DealWithIncrement)
    /// * [x] (DealIntoNewStack, DealIntoNewStack)
    fn fold(self, other: Self) -> (Self, Option<Self>) {
        (
            match (self, other) {
                (Self::CutLeft(a), Self::CutLeft(b)) => Self::CutLeft(a + b),
                (Self::CutRight(a), Self::CutRight(b)) => Self::CutRight(a + b),
                (Self::CutLeft(a), Self::CutRight(b)) | (Self::CutRight(b), Self::CutLeft(a)) => {
                    match a.cmp(&b) {
                        Ordering::Less => Self::CutRight(b - a),
                        Ordering::Equal => Self::Noop,
                        Ordering::Greater => Self::CutLeft(a - b),
                    }
                }
                (Self::DealWithIncrement(a), Self::DealWithIncrement(b)) => {
                    Self::DealWithIncrement(a * b)
                }
                (Self::DealIntoNewStack, Self::CutLeft(a)) => {
                    return (Self::CutRight(a), Some(Self::DealIntoNewStack))
                }
                (Self::DealIntoNewStack, Self::CutRight(a)) => {
                    return (Self::CutLeft(a), Some(Self::DealIntoNewStack))
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
            .find(|c: char| c.is_ascii_digit())
            .map_or_else(|| (input, ""), |i| input.split_at(i));
        let number: Option<i64> = if number == "" {
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

impl fmt::Display for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        match self {
            Self::CutLeft(i) | Self::DealAndCut(1, i) => write!(f, "cut {i}"),
            Self::CutRight(i) => write!(f, "cut -{i}"),
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

    fn deck_from_positions(instructions: &[Instruction]) -> Vec<i64> {
        let mut deck: Vec<i64> = std::iter::repeat(0).take(10).collect();
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
        let mut unvisited: std::collections::HashSet<i64> = (0..20021).collect();

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
    /// * [x] (CutLeft, CutLeft)
    /// * [x] (CutLeft, CutRight)
    /// * [ ] (CutLeft, DealWithIncrement)
    /// * [ ] (CutLeft, DealIntoNewStack)
    /// * [x] (CutRight, CutLeft)
    /// * [x] (CutRight, CutRight)
    /// * [ ] (CutRight, DealWithIncrement)
    /// * [ ] (CutRight, DealIntoNewStack)
    /// * [ ] (DealWithIncrement, CutLeft)
    /// * [ ] (DealWithIncrement, CutRight)
    /// * [x] (DealWithIncrement, DealWithIncrement)
    /// * [ ] (DealWithIncrement, DealIntoNewStack)
    /// * [ ] (DealIntoNewStack, CutLeft)
    /// * [ ] (DealIntoNewStack, CutRight)
    /// * [ ] (DealIntoNewStack, DealWithIncrement)
    /// * [x] (DealIntoNewStack, DealIntoNewStack)
    #[test]
    fn individual_fold_test() {
        let instructions = &[
            [Instruction::CutLeft(2), Instruction::CutLeft(5)],
            [Instruction::CutLeft(2), Instruction::CutRight(5)],
            [Instruction::CutLeft(2), Instruction::DealWithIncrement(8)],
            [Instruction::CutLeft(2), Instruction::DealIntoNewStack],
            [Instruction::CutRight(2), Instruction::CutLeft(5)],
            [Instruction::CutRight(2), Instruction::CutRight(5)],
            [Instruction::CutRight(2), Instruction::DealWithIncrement(8)],
            [Instruction::CutRight(2), Instruction::DealIntoNewStack],
            [Instruction::DealWithIncrement(10), Instruction::CutLeft(5)],
            [Instruction::DealWithIncrement(10), Instruction::CutRight(5)],
            [
                Instruction::DealWithIncrement(10),
                Instruction::DealWithIncrement(8),
            ],
            [
                Instruction::DealWithIncrement(10),
                Instruction::DealIntoNewStack,
            ],
            [Instruction::DealIntoNewStack, Instruction::CutLeft(5)],
            [Instruction::DealIntoNewStack, Instruction::CutRight(5)],
            [
                Instruction::DealIntoNewStack,
                Instruction::DealWithIncrement(8),
            ],
            [Instruction::DealIntoNewStack, Instruction::DealIntoNewStack],
        ][..];

        for instruction in instructions {
            let sequential_output: Vec<_> = (0..23)
                .map(|i| instruction[1].card_position(instruction[0].card_position(i, 23), 23))
                .collect();

            let (combined_instruction, combined_output): (String, Vec<_>) =
                match instruction[0].clone().fold(instruction[1].clone()) {
                    (a, Some(b)) => (
                        format!("({a}, {b})"),
                        (0..23)
                            .map(|i| b.card_position(a.card_position(i, 23), 23))
                            .collect(),
                    ),
                    (a, None) => (
                        format!("{a}"),
                        (0..23).map(|i| a.card_position(i, 23)).collect(),
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
        let instructions = parse(include_str!("input.txt")).unwrap();
        let folded_instructions = fold(instructions.clone());

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

        panic!();
        assert_eq!(6638, card_position(&instructions, 2019, 10007));
    }
}
