use std::fmt;
use std::str::FromStr;

pub fn part1(input: &str) -> Result<u64, String> {
    let deck_len = 10007i128;

    let instructions = parse(input)?;

    Ok(card_position(&instructions, 2019, deck_len) as u64)
}

pub fn part2(input: &str) -> Result<u64, String> {
    const DECK_LEN: u128 = 119_315_717_514_047;
    const SHUFFLES: u128 = 101_741_582_076_661;
    const REVERSED_SHUFFLES: u128 = DECK_LEN - SHUFFLES - 1;
    const CARD_INDEX: u128 = 2020;

    let instruction = fold(parse(input)?, DECK_LEN as i128);

    let card_index =
        instruction.card_position_n(CARD_INDEX as i128, DECK_LEN as i128, REVERSED_SHUFFLES);

    Ok(card_index as u64)
}

fn card_position(instructions: &[Instruction], card: i128, deck_len: i128) -> i128 {
    assert!(card < deck_len, "No card of value {} in deck", card);
    instructions.into_iter().fold(card, |card, instruction| {
        instruction.card_position(card, deck_len)
    })
}

fn fold(instructions: Vec<Instruction>, len: i128) -> DealAndCut {
    instructions
        .into_iter()
        .fold(DealAndCut::default(), |acc, instruction| {
            acc.combine(&instruction.into(), len)
        })
}

fn parse(input: &str) -> Result<Vec<Instruction>, String> {
    input.lines().map(|line| line.parse()).collect()
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct DealAndCut {
    times: i128,
    plus: i128,
}

#[derive(Clone, Debug, Eq, PartialEq)]
enum Instruction {
    Cut(i128),
    DealWithIncrement(i128),
    DealIntoNewStack,
}

impl Instruction {
    fn card_position(&self, card: i128, len: i128) -> i128 {
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
    fn combine(&self, other: &Self, len: i128) -> Self {
        Self {
            times: (self.times * other.times) % len,
            plus: (self.plus * other.times + other.plus) % len,
        }
    }

    fn double(&self, len: i128) -> Self {
        self.combine(self, len)
    }

    fn card_position(&self, card: i128, len: i128) -> i128 {
        (card * self.times - self.plus).rem_euclid(len)
    }

    fn card_position_n(&self, card: i128, len: i128, iterations: u128) -> i128 {
        if iterations == 1 {
            return self.card_position(card, len);
        }

        let max_bit = iterations.ilog2() as usize;
        let mut memo = Vec::with_capacity(max_bit as usize + 1);
        memo.push(self.clone());
        (1..=max_bit).for_each(|_| memo.push(memo.last().unwrap().double(len)));

        eprintln!("iterations: {iterations} ({iterations:b})");
        eprintln!("max_bit: {max_bit}");

        (0..=max_bit)
            .filter(|i| iterations & 1 << i != 0)
            .map(|i| (i, &memo[i]))
            .inspect(|(i, instruction)| eprintln!("Bit {i}: {instruction}"))
            .fold(DealAndCut::default(), |acc, (_, instruction)| {
                acc.combine(&instruction, len)
            })
            .card_position(card, len)
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
        assert!(part2(include_str!("input.txt")).unwrap() < 90739407010994);
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
        const LEN: i128 = 10007;

        let instructions = parse(include_str!("input.txt")).unwrap();
        let folded_instruction = fold(instructions.clone(), LEN);

        assert_eq!(6638, card_position(&instructions, 2019, LEN));
        assert_eq!(6638, folded_instruction.card_position(2019, LEN));
    }

    #[test]
    fn deal_and_cut_default_test() {
        const LEN: i128 = 10007;

        assert_eq!(2019, DealAndCut::default().card_position(2019, LEN));
    }

    #[test]
    fn card_position_n_test() {
        const LEN: i128 = 10007;

        let instruction = fold(parse(include_str!("input.txt")).unwrap(), LEN);

        let mut value = 2020;

        for i in 1..10 {
            value = instruction.card_position(value, LEN);
            assert_eq!(value, instruction.card_position_n(2020, LEN, i), "{}", i);
        }
    }
}
