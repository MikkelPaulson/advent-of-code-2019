use std::io;
use std::io::prelude::*;
use std::str;

pub fn part1(input: Box<dyn Read>) -> Result<String, &'static str> {
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
                            if a != b && b != c && c != d && d != e && e != f {
                                continue;
                            }

                            match format!("{}{}{}{}{}{}", a, b, c, d, e, f).parse::<usize>() {
                                Ok(i) if i < lower_val => continue,
                                Ok(i) if i > upper_val => {
                                    println!("Tests: {:?}", test_count);
                                    return Ok(match_count.to_string());
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

    Ok(String::new())
}

fn parse(input: Box<dyn Read>) -> [String; 2] {
    let line = io::BufReader::new(input).lines().next().unwrap().unwrap();
    [String::from(&line[0..6]), String::from(&line[7..13])]
}
