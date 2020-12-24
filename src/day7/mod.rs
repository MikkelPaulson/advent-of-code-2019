use crate::intcode::{Intcode, Response};
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

pub fn part2(_: Box<dyn Read>) -> Result<usize, &'static str> {
    let mut max = 0;

    for a in 5..10 {
        for b in (5..10).filter(|&b| b != a) {
            for c in (5..10).filter(|c| !&[a, b][..].contains(c)) {
                for d in (5..10).filter(|d| !&[a, b, c][..].contains(d)) {
                    for e in (5..10).filter(|e| !&[a, b, c, d][..].contains(e)) {
                        let output = Signal::feedback([a, b, c, d, e]);

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
        let mut intcode = Self::get_intcode(phase);
        intcode.input.push(self.0);
        intcode.run();
        self.0 = intcode.output.pop().unwrap();
        self
    }

    pub fn feedback(phases: [u8; 5]) -> isize {
        let mut amplifiers = [
            Self::get_intcode(phases[0]),
            Self::get_intcode(phases[1]),
            Self::get_intcode(phases[2]),
            Self::get_intcode(phases[3]),
            Self::get_intcode(phases[4]),
        ];

        let mut signal = 0;
        loop {
            for i in 0..4 {
                amplifiers[i].input.push(signal);
                amplifiers[i].run();
                signal = amplifiers[i].output.pop().unwrap();
            }

            amplifiers[4].input.push(signal);
            if Response::Terminated == amplifiers[4].run() {
                break amplifiers[4].output.pop().unwrap();
            } else {
                signal = amplifiers[4].output.pop().unwrap();
            }
        }
    }

    pub fn get_output(&self) -> isize {
        self.0
    }

    fn get_intcode(phase: u8) -> Intcode {
        let mut intcode: Intcode = include_str!("input.txt").parse().unwrap();
        intcode.input.push(phase as isize);
        intcode
    }
}
