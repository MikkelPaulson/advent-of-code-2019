pub fn part1(input: &str) -> Result<usize, String> {
    let mut sum = 0;
    for mass in parse(input)? {
        sum += calc_fuel_simple(mass).unwrap();
    }
    Ok(sum)
}

pub fn part2(input: &str) -> Result<usize, String> {
    let mut sum = 0;
    for mass in parse(input)? {
        sum += calc_fuel(mass);
    }
    Ok(sum)
}

fn calc_fuel(mass: usize) -> usize {
    calc_fuel_simple(mass).map_or(0, |fuel_mass| fuel_mass + calc_fuel(fuel_mass))
}

fn calc_fuel_simple(mass: usize) -> Option<usize> {
    mass.div_euclid(3).checked_sub(2)
}

fn parse(input: &str) -> Result<Vec<usize>, String> {
    input
        .trim()
        .split('\n')
        .map(|line| line.parse())
        .collect::<Result<_, _>>()
        .map_err(|e| format!("{:?}", e))
}

#[cfg(test)]
mod test {
    use super::{part1, part2};

    #[test]
    fn part1_examples() {
        assert_eq!(Ok(2), part1("12"));
        assert_eq!(Ok(2), part1("14"));
        assert_eq!(Ok(654), part1("1969"));
        assert_eq!(Ok(33583), part1("100756"));
    }

    #[test]
    fn part1_solution() {
        assert_eq!(Ok(3327415), part1(include_str!("input.txt")));
    }

    #[test]
    fn part2_examples() {
        assert_eq!(Ok(2), part2("14"));
        assert_eq!(Ok(966), part2("1969"));
        assert_eq!(Ok(50346), part2("100756"));
    }

    #[test]
    fn part2_solution() {
        assert_eq!(Ok(4988257), part2(include_str!("input.txt")));
    }
}
