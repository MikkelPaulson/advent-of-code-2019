use regex::Regex;

pub fn part1(input: &str) -> Result<usize, String> {
    evaluate(input, Regex::new("00|11|22|33|44|55|66|77|88|99").unwrap())
}

pub fn part2(input: &str) -> Result<usize, String> {
    // Worst regex ever, but the Rust crate doesn't support backreferences.
    evaluate(input, Regex::new("([^0]|^)00([^0]|$)|([^1]|^)11([^1]|$)|([^2]|^)22([^2]|$)|([^3]|^)33([^3]|$)|([^4]|^)44([^4]|$)|([^5]|^)55([^5]|$)|([^6]|^)66([^6]|$)|([^7]|^)77([^7]|$)|([^8]|^)88([^8]|$)|([^9]|^)99([^9]|$)").unwrap())
}

pub fn evaluate(input: &str, pattern: Regex) -> Result<usize, String> {
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

fn parse(input: &str) -> [&str; 2] {
    [&input[0..6], &input[7..13]]
}

#[cfg(test)]
mod test {
    use super::{part1, part2};

    #[test]
    fn part1_examples() {
        assert_eq!(Ok(1), part1("122345-122345"));
        assert_eq!(Ok(1), part1("111123-111123"));
        assert_eq!(Ok(1), part1("111111-111111"));
        assert_eq!(Ok(0), part1("223450-223450"));
        assert_eq!(Ok(0), part1("123789-123789"));
    }

    #[test]
    fn part1_solution() {
        assert_eq!(Ok(1033), part1(include_str!("input.txt")));
    }

    #[test]
    fn part2_examples() {
        assert_eq!(Ok(1), part2("112233-112233"));
        assert_eq!(Ok(0), part2("123444-123444"));
        assert_eq!(Ok(1), part2("111122-111122"));
    }

    #[test]
    fn part2_solution() {
        assert_eq!(Ok(670), part2(include_str!("input.txt")));
    }
}
