use aoc_2023::{day_number, MY_COOKIE};
use aoc_cache::get;
use num::Integer;
use std::{collections::HashMap, iter::repeat, str::FromStr};
use winnow::{
    combinator::{delimited, rest, separated_pair},
    error::ContextError,
    stream::AsChar,
    token::take_while,
    PResult, Parser,
};

struct Directions(Vec<Direction>);

impl FromStr for Directions {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(
            s.chars()
                .map(Direction::try_from)
                .collect::<Result<Vec<_>, _>>()?,
        ))
    }
}

#[derive(Eq, PartialEq, Hash, Copy, Clone, Debug)]
struct Name<'a>(&'a str);
impl<'a> Name<'a> {
    fn is_start(&self) -> bool {
        self.0.ends_with('A')
    }
    fn is_goal(&self) -> bool {
        self.0.ends_with('Z')
    }
}

#[derive(Debug)]
struct Node<'a> {
    name: Name<'a>,
    left: Name<'a>,
    right: Name<'a>,
}

#[derive(Debug)]
struct Children<'a> {
    left: Name<'a>,
    right: Name<'a>,
}

#[derive(Clone)]
enum Direction {
    Left,
    Right,
}

impl TryFrom<char> for Direction {
    type Error = String;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        Ok(match value {
            'L' => Direction::Left,
            'R' => Direction::Right,
            c => return Err(format!("{c} does not correspond to a direction")),
        })
    }
}

impl<'a> From<&'a str> for Name<'a> {
    fn from(value: &'a str) -> Self {
        Self(value)
    }
}

impl<'a> TryFrom<&'a str> for Node<'a> {
    type Error = String;

    fn try_from(value: &'a str) -> Result<Self, Self::Error> {
        fn name<'a>(input: &mut &'a str) -> PResult<&'a str> {
            take_while(1.., AsChar::is_alphanum).parse_next(input)
        }
        let (name, left, right) = (
            name,
            " = ".void(),
            delimited('(', separated_pair(name, ", ", name), ')'),
        )
            .map(|(name, (), (left, right))| (name, left, right))
            .parse(value)
            .map_err(|e| e.to_string())?;
        let node = Node {
            name: Name(name),
            left: Name(left),
            right: Name(right),
        };
        Ok(node)
    }
}

fn part_one_work(input: &str) -> u32 {
    let (directions, (), nodes) = (
        take_while(1.., AsChar::is_alpha),
        "\n\n".void(),
        rest::<_, ContextError>,
    )
        .parse(input)
        .unwrap();
    let directions: Directions = directions.parse().unwrap();
    let nodes: Vec<_> = nodes
        .lines()
        .map(Node::try_from)
        .collect::<Result<_, _>>()
        .unwrap();
    let mut map = HashMap::with_capacity(nodes.len());
    for Node { name, left, right } in nodes {
        map.insert(name, Children { left, right });
    }
    let map = map;

    let goal = Name("ZZZ");

    let mut current = Name("AAA");

    let mut directions = repeat(directions.0.into_iter()).flatten();
    let mut steps = 0;

    while current != goal {
        steps += 1;
        let children = map.get(&current).unwrap();
        current = match directions.next().unwrap() {
            Direction::Left => children.left,
            Direction::Right => children.right,
        };
    }

    steps
}

fn part_one(input: &str) {
    println!("part one: {}", part_one_work(input));
}

fn part_two_work(input: &str) -> u64 {
    let (directions, (), nodes) = (
        take_while(1.., AsChar::is_alpha),
        "\n\n".void(),
        rest::<_, ContextError>,
    )
        .parse(input)
        .unwrap();
    let directions: Directions = directions.parse().unwrap();
    let nodes: Vec<_> = nodes
        .lines()
        .map(Node::try_from)
        .collect::<Result<_, _>>()
        .unwrap();
    let mut map = HashMap::with_capacity(nodes.len());
    for Node { name, left, right } in nodes {
        map.insert(name, Children { left, right });
    }
    let map = map;

    let currents = map.keys().copied().filter(Name::is_start);

    let directions = repeat(directions.0.into_iter()).flatten();

    let mut lcm = 1;

    for mut current in currents {
        let mut directions = directions.clone();
        let mut step = 0;
        while !current.is_goal() {
            step += 1;
            let direction = directions.next().unwrap();
            match direction {
                Direction::Left => {
                    current = map.get(&current).unwrap().left;
                }
                Direction::Right => {
                    current = map.get(&current).unwrap().right;
                }
            }
        }
        lcm = lcm.lcm(&step);
    }

    lcm
}

fn part_two(input: &str) {
    println!("part two: {}", part_two_work(input));
}

fn main() {
    let input = get(
        &format!(
            "https://adventofcode.com/2023/day/{}/input",
            day_number(file!())
        ),
        MY_COOKIE,
    )
    .unwrap();
    part_one(&input);
    part_two(&input);
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT_1: &str = "RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)";
    const TEST_INPUT_2: &str = "LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)";
    const TEST_INPUT_3: &str = "LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)";

    #[test]
    fn part_one_works() {
        assert_eq!(part_one_work(TEST_INPUT_1), 2);
        assert_eq!(part_one_work(TEST_INPUT_2), 6);
    }

    #[test]
    fn part_two_works() {
        assert_eq!(part_two_work(TEST_INPUT_3), 6);
    }
}
