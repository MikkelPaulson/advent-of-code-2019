use std::io::prelude::*;
use std::str;

#[derive(Clone, Debug)]
pub struct Intcode {
    data: Vec<isize>,
    pub input: Vec<isize>,
    pub output: Vec<isize>,
    cursor: usize,
}

impl Intcode {
    pub fn new(data: Vec<isize>) -> Self {
        Self {
            data,
            input: Vec::new(),
            output: Vec::new(),
            cursor: 0,
        }
    }

    pub fn parse(mut input: Box<dyn Read>) -> Self {
        let mut buffer = String::new();
        input.read_to_string(&mut buffer).unwrap();
        buffer.parse().unwrap()
    }

    pub fn set(&mut self, offset: usize, value: isize) {
        self.data[offset] = value;
    }

    pub fn get(&self, offset: usize) -> isize {
        self.data[offset]
    }

    pub fn run(&mut self) {
        while self.step() {}
    }

    pub fn step(&mut self) -> bool {
        let opcode = self.data[self.cursor] % 100;

        match opcode {
            // Opcode 1 adds together numbers read from two positions and stores the
            // result in a third position. The three integers immediately after the
            // opcode tell you these three positions - the first two indicate the
            // positions from which you should read the input values, and the third
            // indicates the position at which the output should be stored.
            1 => {
                let (a_pos, b_pos, o_pos) = (
                    self.data[self.cursor + 1] as usize,
                    self.data[self.cursor + 2] as usize,
                    self.data[self.cursor + 3] as usize,
                );

                self.data[o_pos] = self.data[a_pos] + self.data[b_pos];

                self.cursor += 4;
            }

            // Opcode 2 works exactly like opcode 1, except it multiplies the two inputs
            // instead of adding them. Again, the three integers after the opcode
            // indicate where the inputs and outputs are, not their values.
            2 => {
                let (a_pos, b_pos, o_pos) = (
                    self.data[self.cursor + 1] as usize,
                    self.data[self.cursor + 2] as usize,
                    self.data[self.cursor + 3] as usize,
                );

                self.data[o_pos] = self.data[a_pos] * self.data[b_pos];

                self.cursor += 4;
            }

            // Opcode 3 takes a single integer as input and saves it to the position given
            // by its only parameter. For example, the instruction 3,50 would take an
            // input value and store it at address 50.
            3 => {
                let pos = self.data[self.cursor + 1] as usize;
                self.data[pos] = self.input.remove(0);
                self.cursor += 2;
            }

            // Opcode 4 outputs the value of its only parameter. For example, the
            // instruction 4,50 would output the value at address 50.
            4 => {
                let pos = self.data[self.cursor + 1] as usize;
                self.output.push(self.data[pos]);
                self.cursor += 2;
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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn day2_example() {
        let mut intcode = Intcode::new(vec![1, 9, 10, 3, 2, 3, 11, 0, 99, 30, 40, 50]);

        assert!(intcode.step());
        assert_eq!(70, intcode.data[3]);

        assert!(intcode.step());
        assert_eq!(3500, intcode.data[0]);

        assert_eq!(false, intcode.step());
    }

    #[test]
    fn day2_example2() {
        let mut intcode = Intcode::new(vec![1, 0, 0, 0, 99]);
        intcode.run();
        assert_eq!(vec![2, 0, 0, 0, 99], intcode.data);
    }

    #[test]
    fn day2_example3() {
        let mut intcode = Intcode::new(vec![2, 3, 0, 3, 99]);
        intcode.run();
        assert_eq!(vec![2, 3, 0, 6, 99], intcode.data);
    }

    #[test]
    fn day2_example4() {
        let mut intcode = Intcode::new(vec![2, 4, 4, 5, 99, 0]);
        intcode.run();
        assert_eq!(vec![2, 4, 4, 5, 99, 9801], intcode.data);
    }

    #[test]
    fn day2_example5() {
        let mut intcode = Intcode::new(vec![1, 1, 1, 4, 99, 5, 6, 0, 99]);
        intcode.run();
        assert_eq!(vec![30, 1, 1, 4, 2, 5, 6, 0, 99], intcode.data);
    }

    #[test]
    fn day5_example() {
        let mut intcode = Intcode::new(vec![3, 0, 4, 0, 99]);
        intcode.input.push(123);
        intcode.run();
        assert_eq!(vec![123], intcode.output);
    }
}
