use std::mem;
use std::str::FromStr;

pub fn part1(input: &str) -> Result<u64, String> {
    let cards = evaluate_with_deck_size(&parse(input)?, 10007);

    cards
        .iter()
        .enumerate()
        .find_map(|(i, &card)| if card == 2019 { Some(i as u64) } else { None })
        .ok_or_else(|| "No card at index 2019.".to_string())
}

fn evaluate_with_deck_size(instructions: &Vec<Instruction>, deck_size: u16) -> Vec<u16> {
    let mut deck = Deck::new(deck_size);

    instructions
        .iter()
        .for_each(|instruction| deck.evaluate(instruction));

    deck.cards
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

    pub fn evaluate(&mut self, instruction: &Instruction) {
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

fn parse(input: &str) -> Result<Vec<Instruction>, String> {
    input.lines().map(|line| line.parse()).collect()
}

#[derive(Debug)]
enum Instruction {
    CutLeft(usize),
    CutRight(usize),
    DealWithIncrement(usize),
    DealIntoNewStack,
}

impl FromStr for Instruction {
    type Err = String;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let input = input.trim_end();

        let (prefix, number) = input
            .find(|c: char| c.is_ascii_digit())
            .map_or_else(|| (input, ""), |i| input.split_at(i));
        let number: Option<usize> = if number == "" {
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
        let apply_instruction = |instruction: &Instruction| {
            let mut deck = Deck::new(10);
            deck.evaluate(instruction);
            deck.cards
        };

        assert_eq!(
            vec![9, 8, 7, 6, 5, 4, 3, 2, 1, 0],
            apply_instruction(&Instruction::DealIntoNewStack)
        );

        assert_eq!(
            vec![3, 4, 5, 6, 7, 8, 9, 0, 1, 2],
            apply_instruction(&Instruction::CutLeft(3))
        );

        assert_eq!(
            vec![6, 7, 8, 9, 0, 1, 2, 3, 4, 5],
            apply_instruction(&Instruction::CutRight(4))
        );

        assert_eq!(
            vec![0, 7, 4, 1, 8, 5, 2, 9, 6, 3],
            apply_instruction(&Instruction::DealWithIncrement(3))
        );
    }

    #[test]
    fn part1_examples() {
        assert_eq!(
            vec![0, 3, 6, 9, 2, 5, 8, 1, 4, 7],
            evaluate_with_deck_size(&parse(include_str!("test1.txt")).unwrap(), 10)
        );

        assert_eq!(
            vec![3, 0, 7, 4, 1, 8, 5, 2, 9, 6],
            evaluate_with_deck_size(&parse(include_str!("test2.txt")).unwrap(), 10)
        );

        assert_eq!(
            vec![6, 3, 0, 7, 4, 1, 8, 5, 2, 9],
            evaluate_with_deck_size(&parse(include_str!("test3.txt")).unwrap(), 10)
        );

        assert_eq!(
            vec![9, 2, 5, 8, 1, 4, 7, 0, 3, 6],
            evaluate_with_deck_size(&parse(include_str!("test4.txt")).unwrap(), 10)
        );
    }

    #[test]
    fn part1_solution() {
        assert_eq!(Ok(6638), part1(include_str!("input.txt")));
    }
}
