use std::collections::HashSet;
use std::mem;
use std::str::FromStr;

use super::math;

pub fn part1(input: &str) -> Result<u64, String> {
    let cards = deal_with_deck_size(&parse(input)?, 10007);

    cards
        .iter()
        .enumerate()
        .find_map(|(i, &card)| if card == 2019 { Some(i as u64) } else { None })
        .ok_or_else(|| "No card found with value 2019.".to_string())
}

pub fn part2(input: &str) -> Result<u64, String> {
    // let instructions = parse(input)?;
    let instructions = [Instruction::DealWithIncrement(38)];

    const DECK_SIZE: u64 = 10007;

    for shuffles in 1.. {
        if shuffles % 1000 == 0 {
            println!("{}", shuffles);
        }
        if let Ok(1) = card_at(&instructions, 1, DECK_SIZE, shuffles) {
            let cards: Vec<u64> = (0..10)
                .map(|i| card_at(&instructions, i, DECK_SIZE, shuffles).unwrap())
                .collect();
            println!("{} shuffles: {:?}", shuffles, cards);

            return Ok(shuffles);
        }
    }
    Err("Not found".to_string())

    /*
    let instructions = parse(input)?;
    let deck_size = 119315717514047;
    let shuffles = 101741582076661;

    let period = instructions
        .iter()
        .fold(1, |acc, i| math::lcm(acc, i.get_period(deck_size) as i64));

    println!("Period: {}", period);

    card_at(&instructions, 2020, deck_size, shuffles)
    */
}

fn deal_with_deck_size(instructions: &[Instruction<usize>], deck_size: u16) -> Vec<u16> {
    let mut deck = Deck::new(deck_size);

    instructions
        .iter()
        .for_each(|instruction| deck.evaluate(instruction));

    deck.cards
}

fn card_at(
    instructions: &[Instruction<u64>],
    position: u64,
    deck_size: u64,
    shuffles: u64,
) -> Result<u64, String> {
    //let mut past_values = HashSet::new();

    if position < deck_size {
        Ok((0..shuffles).fold(position, |position, i| {
            let result = instructions.iter().fold(position, |acc, instruction| {
                transform(instruction, acc, deck_size)
            });

            let offset = (deck_size + position - result) % deck_size;
            /*
            if !past_values.insert(offset) {
                println!("{} of {}: {}", i, shuffles, offset);
            }
            */
            result
        }))
    } else {
        Err(format!("No card at position {}", position))
    }
}

fn transform(instruction: &Instruction<u64>, position: u64, deck_size: u64) -> u64 {
    match instruction {
        Instruction::CutLeft(i) => {
            if position >= deck_size - i {
                position - (deck_size - i)
            } else {
                position + i
            }
        }
        Instruction::CutRight(i) => {
            if &position < i {
                deck_size - i + position
            } else {
                position - i
            }
        }
        Instruction::DealWithIncrement(increment) => {
            if position == 0 {
                0
            } else {
                (0..*increment)
                    .find_map(|i| {
                        if (deck_size * i + position) % increment == 0 {
                            Some((deck_size * i + position) / increment)
                        } else {
                            None
                        }
                    })
                    .unwrap()
            }
        }
        Instruction::DealIntoNewStack => deck_size - position - 1,
    }
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
    fn get_period(&self, deck_size: u64) -> u64 {
        match self {
            Self::CutLeft(i) => deck_size / (math::gcd(deck_size as i64, *i as i64) as u64),
            Self::CutRight(i) => deck_size / (math::gcd(deck_size as i64, *i as i64) as u64),
            Self::DealWithIncrement(i) => *i,
            Self::DealIntoNewStack => 2,
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

        let apply_with_transform = |instruction: &Instruction<u64>| {
            let cards: Vec<u64> = (0..10).map(|i| transform(instruction, i, 10)).collect();
            cards
        };

        assert_eq!(
            vec![9, 8, 7, 6, 5, 4, 3, 2, 1, 0],
            apply_with_deck(&Instruction::DealIntoNewStack)
        );
        assert_eq!(
            vec![9, 8, 7, 6, 5, 4, 3, 2, 1, 0],
            apply_with_transform(&Instruction::DealIntoNewStack)
        );

        assert_eq!(
            vec![3, 4, 5, 6, 7, 8, 9, 0, 1, 2],
            apply_with_deck(&Instruction::CutLeft(3))
        );
        assert_eq!(
            vec![3, 4, 5, 6, 7, 8, 9, 0, 1, 2],
            apply_with_transform(&Instruction::CutLeft(3))
        );

        assert_eq!(
            vec![6, 7, 8, 9, 0, 1, 2, 3, 4, 5],
            apply_with_deck(&Instruction::CutRight(4))
        );
        assert_eq!(
            vec![6, 7, 8, 9, 0, 1, 2, 3, 4, 5],
            apply_with_transform(&Instruction::CutRight(4))
        );

        assert_eq!(
            vec![0, 7, 4, 1, 8, 5, 2, 9, 6, 3],
            apply_with_deck(&Instruction::DealWithIncrement(3))
        );
        assert_eq!(
            vec![0, 7, 4, 1, 8, 5, 2, 9, 6, 3],
            apply_with_transform(&Instruction::DealWithIncrement(3))
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
    #[ignore]
    fn part1_solution() {
        assert_eq!(Ok(6638), part1(include_str!("input.txt")));
        assert_eq!(
            Ok(2019),
            card_at(&parse(include_str!("input.txt")).unwrap(), 6638, 10007, 1)
        );
    }

    #[test]
    #[ignore]
    fn part2_period() {
        let instructions = [Instruction::DealWithIncrement(5)];
        let expect_cards: Vec<u64> = (0..10).collect();

        for shuffles in 1..=30 {
            let cards: Vec<u64> = (0..10)
                .map(|i| card_at(&instructions, i, 10007, shuffles).unwrap())
                .collect();
            println!("{} shuffles: {:?}", shuffles, cards);
        }

        let cards: Vec<u64> = (0..10)
            .map(|i| card_at(&instructions, i, 10, 30).unwrap())
            .collect();

        assert_eq!(
            expect_cards,
            cards,
            "Failed with period: {}",
            instructions[0].get_period(10)
        );
    }

    #[test]
    #[ignore]
    fn part2_solution() {
        assert_eq!(Ok(0), part2(include_str!("input.txt")));
    }
}
