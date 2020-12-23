use crate::intcode::Intcode;
use std::io;
use std::io::prelude::*;
use std::str;

pub fn part1(_: Box<dyn Read>) -> Result<usize, &'static str> {
    let mut intcode = get_input().parse::<Intcode>()?;

    let stdin = io::stdin();
    loop {
        intcode.run();
        println!("{}", intcode.output_string());
        intcode.output.clear();

        let mut input = String::new();
        stdin
            .lock()
            .read_line(&mut input)
            .map_err(|_| "Unable to read from stdin")?;

        if input.trim() == "" {
            break;
        }

        intcode.input_str(&input);
    }

    Err("")
}

fn get_input() -> &'static str {
    include_str!("input.txt")
}
