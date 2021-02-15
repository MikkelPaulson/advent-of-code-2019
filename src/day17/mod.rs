use crate::intcode::Intcode;

pub fn part1(input: &str) -> Result<u64, String> {
    let mut intcode: Intcode = input.parse()?;
    intcode.run();

    let map = intcode.output_string();

    println!("{}", map);
    println!("{:?}", get_intersections(&map));

    Ok(get_intersections(&map)
        .iter()
        .map(|(row, col)| row * col)
        .sum())
}

pub fn part2(input: &str) -> Result<u64, String> {
    let mut intcode: Intcode = input.parse()?;
    intcode.set(0, 2);

    // Main movement routine
    intcode.input_str(&"A,A,B,C,B,C,B,C,B,A\n");

    // Function A
    intcode.input_str(&"R,10,L,12,R,6\n");

    // Function B
    intcode.input_str(&"R,6,R,10,R,12,R,6\n");

    // Function C
    intcode.input_str(&"R,10,L,12,L,12\n");

    // "Continuous video feed"
    intcode.input_str(&"n\n");
    intcode.run();

    let result = intcode
        .output
        .pop()
        .map(|i| i as u64)
        .ok_or_else(|| "No output.".to_string());
    println!("{}", intcode.output_string());
    result
}

fn get_intersections(map: &str) -> Vec<(u64, u64)> {
    let map_lines = map.trim_end().split('\n').collect::<Vec<&str>>();
    let mut intersections = Vec::new();

    for row in 1..map_lines.len() - 1 {
        for col in 1..map_lines[0].len() - 1 {
            if &map_lines[row][col..col + 1] != "."
                && &map_lines[row - 1][col..col + 1] != "."
                && &map_lines[row][col - 1..col] != "."
                && &map_lines[row + 1][col..col + 1] != "."
                && &map_lines[row][col..col + 1] != "."
            {
                intersections.push((row as u64, col as u64));
            }
        }
    }

    intersections
}

#[cfg(test)]
mod test {
    use super::{part1, part2};

    #[test]
    fn part1_solution() {
        assert_eq!(Ok(7816), part1(include_str!("input.txt")));
    }

    #[test]
    fn part2_solution() {
        assert_eq!(Ok(952010), part2(include_str!("input.txt")));
    }
}
