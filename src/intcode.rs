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
            1 => self.do_add(),
            2 => self.do_mul(),
            3 => self.do_input(),
            4 => self.do_output(),
            5 => self.do_jump_if_true(),
            6 => self.do_jump_if_false(),
            7 => self.do_less_than(),
            8 => self.do_equals(),
            99 => self.do_halt(),
            _ => panic!(
                "Unknown opcode {} at offset {}!",
                self.data[self.cursor], self.cursor,
            ),
        }
    }

    /// Opcode 1 adds together numbers read from two positions and stores the result in a third
    /// position. The three integers immediately after the opcode tell you these three
    /// positions - the first two indicate the positions from which you should read the input
    /// values, and the third indicates the position at which the output should be stored.
    fn do_add(&mut self) -> bool {
        self.set_pos(2, self.get_param(0) + self.get_param(1));
        self.cursor += 4;
        true
    }

    /// Opcode 2 works exactly like opcode 1, except it multiplies the two inputs instead of
    /// adding them. Again, the three integers after the opcode indicate where the inputs and
    /// outputs are, not their values.
    fn do_mul(&mut self) -> bool {
        self.set_pos(2, self.get_param(0) * self.get_param(1));
        self.cursor += 4;
        true
    }

    /// Opcode 3 takes a single integer as input and saves it to the position given
    /// by its only parameter. For example, the instruction 3,50 would take an
    /// input value and store it at address 50.
    fn do_input(&mut self) -> bool {
        let value = self.input.remove(0);
        self.set_pos(0, value);
        self.cursor += 2;
        true
    }

    /// Opcode 4 outputs the value of its only parameter. For example, the
    /// instruction 4,50 would output the value at address 50.
    fn do_output(&mut self) -> bool {
        self.output.push(self.get_param(0));
        self.cursor += 2;
        true
    }

    /// Opcode 5 is jump-if-true: if the first parameter is non-zero, it sets the instruction
    /// pointer to the value from the second parameter. Otherwise, it does nothing.
    fn do_jump_if_true(&mut self) -> bool {
        self.cursor = if self.get_param(0) != 0 {
            self.get_param(1) as usize
        } else {
            self.cursor + 3
        };

        true
    }

    /// Opcode 6 is jump-if-false: if the first parameter is zero, it sets the instruction pointer
    /// to the value from the second parameter. Otherwise, it does nothing.
    fn do_jump_if_false(&mut self) -> bool {
        self.cursor = if self.get_param(0) == 0 {
            self.get_param(1) as usize
        } else {
            self.cursor + 3
        };

        true
    }

    /// Opcode 7 is less than: if the first parameter is less than the second parameter, it stores
    /// 1 in the position given by the third parameter. Otherwise, it stores 0.
    fn do_less_than(&mut self) -> bool {
        self.set_pos(
            2,
            if self.get_param(0) < self.get_param(1) {
                1
            } else {
                0
            },
        );
        self.cursor += 4;
        true
    }

    /// Opcode 8 is equals: if the first parameter is equal to the second parameter, it stores 1 in
    /// the position given by the third parameter. Otherwise, it stores 0.
    fn do_equals(&mut self) -> bool {
        self.set_pos(
            2,
            if self.get_param(0) == self.get_param(1) {
                1
            } else {
                0
            },
        );
        self.cursor += 4;
        true
    }

    /// 99 means that the program is finished and should immediately halt.
    fn do_halt(&mut self) -> bool {
        false
    }

    fn get_param(&self, param_index: usize) -> isize {
        let param = self.data[self.cursor + param_index + 1];

        match Self::get_mode(self.data[self.cursor] as usize, param_index) {
            // Position mode - interpret as pointer.
            0 => self.data[param as usize],

            // Immediate mode - interpret as value.
            1 => param,

            x => panic!("Unrecognized mode: {}", x),
        }
    }

    fn set_pos(&mut self, param_index: usize, value: isize) {
        if Self::get_mode(self.data[self.cursor] as usize, param_index) != 0 {
            panic!("Output parameters must never be in immedate mode!");
        }

        let pos = self.data[self.cursor + param_index + 1] as usize;
        self.data[pos] = value;
    }

    fn get_mode(instruction: usize, param_index: usize) -> usize {
        (instruction as u32 / 10u32.pow(param_index as u32 + 2) % 10) as usize
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
    fn get_mode() {
        assert_eq!(0, Intcode::get_mode(12001, 0));
        assert_eq!(2, Intcode::get_mode(12001, 1));
        assert_eq!(1, Intcode::get_mode(12001, 2));
        assert_eq!(0, Intcode::get_mode(12001, 3));
    }

    #[test]
    fn day2_example1() {
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
    fn day5_example1() {
        let mut intcode = Intcode::new(vec![3, 0, 4, 0, 99]);
        intcode.input.push(123);
        intcode.run();
        assert_eq!(vec![123], intcode.output);
    }

    #[test]
    fn day5_example2() {
        let mut intcode = Intcode::new(vec![1002, 4, 3, 4, 33]);

        assert!(intcode.step());
        assert_eq!(99, intcode.data[4]);

        assert_eq!(false, intcode.step());
    }

    #[test]
    fn day5_example3() {
        let mut intcode = Intcode::new(vec![1101, 100, -1, 4, 0]);

        assert!(intcode.step());
        assert_eq!(99, intcode.data[4]);

        assert_eq!(false, intcode.step());
    }

    #[test]
    fn day5_example4() {
        for i in 6..=10 {
            let mut intcode = Intcode::new(vec![3, 9, 8, 9, 10, 9, 4, 9, 99, -1, 8]);
            intcode.input.push(i);
            intcode.run();
            assert_eq!(vec![if i == 8 { 1 } else { 0 }], intcode.output);
        }
    }

    #[test]
    fn day5_example5() {
        for i in 6..=10 {
            let mut intcode = Intcode::new(vec![3, 9, 7, 9, 10, 9, 4, 9, 99, -1, 8]);
            intcode.input.push(i);
            intcode.run();
            assert_eq!(vec![if i < 8 { 1 } else { 0 }], intcode.output);
        }
    }

    #[test]
    fn day5_example6() {
        for i in 6..=10 {
            let mut intcode = Intcode::new(vec![3, 3, 1108, -1, 8, 3, 4, 3, 99]);
            intcode.input.push(i);
            intcode.run();
            assert_eq!(vec![if i == 8 { 1 } else { 0 }], intcode.output);
        }
    }

    #[test]
    fn day5_example7() {
        for i in 6..=10 {
            let mut intcode = Intcode::new(vec![3, 3, 1107, -1, 8, 3, 4, 3, 99]);
            intcode.input.push(i);
            intcode.run();
            assert_eq!(vec![if i < 8 { 1 } else { 0 }], intcode.output);
        }
    }

    #[test]
    fn day5_example8() {
        for i in -2..=2 {
            let mut intcode = Intcode::new(vec![
                3, 12, 6, 12, 15, 1, 13, 14, 13, 4, 13, 99, -1, 0, 1, 9,
            ]);
            intcode.input.push(i);
            intcode.run();
            assert_eq!(vec![if i == 0 { 0 } else { 1 }], intcode.output);
        }
    }

    #[test]
    fn day5_example9() {
        for i in -2..=2 {
            let mut intcode = Intcode::new(vec![3, 3, 1105, -1, 9, 1101, 0, 0, 12, 4, 12, 99, 1]);
            intcode.input.push(i);
            intcode.run();
            assert_eq!(vec![if i == 0 { 0 } else { 1 }], intcode.output);
        }
    }

    #[test]
    fn day5_example10() {
        for i in 6..=10 {
            let mut intcode = Intcode::new(vec![
                3, 21, 1008, 21, 8, 20, 1005, 20, 22, 107, 8, 21, 20, 1006, 20, 31, 1106, 0, 36,
                98, 0, 0, 1002, 21, 125, 20, 4, 20, 1105, 1, 46, 104, 999, 1105, 1, 46, 1101, 1000,
                1, 20, 4, 20, 1105, 1, 46, 98, 99,
            ]);
            intcode.input.push(i);
            intcode.run();
            assert_eq!(
                vec![if i < 8 {
                    999
                } else if i == 8 {
                    1000
                } else {
                    1001
                }],
                intcode.output
            );
        }
    }
}
