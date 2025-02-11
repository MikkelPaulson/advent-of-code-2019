use std::collections::HashMap;
use std::fmt;
use std::mem;
use std::str::FromStr;

pub fn part1(input: &str) -> Result<u64, String> {
    let cards = deal_with_deck_size(&parse(input)?, 10007);

    cards
        .iter()
        .enumerate()
        .find_map(|(i, &card)| if card == 2019 { Some(i as u64) } else { None })
        .ok_or_else(|| "No card found with value 2019.".to_string())
}

pub fn part2(input: &str) -> Result<u64, String> {
    let instructions = parse(input)?;

    let deck_size = 119315717514047u64;
    let shuffles = 101741582076661u64;
    let mut card_index = 2020;
    let mut known_positions = HashMap::new();

    for shuffle in 0..shuffles {
        if known_positions.insert(card_index, shuffle).is_some() {
            panic!("Periodicity at {}, {}!", card_index, shuffle);
        }
        card_index = card_at(&instructions, card_index, deck_size);
        if shuffle % 10000 == 0 {
            eprintln!(
                "Shuffle {shuffle}: {card_index} (len: {})",
                known_positions.len()
            );

            known_positions
                .iter()
                .take(3)
                .for_each(|(k, v)| eprintln!("  {k}: {v}"));
        }
    }

    todo!();
}

fn deal_with_deck_size(instructions: &[Instruction<usize>], deck_size: u16) -> Vec<u16> {
    let mut deck = Deck::new(deck_size);

    instructions
        .iter()
        .for_each(|instruction| deck.evaluate(instruction));

    deck.cards
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

struct Deck {
    pub cards: Vec<u16>,
}

impl Deck {
    pub fn new(size: u16) -> Self {
        let mut cards = Vec::with_capacity(size as usize);
        for i in 0..size {
            cards.push(i);
        }
        Self { cards }
    }

    pub fn evaluate(&mut self, instruction: &Instruction<usize>) {
        println!("Evaluating instruction: {:?}", instruction);

        match instruction {
            Instruction::CutLeft(i) => self.cut_left(*i),
            Instruction::CutRight(i) => self.cut_right(*i),
            Instruction::DealWithIncrement(i) => self.deal_with_increment(*i),
            Instruction::DealIntoNewStack => self.deal_into_new_stack(),
        }
    }

    fn cut_left(&mut self, index: usize) {
        let mut remainder = self.cards.split_off(index);
        mem::swap(&mut self.cards, &mut remainder);
        self.cards.append(&mut remainder);
    }

    fn cut_right(&mut self, index: usize) {
        let mut remainder = self.cards.split_off(self.cards.len() - index);
        mem::swap(&mut self.cards, &mut remainder);
        self.cards.append(&mut remainder);
    }

    fn deal_with_increment(&mut self, step: usize) {
        let len = self.cards.len();
        let mut cards: Vec<u16> = (0..len).map(|_| u16::MAX).collect();

        self.cards
            .drain(..)
            .enumerate()
            .for_each(|(i, card)| cards[i * step % len] = card);

        self.cards = cards;
    }

    fn deal_into_new_stack(&mut self) {
        let mut cards = Vec::with_capacity(self.cards.len());

        while let Some(card) = self.cards.pop() {
            cards.push(card);
        }

        self.cards = cards;
    }
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
    fn card_at(&self, index: u64, len: u64) -> u64 {
        match self {
            &Instruction::CutLeft(i) if index >= len - i => index - (len - i),
            Instruction::CutLeft(i) => index + i,
            &Instruction::CutRight(i) if index < i => len - i + index,
            Instruction::CutRight(i) => index - i,
            &Instruction::DealWithIncrement(period) => {
                // 4 % 3 has the same wrapping behaviour as 10 % 3, so run a mini model.
                let model_target_index = index % period;
                let model_skip = len / period;
                let model_len = period + len % period;

                let mut model_index = 0u64;
                let mut model_result = 0u64;

                loop {
                    // A hit on this iteration!
                    if model_target_index == model_index {
                        break model_result + (index - model_target_index) / period;
                    }

                    model_index += period;
                    model_result += model_skip;
                    if model_index < model_len {
                        model_index += period;
                        model_result += 1;
                    }
                    model_index %= model_len;
                }
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

    #[test]
    fn part1_instructions() {
        let apply_with_deck = |instruction: &Instruction<usize>| {
            let mut deck = Deck::new(10);
            deck.evaluate(instruction);
            deck.cards
        };

        assert_eq!(
            vec![9, 8, 7, 6, 5, 4, 3, 2, 1, 0],
            apply_with_deck(&Instruction::DealIntoNewStack)
        );

        assert_eq!(
            vec![3, 4, 5, 6, 7, 8, 9, 0, 1, 2],
            apply_with_deck(&Instruction::CutLeft(3))
        );

        assert_eq!(
            vec![6, 7, 8, 9, 0, 1, 2, 3, 4, 5],
            apply_with_deck(&Instruction::CutRight(4))
        );

        assert_eq!(
            vec![0, 7, 4, 1, 8, 5, 2, 9, 6, 3],
            apply_with_deck(&Instruction::DealWithIncrement(3))
        );
    }

    #[test]
    fn part1_examples() {
        assert_eq!(
            vec![0, 3, 6, 9, 2, 5, 8, 1, 4, 7],
            deal_with_deck_size(&parse(include_str!("test1.txt")).unwrap(), 10)
        );

        assert_eq!(
            vec![3, 0, 7, 4, 1, 8, 5, 2, 9, 6],
            deal_with_deck_size(&parse(include_str!("test2.txt")).unwrap(), 10)
        );

        assert_eq!(
            vec![6, 3, 0, 7, 4, 1, 8, 5, 2, 9],
            deal_with_deck_size(&parse(include_str!("test3.txt")).unwrap(), 10)
        );

        assert_eq!(
            vec![9, 2, 5, 8, 1, 4, 7, 0, 3, 6],
            deal_with_deck_size(&parse(include_str!("test4.txt")).unwrap(), 10)
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
        assert_eq!(Ok(0), part2(include_str!("input.txt")));
    }
}
