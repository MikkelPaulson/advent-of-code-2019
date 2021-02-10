use crate::intcode::Intcode;

pub fn part1(input: &str) -> Result<usize, String> {
    let intcode: Intcode = input.parse()?;
    let mut affected_points = 0;

    for y in 0..50 {
        for x in 0..50 {
            let mut intcode = intcode.clone();
            intcode.input.push(x);
            intcode.input.push(y);
            intcode.run();

            match intcode.output.pop() {
                Some(0) => print!("."),
                Some(1) => {
                    print!("#");
                    affected_points += 1;
                }
                Some(i) => return Err(format!("Invalid output for {}, {}: {}", x, y, i)),
                None => return Err(format!("Empty output for {}, {}", x, y)),
            }
        }
        println!("");
    }

    Ok(affected_points)
}

#[cfg(test)]
mod test {
    use super::part1;

    #[test]
    fn part1_solution() {
        assert_eq!(Ok(183), part1(include_str!("input.txt")));
    }
}
