use crate::intcode::Intcode;
use std::io::prelude::*;
use std::str;

pub fn part1(_: Box<dyn Read>) -> Result<usize, &'static str> {
    let mut max = 0;

    for a in 0..5 {
        for b in (0..5).filter(|&b| b != a) {
            for c in (0..5).filter(|c| !&[a, b][..].contains(c)) {
                for d in (0..5).filter(|d| !&[a, b, c][..].contains(d)) {
                    for e in (0..5).filter(|e| !&[a, b, c, d][..].contains(e)) {
                        let output = Signal::new()
                            .amplify(a)
                            .amplify(b)
                            .amplify(c)
                            .amplify(d)
                            .amplify(e)
                            .get_output();

                        println!("{}, {}, {}, {}, {} => {}", a, b, c, d, e, output);

                        if output > max {
                            max = output;
                        }
                    }
                }
            }
        }
    }

    Ok(max as usize)
}

struct Signal(isize);

impl Signal {
    pub fn new() -> Self {
        Self(0)
    }

    pub fn amplify(&mut self, phase: u8) -> &mut Self {
        let mut intcode: Intcode = include_str!("input.txt").parse().unwrap();
        intcode.input.push(phase as isize);
        intcode.input.push(self.0);
        intcode.run();
        self.0 = intcode.output.pop().unwrap();
        self
    }

    pub fn get_output(&self) -> isize {
        self.0
    }
}
