use std::cmp;
use std::collections::HashMap;
use std::hash::Hash;

pub fn part1(input: &str) -> Result<usize, String> {
    let reactions = parse(input)?;
    let mut supply = Supply::default();

    get(&Chemical::FUEL, 1, &reactions, &mut supply)?;

    Ok(supply.ore_required as usize)
}

pub fn part2(input: &str) -> Result<usize, String> {
    let reactions = parse(input)?;
    let ore_available: u64 = 1_000_000_000_000;

    let (mut min, mut max) = (0, ore_available);

    while min < max - 1 {
        let mut supply = Supply::default();
        let test_val = ((min + max) as f64 / 2.).ceil() as u64;
        get(&Chemical::FUEL, test_val, &reactions, &mut supply)?;

        if supply.ore_required > ore_available {
            max = test_val;
        } else {
            min = test_val;
        }
    }

    Ok(min as usize)
}

type ChemicalQuantity<'a> = (Chemical<'a>, u64);
type Reactions<'a> = HashMap<Chemical<'a>, Reaction<'a>>;

fn get<'a>(
    chemical: &Chemical<'a>,
    quantity: u64,
    reactions: &Reactions<'a>,
    supply: &mut Supply<'a>,
) -> Result<(), String> {
    if chemical == &Chemical::ORE {
        supply.ore_required += quantity;
        return Ok(());
    }

    let need = if let Some(available) = supply.chemicals.remove(chemical) {
        match quantity.cmp(&available) {
            cmp::Ordering::Less => {
                supply.chemicals.insert(*chemical, available - quantity);
                0
            }
            cmp::Ordering::Equal => 0,
            cmp::Ordering::Greater => quantity - available,
        }
    } else {
        quantity
    };

    if need > 0 {
        let reaction = reactions
            .get(chemical)
            .ok_or_else(|| format!("No reaction to produce {:?}.", chemical))?;

        let multiplier = (need as f64 / reaction.product.1 as f64).ceil() as u64;

        for reagent in reaction.reagents.iter() {
            get(
                &reagent.0,
                reagent
                    .1
                    .checked_mul(multiplier)
                    .ok_or_else(|| format!("{:?} * {} (need {})", reaction, multiplier, need))?,
                reactions,
                supply,
            )?;
        }

        let product = multiplier * reaction.product.1;
        if product > need {
            supply
                .chemicals
                .entry(*chemical)
                .and_modify(|v| *v += product - need)
                .or_insert(product - need);
        }
    }

    Ok(())
}

fn parse(input: &str) -> Result<Reactions, String> {
    input
        .trim()
        .split('\n')
        .map(|s| Reaction::from_str(s).map(|r| (r.product.0, r)))
        .collect::<Result<_, _>>()
}

#[derive(Clone, Debug, Default)]
struct Supply<'a> {
    chemicals: HashMap<Chemical<'a>, u64>,
    ore_required: u64,
}

#[derive(Debug)]
struct Reaction<'a> {
    reagents: Vec<ChemicalQuantity<'a>>,
    product: ChemicalQuantity<'a>,
}

impl<'a> Reaction<'a> {
    fn from_str(input: &'a str) -> Result<Self, String> {
        let mut parts = input.trim().split(" => ");

        let (reagents, product) = (
            parts.next().ok_or_else(|| "Missing reagent.".to_string())?,
            parts.next().ok_or_else(|| "Missing product.".to_string())?,
        );

        if let Some(part) = parts.next() {
            Err(format!("Unexpected input: {}", part))
        } else {
            Ok(Reaction {
                reagents: reagents
                    .split(", ")
                    .map(|s| Chemical::from_str(s))
                    .collect::<Result<_, _>>()?,
                product: Chemical::from_str(product)?,
            })
        }
    }
}

#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
struct Chemical<'a>(&'a str);

impl<'a> Chemical<'a> {
    const ORE: Chemical<'static> = Chemical("ORE");
    const FUEL: Chemical<'static> = Chemical("FUEL");

    fn from_str(input: &'a str) -> Result<ChemicalQuantity<'a>, String> {
        let mut iter = input.split(' ');
        if let (Some(quantity), Some(symbol)) = (iter.next(), iter.next()) {
            match quantity.parse() {
                Ok(quantity) => Ok((Self(symbol), quantity)),
                Err(e) => Err(format!("{:?}", e)),
            }
        } else {
            Err("Invalid input.".to_string())
        }
    }
}

#[cfg(test)]
mod test {
    use super::{part1, part2};

    #[test]
    fn part1_examples() {
        assert_eq!(Ok(31), part1(include_str!("test1.txt")));
        assert_eq!(Ok(165), part1(include_str!("test2.txt")));
        assert_eq!(Ok(13312), part1(include_str!("test3.txt")));
        assert_eq!(Ok(180697), part1(include_str!("test4.txt")));
        assert_eq!(Ok(2210736), part1(include_str!("test5.txt")));
    }

    #[test]
    fn part1_solution() {
        assert_eq!(Ok(1037742), part1(include_str!("input.txt")));
    }

    #[test]
    fn part2_examples() {
        assert_eq!(Ok(82892753), part2(include_str!("test3.txt")));
        assert_eq!(Ok(5586022), part2(include_str!("test4.txt")));
        assert_eq!(Ok(460664), part2(include_str!("test5.txt")));
    }

    #[test]
    fn part2_solution() {
        assert_eq!(Ok(1572358), part2(include_str!("input.txt")));
    }
}
