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

        for (x, y, _) in intersections {
            let distance = x.abs() + y.abs();
            if distance > 0 && distance < min_distance {
                min_distance = distance;
            }
        }

        Ok(min_distance.to_string())
    }
}

pub fn part2(input: Box<dyn Read>) -> Result<String, &'static str> {
    let lines = parse(input);
    let intersections = lines[0].plot_intersections(&lines[1]);

    if intersections.is_empty() {
        Err("No intersections found!")
    } else {
        println!("{:?}", intersections);
        intersections
            .iter()
            .map(|(_, _, distance)| distance)
            .min()
            .map(|distance| distance.to_string())
            .ok_or("Not OK!")
    }
}

fn parse(input: Box<dyn Read>) -> [Line; 2] {
    let mut result = io::BufReader::new(input)
        .lines()
        .map(|line| line.unwrap().trim().parse().unwrap());

    [result.next().unwrap(), result.next().unwrap()]
}

#[derive(Debug)]
struct Segment {
    start: isize,
    end: isize,
    distance: isize,
}

impl Segment {
    pub fn new(start: isize, end: isize, distance: isize) -> Self {
        Self {
            start,
            end,
            distance,
        }
    }
}

#[derive(Debug)]
struct Line {
    vertical_segments: collections::HashMap<isize, Vec<Segment>>,
    horizontal_segments: collections::HashMap<isize, Vec<Segment>>,
}

impl Line {
    fn plot_intersections(&self, other: &Line) -> Vec<(isize, isize, isize)> {
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
                        if *a != 0isize || b != 0isize {
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
                                                + (b - my_segment.start).abs()
                                                + your_segment.distance
                                                + (a - your_segment.start).abs(),
                                        );
                                        overlaps.push((
                                            a.clone(),
                                            b.clone(),
                                            my_segment.distance
                                                + (b - my_segment.start).abs()
                                                + your_segment.distance
                                                + (a - your_segment.start).abs(),
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

            total_distance += distance;
        }

        Ok(Self {
            vertical_segments,
            horizontal_segments,
        })
    }
}
