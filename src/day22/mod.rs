use std::fmt;
use std::str::FromStr;

pub fn part1(input: &str) -> Result<u64, String> {
    let deck_len = 10007i64;

    let instructions = parse(input)?;

    Ok(card_position(&instructions, 2019, deck_len) as u64)
}

pub fn part2(input: &str) -> Result<u64, String> {
    let deck_len = 119_315_717_514_047i64;
    let shuffles = 101_741_582_076_661i64;
    let reversed_shuffles = deck_len - shuffles - 1;

    let instruction = fold(parse(input)?, deck_len);

    let mut card_index = 2020;

    for shuffle in 0..reversed_shuffles {
        if shuffle % 1_000_000 == 0 {
            eprintln!(
                "Shuffle {shuffle} of {reversed_shuffles} ({}%): {card_index} ({}%)",
                shuffle * 100 / reversed_shuffles,
                card_index * 100 / deck_len,
            );
        }

        card_index = card_position_single(&instruction, card_index, deck_len);
    }

    Ok(card_index as u64)
}

fn card_position(instructions: &[Instruction], card: i64, deck_len: i64) -> i64 {
    assert!(card < deck_len, "No card of value {} in deck", card);
    instructions.into_iter().fold(card, |card, instruction| {
        instruction.card_position(card, deck_len)
    })
}

fn card_position_single(instruction: &DealAndCut, card: i64, deck_len: i64) -> i64 {
    assert!(card < deck_len, "No card of value {} in deck", card);
    instruction.card_position(card, deck_len)
}

fn fold(instructions: Vec<Instruction>, len: i64) -> DealAndCut {
    instructions
        .into_iter()
        .fold(DealAndCut::default(), |acc, instruction| {
            acc.combine(instruction.into(), len)
        })
}

fn parse(input: &str) -> Result<Vec<Instruction>, String> {
    input.lines().map(|line| line.parse()).collect()
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct DealAndCut {
    times: i64,
    plus: i64,
}

#[derive(Clone, Debug, Eq, PartialEq)]
enum Instruction {
    Cut(i64),
    DealWithIncrement(i64),
    DealIntoNewStack,
}

impl Instruction {
    fn card_position(&self, card: i64, len: i64) -> i64 {
        match self {
            Self::Cut(cut_size) => (card - cut_size).rem_euclid(len),
            Self::DealWithIncrement(period) => (card * period).rem_euclid(len),
            Self::DealIntoNewStack => len - card - 1,
        }
    }
}

impl From<Instruction> for DealAndCut {
    fn from(instruction: Instruction) -> DealAndCut {
        match instruction {
            Instruction::Cut(plus) => DealAndCut { times: 1, plus },
            Instruction::DealWithIncrement(times) => DealAndCut { times, plus: 0 },
            Instruction::DealIntoNewStack => DealAndCut { times: -1, plus: 1 },
        }
    }
}

impl DealAndCut {
    fn combine(self, other: Self, len: i64) -> Self {
        Self {
            times: (self.times * other.times) % len,
            plus: (self.plus * other.times) % len + other.plus,
        }
    }

    fn card_position(&self, card: i64, len: i64) -> i64 {
        (card * self.times - self.plus).rem_euclid(len)
    }
}

impl Default for DealAndCut {
    fn default() -> Self {
        Self { times: 1, plus: 0 }
    }
}

impl FromStr for Instruction {
    type Err = String;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let input = input.trim_end();

        let (prefix, number) = input
            .find(|c: char| c.is_ascii_digit() || c == '-')
            .map_or_else(|| (input, ""), |i| input.split_at(i));
        let number: Option<i64> = if number == "" {
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
            Self::Cut(i) => write!(f, "cut {i}"),
            Self::DealWithIncrement(i) => write!(f, "deal with increment {i}"),
            Self::DealIntoNewStack => write!(f, "deal into new stack"),
        }
    }
}

impl fmt::Display for DealAndCut {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(
            f,
            "deal with increment {} and cut {}",
            self.times, self.plus
        )
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
    fn part1_solution() {
        assert_eq!(Ok(6638), part1(include_str!("input.txt")));
    }

    #[test]
    #[ignore]
    fn part2_solution() {
        assert!(part2(include_str!("input.txt")).unwrap() > 53660045266244);
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
        const LEN: i64 = 23;

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

            let (combined_instruction, combined_output): (String, Vec<_>) = {
                let combined = fold(instruction.iter().cloned().collect(), LEN);

                (
                    format!("{combined}"),
                    (0..LEN).map(|i| combined.card_position(i, LEN)).collect(),
                )
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
        const LEN: i64 = 10007;

        let instructions = parse(include_str!("input.txt")).unwrap();
        let folded_instructions = fold(instructions.clone(), LEN);

        assert_eq!(6638, card_position(&instructions, 2019, LEN));
        assert_eq!(6638, card_position_single(&folded_instructions, 2019, LEN));
    }
}
