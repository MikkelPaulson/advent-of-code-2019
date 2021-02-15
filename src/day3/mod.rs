use std::collections;
use std::str;

pub fn part1(input: &str) -> Result<u64, String> {
    let lines = parse(input);
    let intersections = lines[0].plot_intersections(&lines[1]);

    if intersections.is_empty() {
        Err("No intersections found!".to_string())
    } else {
        let mut min_distance = u64::MAX;

        for (x, y, _) in intersections {
            let distance = (x.abs() + y.abs()) as u64;
            if distance > 0 && distance < min_distance {
                min_distance = distance;
            }
        }

        Ok(min_distance)
    }
}

pub fn part2(input: &str) -> Result<u64, String> {
    let lines = parse(input);
    let intersections = lines[0].plot_intersections(&lines[1]);

    if intersections.is_empty() {
        Err("No intersections found!".to_string())
    } else {
        println!("{:?}", intersections);
        intersections
            .iter()
            .map(|(_, _, distance)| *distance)
            .min()
            .ok_or("Not OK!".to_string())
    }
}

fn parse(input: &str) -> [Line; 2] {
    let mut result = input.trim().split('\n').map(|s| s.parse().unwrap());
    [result.next().unwrap(), result.next().unwrap()]
}

#[derive(Debug)]
struct Segment {
    start: i64,
    end: i64,
    distance: u64,
}

impl Segment {
    pub fn new(start: i64, end: i64, distance: u64) -> Self {
        Self {
            start,
            end,
            distance,
        }
    }
}

#[derive(Debug)]
struct Line {
    vertical_segments: collections::HashMap<i64, Vec<Segment>>,
    horizontal_segments: collections::HashMap<i64, Vec<Segment>>,
}

impl Line {
    fn plot_intersections(&self, other: &Line) -> Vec<(i64, i64, u64)> {
        let mut overlaps = Vec::new();
        println!("{:?}", self);
        println!("{:?}", other);

        for (my_segments, your_segments) in &[
            (&self.vertical_segments, &other.horizontal_segments),
            (&self.horizontal_segments, &other.vertical_segments),
        ] {
            for (a, my_segment_set) in my_segments.iter() {
                for my_segment in my_segment_set {
                    for i in 0..=(my_segment.end - my_segment.start).abs() {
                        let b = my_segment.start + i * (my_segment.end - my_segment.start).signum();
                        if *a != 0i64 || b != 0i64 {
                            if let Some(your_segment_set) = your_segments.get(&b) {
                                for your_segment in your_segment_set {
                                    if (your_segment.start..=your_segment.end).contains(&a)
                                        || (your_segment.end..=your_segment.start).contains(&a)
                                    {
                                        println!(
                                            "({})=({})=({})@{} crosses ({})=({})=({})@{}: {}",
                                            my_segment.start,
                                            b,
                                            my_segment.end,
                                            my_segment.distance,
                                            your_segment.start,
                                            a,
                                            your_segment.end,
                                            your_segment.distance,
                                            my_segment.distance
                                                + (b - my_segment.start).abs() as u64
                                                + your_segment.distance
                                                + (a - your_segment.start).abs() as u64,
                                        );
                                        overlaps.push((
                                            a.clone(),
                                            b.clone(),
                                            my_segment.distance
                                                + (b - my_segment.start).abs() as u64
                                                + your_segment.distance
                                                + (a - your_segment.start).abs() as u64,
                                        ));
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        overlaps
    }
}

impl str::FromStr for Line {
    type Err = &'static str;

    fn from_str(raw: &str) -> Result<Self, Self::Err> {
        let mut x = 0;
        let mut y = 0;

        let mut vertical_segments = collections::HashMap::new();
        let mut horizontal_segments = collections::HashMap::new();
        let mut total_distance = 0;

        for segment in raw.trim().split(',') {
            let direction = segment.chars().nth(0).unwrap();
            let distance = &segment[1..].parse().unwrap();

            match direction {
                'R' | 'L' => {
                    let new_x = if direction == 'R' {
                        x + distance
                    } else {
                        x - distance
                    };

                    if !horizontal_segments.contains_key(&y) {
                        horizontal_segments.insert(y, Vec::with_capacity(1));
                    }
                    horizontal_segments.get_mut(&y).unwrap().push(Segment::new(
                        x,
                        new_x,
                        total_distance,
                    ));

                    x = new_x;
                }
                'U' | 'D' => {
                    let new_y = if direction == 'U' {
                        y + distance
                    } else {
                        y - distance
                    };

                    if !vertical_segments.contains_key(&x) {
                        vertical_segments.insert(x, Vec::with_capacity(1));
                    }
                    vertical_segments.get_mut(&x).unwrap().push(Segment::new(
                        y,
                        new_y,
                        total_distance,
                    ));

                    y = new_y;
                }
                _ => return Err("Invalid direction."),
            }

            total_distance += *distance as u64;
        }

        Ok(Self {
            vertical_segments,
            horizontal_segments,
        })
    }
}

#[cfg(test)]
mod test {
    use super::{part1, part2};

    #[test]
    fn part1_examples() {
        assert_eq!(Ok(6), part1(include_str!("test1.txt")));
        assert_eq!(Ok(159), part1(include_str!("test2.txt")));
        assert_eq!(Ok(135), part1(include_str!("test3.txt")));
    }

    #[test]
    fn part1_solution() {
        assert_eq!(Ok(1519), part1(include_str!("input.txt")));
    }

    #[test]
    fn part2_examples() {
        assert_eq!(Ok(30), part2(include_str!("test1.txt")));
        assert_eq!(Ok(610), part2(include_str!("test2.txt")));
        assert_eq!(Ok(410), part2(include_str!("test3.txt")));
    }

    #[test]
    fn part2_solution() {
        assert_eq!(Ok(14358), part2(include_str!("input.txt")));
    }
}
