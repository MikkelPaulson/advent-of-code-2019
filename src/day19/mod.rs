use crate::intcode::Intcode;
use crate::map::Coord;

pub fn part1(input: &str) -> Result<u64, String> {
    let intcode: Intcode = input.parse()?;
    let mut affected_points = 0;

    for y in 0..50i64 {
        for x in 0..50i64 {
            if is_hit(&intcode, [x, y])? {
                print!("#");
                affected_points += 1;
            } else {
                print!(".");
            }
        }
        println!("");
    }

    Ok(affected_points)
}

pub fn part2(input: &str) -> Result<u64, String> {
    let intcode = input.parse()?;

    let mut y = 6i64;
    let mut x = 0i64;

    loop {
        print!("{}, {}: ", x, y);

        if is_hit(&intcode, [x, y])? {
            if !is_hit(&intcode, [x + 99, y])? {
                println!("x + 99 misses, moving down");
                y += 1;
            } else if !is_hit(&intcode, [x, y + 99])? {
                println!("y + 99 misses, moving right");
                x += 1;
            } else {
                break;
            }
        } else {
            println!("coordinate misses, moving right");
            x += 1;
        }
    }

    Ok((x * 10000 + y) as u64)
}

pub fn is_hit(intcode: &Intcode, coord: impl Into<Coord>) -> Result<bool, String> {
    let coord = coord.into();
    let mut intcode = intcode.clone();
    intcode.input.push(coord.x);
    intcode.input.push(coord.y);
    intcode.run();

    match intcode.output.pop() {
        Some(0) => Ok(false),
        Some(1) => Ok(true),
        Some(i) => Err(format!(
            "Invalid output for {}, {}: {}",
            coord.x, coord.y, i
        )),
        None => Err(format!("Empty output for {}, {}", coord.x, coord.y)),
    }
}

#[cfg(test)]
mod test {
    use super::{part1, part2};

    #[test]
    fn part1_solution() {
        assert_eq!(Ok(183), part1(include_str!("input.txt")));
    }

    #[test]
    fn part2_solution() {
        assert_eq!(Ok(11221248), part2(include_str!("input.txt")));
    }
}
