use std::fmt;
use std::iter;
use std::ops;
use std::slice;

pub fn part1(input: &str) -> Result<u64, String> {
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

    Ok((one_count * two_count) as u64)
}

pub fn part2(input: &str) -> Result<u64, String> {
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
    pub fn new(data: &str, width: u64, height: u64) -> Self {
        let layer_size = (width * height) as usize;
        let mut layers = Vec::with_capacity(data.trim().len() / layer_size);

        for start in (0..data.len()).step_by(layer_size) {
            layers.push(Layer::new(&data[start..start + layer_size], width, height));
        }

        Self { layers }
    }
}

struct Layer {
    data: Vec<u8>,
    width: u64,
    height: u64,
}

impl Layer {
    pub fn new(data: &str, width: u64, height: u64) -> Self {
        Self {
            data: data
                .chars()
                .map(|c| c.to_digit(10).unwrap() as u8)
                .collect(),
            width,
            height,
        }
    }

    pub fn empty(width: u64, height: u64) -> Self {
        Self {
            data: iter::repeat(2).take((width * height) as usize).collect(),
            width,
            height,
        }
    }

    pub fn rows(&self) -> slice::ChunksExact<u8> {
        self.data.chunks_exact(self.width as usize)
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

fn parse(input: &str, width: u64, height: u64) -> Image {
    Image::new(input.trim(), width, height)
}

#[cfg(test)]
mod test {
    use super::part1;

    #[test]
    fn part1_solution() {
        assert_eq!(Ok(1677), part1(include_str!("input.txt")));
    }
}
