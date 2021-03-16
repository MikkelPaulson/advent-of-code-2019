use super::intcode::Intcode;

pub fn part1(input: &str) -> Result<u64, String> {
    let (mut nodes, mut packets) = parse(input)?;

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

pub fn part2(input: &str) -> Result<u64, String> {
    let (mut nodes, mut packets) = parse(input)?;
    let mut is_idle = true;
    let mut last_nat = None;
    let mut nat = None;

    loop {
        for (i, node) in nodes.iter_mut().enumerate() {
            node.run();

            while !node.output.is_empty() {
                is_idle = false;

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
                    nat = Some(packet);
                } else {
                    packets[address].push(packet);
                }
            }
        }

        nodes.iter_mut().enumerate().for_each(|(i, node)| {
            if packets[i].is_empty() {
                node.input.push(-1);
            } else {
                is_idle = false;

                let [x, y] = packets[i].remove(0);
                node.input.push(x);
                node.input.push(y);
            }
        });

        if is_idle {
            if let Some(nat) = nat {
                if last_nat == Some(nat) {
                    return Ok(nat[1] as u64);
                }

                packets[0].push(nat);

                last_nat = Some(nat);
            }
        }

        is_idle = true;
    }
}

fn parse(input: &str) -> Result<(Vec<Intcode>, Vec<Vec<[i64; 2]>>), String> {
    let intcode: Intcode = input.parse()?;

    let nodes: Vec<Intcode> = (0..=49)
        .map(|i| {
            let mut node = intcode.clone();
            node.input.push(i);
            node
        })
        .collect();
    let packets: Vec<Vec<[i64; 2]>> = (0..=49).map(|_| Vec::new()).collect();

    Ok((nodes, packets))
}

#[cfg(test)]
mod test {
    use super::{part1, part2};

    #[test]
    fn part1_solution() {
        assert_eq!(Ok(19530), part1(include_str!("input.txt")));
    }

    #[test]
    fn part2_solution() {
        assert_eq!(Ok(12725), part2(include_str!("input.txt")));
    }
}
