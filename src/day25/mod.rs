use crate::intcode::Intcode;
use std::collections;
use std::fmt;
use std::io;
use std::io::prelude::*;
use std::str;

pub fn part1(input: &str) -> Result<usize, String> {
    let mut game = Game::new(input);
    game.loot()?;

    let stdin = io::stdin();
    loop {
        println!("{:?}", game.room);

        let mut input = String::new();
        stdin
            .lock()
            .read_line(&mut input)
            .map_err(|_| "Unable to read from stdin")?;

        if let Ok(command) = input.parse() {
            println!("{:?}", command);
            game.command(command);
        } else {
            println!("Invalid command: {}", input);
        }
    }
}

struct Game {
    intcode: Intcode,
    room: Room,
}

impl Game {
    pub fn new(input: &str) -> Self {
        let mut intcode: Intcode = input.parse().unwrap();
        intcode.run();
        let room: Room = intcode.output_string().parse().unwrap();
        intcode.output.clear();

        Game { intcode, room }
    }

    pub fn command(&mut self, command: Command) {
        self.intcode.input_str(&String::from(&command)[..]);
        self.intcode.input.push('\n' as isize);
        self.intcode.run();

        if let Ok(room) = self.intcode.output_string().parse() {
            println!("{}", self.intcode.output_string());
            self.room = room;
        } else {
            println!("{}", self.intcode.output_string());
        }

        self.intcode.output.clear();
    }

    pub fn loot(&mut self) -> Result<(), &'static str> {
        self.command("south".parse()?); // Hull Breach -> Holodeck
        self.command("west".parse()?); // Holodeck -> Corridor
        self.command("north".parse()?); // Corridor -> Crew Quarters
                                        //self.command("take fuel cell".parse()?);
        self.command("south".parse()?); // Crew Quarters -> Corridor
        self.command("east".parse()?); // Corridor -> Holodeck
        self.command("north".parse()?); // Holodeck -> Hull Breach
        self.command("north".parse()?); // Hull Breach -> Stables
        self.command("east".parse()?); // Stables -> Gift Wrapping Center
                                       //self.command("take candy cane".parse()?);
        self.command("south".parse()?); // Gift Wrapping Center -> Engineering
        self.command("take hypercube".parse()?);
        self.command("north".parse()?); // Engineering -> Gift Wrapping Center
        self.command("west".parse()?); // Gift Wrapping Center -> Stables
        self.command("north".parse()?); // Stables -> Observatory
                                        //self.command("take coin".parse()?);
        self.command("east".parse()?); // Observatory -> Hallway
        self.command("take tambourine".parse()?);
        self.command("west".parse()?); // Hallway -> Observatory
        self.command("west".parse()?); // Observatory -> Arcade
        self.command("take spool of cat6".parse()?);
        self.command("north".parse()?); // Arcade -> Navigation
        self.command("take weather machine".parse()?);
        self.command("west".parse()?); // Navigation -> Hot Chocolate Fountain
                                       //self.command("take mutex".parse()?); // Navigation -> Hot Chocolate Fountain
        self.command("west".parse()?); // Hot Chocolate Fountain -> Security Checkpoint
        self.command("west".parse()?); // Security Checkpoint -> Cockpit
        Ok(())
    }
}

#[derive(Debug)]
struct Room {
    name: String,
    description: String,
    doors: collections::HashSet<Direction>,
    items: collections::HashSet<Item>,
}

/// ```text
/// == Stables ==
/// Reindeer-sized. They're all empty.
///
/// Doors here lead:
/// - north
/// - east
/// - south
///
/// Items here:
/// - escape pod
/// ```
impl str::FromStr for Room {
    type Err = &'static str;

