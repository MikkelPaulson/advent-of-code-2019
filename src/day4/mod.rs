use regex::Regex;
use std::io;
use std::io::prelude::*;
use std::str;

pub fn part1(input: Box<dyn Read>) -> Result<usize, &'static str> {
    evaluate(input, Regex::new("00|11|22|33|44|55|66|77|88|99").unwrap())
}

pub fn part2(input: Box<dyn Read>) -> Result<usize, &'static str> {
    // Worst regex ever, but the Rust crate doesn't support backreferences.
    evaluate(input, Regex::new("([^0]|^)00([^0]|$)|([^1]|^)11([^1]|$)|([^2]|^)22([^2]|$)|([^3]|^)33([^3]|$)|([^4]|^)44([^4]|$)|([^5]|^)55([^5]|$)|([^6]|^)66([^6]|$)|([^7]|^)77([^7]|$)|([^8]|^)88([^8]|$)|([^9]|^)99([^9]|$)").unwrap())
}

pub fn evaluate(input: Box<dyn Read>, pattern: Regex) -> Result<usize, &'static str> {
    let [lower, upper] = parse(input);
    println!("{:?}", lower);
    println!("{:?}", upper);

    let lower_val: usize = lower.parse().unwrap();
    let upper_val: usize = upper.parse().unwrap();

    let mut match_count = 0;
    let mut test_count = 0;
    for a in lower.chars().nth(0).unwrap()..='9' {
        for b in a..='9' {
            for c in b..='9' {
                for d in c..='9' {
                    for e in d..='9' {
                        for f in e..='9' {
                            test_count += 1;
                            let test_string = format!("{}{}{}{}{}{}", a, b, c, d, e, f);

                            if !pattern.is_match(&test_string) {
                                continue;
                            }

                            match test_string.parse::<usize>() {
                                Ok(i) if i < lower_val => continue,
                                Ok(i) if i > upper_val => {
                                    println!("Tests: {:?}", test_count);
                                    return Ok(match_count);
                                }
                                _ => {}
                            }

                            match_count += 1;
                        }
                    }
                }
            }
        }
    }

    Ok(match_count)
}

fn parse(input: Box<dyn Read>) -> [String; 2] {
    let line = io::BufReader::new(input).lines().next().unwrap().unwrap();
    [String::from(&line[0..6]), String::from(&line[7..13])]
}
