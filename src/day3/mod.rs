use std::collections;
use std::io;
use std::io::prelude::*;
use std::str;

pub fn part1(input: Box<dyn Read>) -> Result<String, &'static str> {
    let lines = parse(input);
    let intersections = lines[0].plot_intersections(&lines[1]);

    if intersections.is_empty() {
        Err("No intersections found!")
    } else {
        let mut min_distance = isize::MAX;

        for (x, y) in intersections {
            let distance = x.abs() + y.abs();
            if distance > 0 && distance < min_distance {
                min_distance = distance;
            }
        }

        Ok(min_distance.to_string())
    }
}

fn parse(input: Box<dyn Read>) -> [Line; 2] {
    let mut result = io::BufReader::new(input)
        .lines()
        .map(|line| line.unwrap().trim().parse().unwrap());

    [result.next().unwrap(), result.next().unwrap()]
}

#[derive(Debug)]
struct Line {
    vertical_segments: collections::HashMap<isize, Vec<(isize, isize)>>,
    horizontal_segments: collections::HashMap<isize, Vec<(isize, isize)>>,
}

impl Line {
    fn plot_intersections(&self, other: &Line) -> Vec<(isize, isize)> {
        let mut overlaps = Vec::new();

        for (my_segments, your_segments) in &[
            (&self.vertical_segments, &other.horizontal_segments),
            (&self.horizontal_segments, &other.vertical_segments),
        ] {
            for (a, segments) in my_segments.iter() {
                for (my_start, my_end) in segments {
                    for b in my_start.clone()..=my_end.clone() {
                        if (*a != 0isize || b != 0isize)
                            && your_segments.get(&b).map_or(false, |v| {
                                v.iter().any(|(min, max)| (min..=max).contains(&a))
                            })
                        {
                            overlaps.push((a.clone(), b.clone()));
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
                    horizontal_segments.get_mut(&y).unwrap().push((x, new_x));

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
                    vertical_segments.get_mut(&x).unwrap().push((y, new_y));

                    y = new_y;
                }
                _ => return Err("Invalid direction."),
            }
        }

        Ok(Self {
            vertical_segments,
            horizontal_segments,
        })
    }
}
