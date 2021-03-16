use super::intcode::Intcode;

pub fn part1(input: &str) -> Result<u64, String> {
    let intcode: Intcode = input.parse()?;

    let mut nodes: Vec<Intcode> = (0..=49)
        .map(|i| {
            let mut node = intcode.clone();
            node.input.push(i);
            node
        })
        .collect();
    let mut packets: Vec<Vec<[i64; 2]>> = (0..=49).map(|_| Vec::new()).collect();

    loop {
        for (i, node) in nodes.iter_mut().enumerate() {
            node.run();

            while !node.output.is_empty() {
                let mut iter = node.output.drain(..3);
                let address = iter
                    .next()
                    .ok_or_else(|| format!("Missing address on node {}", i))?
                    as usize;

                let packet = [
                    iter.next()
                        .ok_or_else(|| format!("Missing x value on node {}", i))?,
                    iter.next()
                        .ok_or_else(|| format!("Missing y value on node {}", i))?,
                ];

                println!("{} => {}: {:?}", i, address, packet);

                if address == 255 {
                    return Ok(packet[1] as u64);
                } else {
                    packets[address].push(packet);
                }
            }
        }

        nodes.iter_mut().enumerate().for_each(|(i, node)| {
            if packets[i].is_empty() {
                node.input.push(-1);
            } else {
                let [x, y] = packets[i].remove(0);
                node.input.push(x);
                node.input.push(y);
            }
        });
    }
}

#[cfg(test)]
mod test {
    use super::part1;

    #[test]
    fn part1_solution() {
        assert_eq!(Ok(19530), part1(include_str!("input.txt")));
    }
}