    fn from_str(raw: &str) -> Result<Self, Self::Err> {
        let mut lines = raw
            .split('\n')
            .filter(|line| !line.is_empty())
            .rev()
            .collect::<Vec<&str>>();

        let name = match lines.pop() {
            Some(name) if name.starts_with("== ") && name.ends_with(" ==") => {
                name.trim_matches(&['=', ' '][..]).to_string()
            }
            _ => return Err("Invalid title line."),
        };

        let description = lines
            .pop()
            .ok_or_else(|| "No description line.")?
            .to_string();

        let mut doors = collections::HashSet::new();
        if let Some(&"Doors here lead:") = lines.last() {
            lines.pop();
            while let Some(direction_raw) = lines.last() {
                if let Ok(direction) = direction_raw.parse() {
                    doors.insert(direction);
                    lines.pop();
                } else {
                    break;
                }
            }
        }

        let mut items = collections::HashSet::new();
        if let Some(&"Items here:") = lines.last() {
            lines.pop();
            while let Some(item_raw) = lines.last() {
                match item_raw.parse() {
                    Ok(item) if item_raw.starts_with("- ") => {
                        items.insert(item);
                        lines.pop();
                    }
                    _ => break,
                }
            }
        }

        Ok(Room {
            name,
            description,
            doors,
            items,
        })
    }
}

#[derive(Debug, Hash, PartialEq, Eq)]
enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    fn to_str(&self) -> &'static str {
        match self {
            Direction::North => "north",
            Direction::East => "east",
            Direction::South => "south",
            Direction::West => "west",
        }
    }
}

impl str::FromStr for Direction {
    type Err = &'static str;

    fn from_str(raw: &str) -> Result<Self, Self::Err> {
        match raw.trim_matches(&['-', ' '][..]) {
            "north" => Ok(Direction::North),
            "east" => Ok(Direction::East),
            "south" => Ok(Direction::South),
            "west" => Ok(Direction::West),
            _ => Err("Invalid direction."),
        }
    }
}

impl From<&Direction> for String {
    fn from(direction: &Direction) -> Self {
        direction.to_str().to_string()
    }
}

impl fmt::Display for Direction {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        fmt.write_str(self.to_str())
    }
}

#[derive(Clone, Hash, PartialEq, Eq)]
struct Item(String);

impl Item {
    fn is_unsafe(&self) -> bool {
        match &self.0[..] {
            "infinite loop" | "escape pod" | "molten lava" => true,
            _ => false,
        }
    }
}

impl str::FromStr for Item {
    type Err = &'static str;

    fn from_str(raw: &str) -> Result<Self, Self::Err> {
        Ok(Item(raw.trim_start_matches(&['-', ' '][..]).to_string()))
    }
}

impl From<&Item> for String {
    fn from(item: &Item) -> Self {
        item.0.to_owned()
    }
}

impl fmt::Debug for Item {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        fmt.write_str("Item(\"")?;
        fmt.write_str(&self.0[..])?;
        if self.is_unsafe() {
            fmt.write_str("\" - UNSAFE)")?;
        } else {
            fmt.write_str("\")")?;
        }
        Ok(())
    }
}

impl fmt::Display for Item {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        fmt.write_str(&self.0[..])
    }
}

#[derive(Debug)]
enum Command {
    Movement(Direction),
    Take(Item),
    Drop(Item),
    List,
}

impl str::FromStr for Command {
    type Err = &'static str;

    fn from_str(raw: &str) -> Result<Self, Self::Err> {
        match raw.trim() {
            "inv" => Ok(Self::List),
            cmd if cmd.starts_with("take ") => Ok(Self::Take(cmd[5..].parse()?)),
            cmd if cmd.starts_with("drop ") => Ok(Self::Drop(cmd[5..].parse()?)),
            cmd => Ok(Command::Movement(
                cmd.parse().map_err(|_| "Unrecognized command.")?,
            )),
        }
    }
}

impl From<&Command> for String {
    fn from(command: &Command) -> Self {
        match &command {
            &Command::Movement(direction) => String::from(direction),
            &Command::Take(item) => format!("take {}", item),
            &Command::Drop(item) => format!("drop {}", item),
            &Command::List => String::from("inv"),
        }
    }
}
