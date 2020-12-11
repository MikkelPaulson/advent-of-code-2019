use std::io::prelude::*;
use std::str;

#[derive(Debug)]
pub struct Intcode {
    data: Vec<usize>,
    cursor: usize,
}

impl Intcode {
    pub fn new(data: Vec<usize>) -> Self {
        Self { data, cursor: 0 }
    }

    pub fn parse(mut input: Box<dyn Read>) -> Self {
        let mut buffer = String::new();
        input.read_to_string(&mut buffer).unwrap();
        buffer.parse().unwrap()
    }

    pub fn set(&mut self, offset: usize, value: usize) {
        self.data[offset] = value;
    }

    pub fn get(&self, offset: usize) -> usize {
        self.data[offset]
    }

    pub fn run(&mut self) {
        while self.step() {}
    }

    pub fn step(&mut self) -> bool {
        match self.data[self.cursor] {
            // Opcode 1 adds together numbers read from two positions and stores the
            // result in a third position. The three integers immediately after the
            // opcode tell you these three positions - the first two indicate the
            // positions from which you should read the input values, and the third
            // indicates the position at which the output should be stored.
            1 => {
                let (a_pos, b_pos, o_pos) = (
                    self.data[self.cursor + 1],
                    self.data[self.cursor + 2],
                    self.data[self.cursor + 3],
                );

                self.data[o_pos] = self.data[a_pos] + self.data[b_pos];

                self.cursor += 4;
            }

            // Opcode 2 works exactly like opcode 1, except it multiplies the two inputs
            // instead of adding them. Again, the three integers after the opcode
            // indicate where the inputs and outputs are, not their values.
            2 => {
                let (a_pos, b_pos, o_pos) = (
                    self.data[self.cursor + 1],
                    self.data[self.cursor + 2],
                    self.data[self.cursor + 3],
                );

                self.data[o_pos] = self.data[a_pos] * self.data[b_pos];

                self.cursor += 4;
            }

            // 99 means that the program is finished and should immediately halt.
            99 => return false,

            _ => panic!(
                "Invalid opcode {:?} at offset {:?}",
                self.data[self.cursor], self.cursor,
            ),
        }
        true
    }
}

impl str::FromStr for Intcode {
    type Err = &'static str;

    fn from_str(raw: &str) -> Result<Self, Self::Err> {
        Ok(Self::new(
            raw.split(',')
                .map(|input| input.trim().parse().unwrap())
                .collect(),
        ))
    }
}
