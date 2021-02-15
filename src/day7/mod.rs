use crate::intcode::{Intcode, Response};

pub fn part1(input: &str) -> Result<u64, String> {
    let mut max = 0;
    let intcode: Intcode = input.parse()?;

    for a in 0..5 {
        for b in (0..5).filter(|&b| b != a) {
            for c in (0..5).filter(|c| !&[a, b][..].contains(c)) {
                for d in (0..5).filter(|d| !&[a, b, c][..].contains(d)) {
                    for e in (0..5).filter(|e| !&[a, b, c, d][..].contains(e)) {
                        let output = Signal::new()
                            .amplify(&intcode, a)
                            .amplify(&intcode, b)
                            .amplify(&intcode, c)
                            .amplify(&intcode, d)
                            .amplify(&intcode, e)
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

    Ok(max as u64)
}

pub fn part2(input: &str) -> Result<u64, String> {
    let mut max = 0;
    let intcode: Intcode = input.parse()?;

    for a in 5..10 {
        for b in (5..10).filter(|&b| b != a) {
            for c in (5..10).filter(|c| !&[a, b][..].contains(c)) {
                for d in (5..10).filter(|d| !&[a, b, c][..].contains(d)) {
                    for e in (5..10).filter(|e| !&[a, b, c, d][..].contains(e)) {
                        let output = Signal::feedback(&intcode, [a, b, c, d, e]);

                        println!("{}, {}, {}, {}, {} => {}", a, b, c, d, e, output);

                        if output > max {
                            max = output;
                        }
                    }
                }
            }
        }
    }

    Ok(max as u64)
}

struct Signal(i64);

impl Signal {
    pub fn new() -> Self {
        Self(0)
    }

    pub fn amplify(&mut self, intcode: &Intcode, phase: i64) -> &mut Self {
        let mut intcode = intcode.clone().with_input(&[phase, self.0]);
        intcode.run();
        self.0 = intcode.output.pop().unwrap();
        self
    }

    pub fn feedback(intcode: &Intcode, phases: [i64; 5]) -> i64 {
        let mut amplifiers = [
            intcode.clone().with_input(&phases[0..1]),
            intcode.clone().with_input(&phases[1..2]),
            intcode.clone().with_input(&phases[2..3]),
            intcode.clone().with_input(&phases[3..4]),
            intcode.clone().with_input(&phases[4..5]),
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

    pub fn get_output(&self) -> i64 {
        self.0
    }
}

#[cfg(test)]
mod test {
    use super::{part1, part2};

    #[test]
    fn part1_examples() {
        assert_eq!(
            Ok(43210),
            part1("3,15,3,16,1002,16,10,16,1,16,15,15,4,15,99,0,0")
        );
        assert_eq!(
            Ok(54321),
            part1("3,23,3,24,1002,24,10,24,1002,23,-1,23,101,5,23,23,1,24,23,23,4,23,99,0,0")
        );
        assert_eq!(
            Ok(65210),
            part1("3,31,3,32,1002,32,10,32,1001,31,-2,31,1007,31,0,33,1002,33,7,33,1,33,31,31,1,32,31,31,4,31,99,0,0,0")
        );
    }

    #[test]
    fn part1_solution() {
        assert_eq!(Ok(21760), part1(include_str!("input.txt")));
    }

    #[test]
    fn part2_examples() {
        assert_eq!(
            Ok(139629729),
            part2("3,26,1001,26,-4,26,3,27,1002,27,2,27,1,27,26,27,4,27,1001,28,-1,28,1005,28,6,99,0,0,5")
        );
        assert_eq!(
            Ok(18216),
            part2("3,52,1001,52,-5,52,3,53,1,52,56,54,1007,54,5,55,1005,55,26,1001,54,-5,54,1105,1,12,1,53,54,53,1008,54,0,55,1001,55,1,55,2,53,55,53,4,53,1001,56,-1,56,1005,56,6,99,0,0,0,0,10")
        );
    }

    #[test]
    fn part2_solution() {
        assert_eq!(Ok(69816958), part2(include_str!("input.txt")));
    }
}
