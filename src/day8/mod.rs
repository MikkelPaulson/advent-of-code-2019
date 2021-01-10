use std::fmt;
use std::io::prelude::*;
use std::iter;
use std::ops;
use std::slice;
use std::str;

pub fn part1(input: Box<dyn Read>) -> Result<usize, &'static str> {
    let image = parse(input, 25, 6);

    let min_layer = image
        .layers
        .iter()
        .min_by(|a, b| {
            a.data
                .iter()
                .filter(|i| i == &&0)
                .count()
                .cmp(&b.data.iter().filter(|i| i == &&0).count())
        })
        .ok_or("No min layer somehow???")?;

    println!("{}", min_layer);

    let (one_count, two_count) = (
        min_layer.data.iter().filter(|i| i == &&1).count(),
        min_layer.data.iter().filter(|i| i == &&2).count(),
    );

    println!("{} * {}", one_count, two_count);

    Ok(one_count * two_count)
}

pub fn part2(input: Box<dyn Read>) -> Result<usize, &'static str> {
    let image = parse(input, 25, 6);
    let result = image
        .layers
        .iter()
        .fold(Layer::empty(25, 6), |acc, layer| &acc + layer);

    println!(
        "{}",
        format!("{}", result)
            .chars()
            .map(|c| if c == '0' { ' ' } else { c })
            .collect::<String>()
    );

    Ok(0)
}

struct Image {
    pub layers: Vec<Layer>,
}

impl Image {
    pub fn new(data: &str, width: usize, height: usize) -> Self {
        let layer_size = width * height;
        let mut layers = Vec::with_capacity(data.trim().len() / layer_size);

        for start in (0..data.len()).step_by(layer_size) {
            layers.push(Layer::new(&data[start..start + layer_size], width, height));
        }

        Self { layers }
    }
}

struct Layer {
    data: Vec<u8>,
    width: usize,
    height: usize,
}

impl Layer {
    pub fn new(data: &str, width: usize, height: usize) -> Self {
        Self {
            data: data
                .chars()
                .map(|c| c.to_digit(10).unwrap() as u8)
                .collect(),
            width,
            height,
        }
    }

    pub fn empty(width: usize, height: usize) -> Self {
        Self {
            data: iter::repeat(2).take(width * height).collect(),
            width,
            height,
        }
    }

    pub fn rows(&self) -> slice::ChunksExact<u8> {
        self.data.chunks_exact(self.width)
    }
}

impl fmt::Display for Layer {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        for row in self.rows() {
            writeln!(
                formatter,
                "{}",
                row.iter().map(|i| i.to_string()).collect::<String>()
            )?;
        }
        Ok(())
    }
}

impl ops::Add for &Layer {
    type Output = Layer;

    fn add(self, other: Self) -> Self::Output {
        let mut data = Vec::with_capacity(self.data.len());

        for (a, b) in self.data.iter().zip(other.data.iter()) {
            data.push(if a == &2 { *b } else { *a });
        }

        Layer {
            data,
            width: self.width,
            height: self.height,
        }
    }
}

fn parse(mut input: Box<dyn Read>, width: usize, height: usize) -> Image {
    let mut buffer = String::new();
    input.read_to_string(&mut buffer).unwrap();
    Image::new(&buffer.trim(), width, height)
}
