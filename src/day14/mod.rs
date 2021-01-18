use std::cmp;
use std::collections::HashMap;
use std::hash::Hash;

pub fn part1(input: &str) -> Result<usize, String> {
    let reactions = parse(input)?;
    let mut supply = Supply::default();

    supply = get(&Chemical::FUEL, 1, &reactions, supply)?;

    println!("{:#?}", supply);

    Ok(supply.ore_required as usize)
}

type ChemicalQuantity<'a> = (Chemical<'a>, u32);
type Reactions<'a> = HashMap<Chemical<'a>, Reaction<'a>>;

fn get<'a>(
    chemical: &Chemical<'a>,
    quantity: u32,
    reactions: &Reactions<'a>,
    mut supply: Supply<'a>,
) -> Result<Supply<'a>, String> {
    if chemical == &Chemical::ORE {
        supply.ore_required += quantity;
        return Ok(supply);
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

        let multiplier = (need as f32 / reaction.product.1 as f32).ceil() as u32;

        for reagent in reaction.reagents.iter() {
            supply = get(&reagent.0, reagent.1 * multiplier, reactions, supply)?;
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

    Ok(supply)
}

fn parse(input: &str) -> Result<Reactions, String> {
    input
        .trim()
        .split('\n')
        .map(|s| Reaction::from_str(s).map(|r| (r.product.0, r)))
        .collect::<Result<_, _>>()
}

#[derive(Debug, Default)]
struct Supply<'a> {
    chemicals: HashMap<Chemical<'a>, u32>,
    ore_required: u32,
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
    use super::part1;

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
}
