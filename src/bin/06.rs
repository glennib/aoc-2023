use aoc_2023::MY_COOKIE;
use aoc_cache::get;
use std::fmt::Debug;
use std::ops::Add;
use std::{iter::zip, str::FromStr};
use winnow::{
    ascii::{dec_uint, multispace1},
    combinator::separated,
    error::{ContextError, ParseError},
    Parser,
};

// fn distance_mm(hold_ms: u16, total_ms: u16) -> u16 {
//     if hold_ms >= total_ms {
//         return 0;
//     }
//     (total_ms - hold_ms) * hold_ms
// }

fn hold_ms(total_ms: f64, distance_mm: f64) -> Option<RealQuadraticSolution> {
    solve_quadratic(1., -total_ms, distance_mm).map(Into::into)
}

// math
// distance = (total - hold) * hold
// find hold
// distance = total hold - hold^2
// hold^2 - total * hold + distance = 0
// 1 * x^2 + (-total) * x + distance = 0
// a * x^2 + b * x + c = 0
// a = 1, b = -total, c = distance
// x = (-b +- sqrt(b^2 - 4 * a * c))/(2*a)

#[derive(Debug)]
enum RealQuadraticSolution {
    Single(f64),
    Double { pos: f64, neg: f64 },
}
fn solve_quadratic(a: f64, b: f64, c: f64) -> Option<RealQuadraticSolution> {
    let square = b * b - 4. * a * c;
    if square < 0. {
        return None;
    }
    if square == 0. {
        return Some(RealQuadraticSolution::Single(-b / (2. * a)));
    }
    let rooted = square.sqrt();
    let pos = (-b + rooted) / (2. * a);
    let neg = (-b - rooted) / (2. * a);
    Some(RealQuadraticSolution::Double { pos, neg })
}

struct Race<U> {
    total_ms: U,
    distance_mm: U,
}

impl<U: Copy + Into<u64> + Add<U, Output = U>> Race<U> {
    fn ways_to_beat(&self) -> u64 {
        let total_ms = self.total_ms.into();
        let current_record_distance_mm = self.distance_mm.into();
        #[allow(clippy::cast_precision_loss)]
        let hold_ms_for_record = hold_ms(
            total_ms as _,
            current_record_distance_mm as _,
        )
        .unwrap();
        let RealQuadraticSolution::Double { neg, pos } = hold_ms_for_record else {
            panic!("single solution");
        };
        #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
        let sol = (((pos - 0.000_001).floor()) - (neg + 0.000_001).ceil()) as u64 + 1;
        sol
    }
}

struct Races {
    races: Vec<Race<u16>>,
}

impl FromStr for Races {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut times = (
            "Time:".void(),
            multispace1.void(),
            separated(1.., dec_uint::<_, u16, _>, multispace1),
        )
            .map(|((), (), times)| times);
        let mut distances = (
            "Distance:".void(),
            multispace1.void(),
            separated(1.., dec_uint::<_, u16, _>, multispace1),
        )
            .map(|((), (), distances)| distances);
        let mut lines = s.lines();
        let times_input = lines.next().unwrap();
        let distances_input = lines.next().unwrap();
        let times: Vec<_> = times
            .parse(times_input)
            .map_err(|e: ParseError<_, ContextError>| e.to_string())?;
        let distances: Vec<_> = distances
            .parse(distances_input)
            .map_err(|e: ParseError<_, ContextError>| e.to_string())?;
        assert_eq!(times.len(), distances.len());
        let races = zip(times, distances)
            .map(|(time_ms, distance_mm)| Race {
                total_ms: time_ms,
                distance_mm,
            })
            .collect();
        Ok(Self { races })
    }
}

struct RacePartTwo {
    race: Race<u64>,
}

impl FromStr for RacePartTwo {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut time_parts = (
            "Time:".void(),
            multispace1.void(),
            separated(1.., dec_uint::<_, u16, _>.recognize(), multispace1),
        )
            .map(|((), (), number_parts): ((), (), Vec<&str>)| number_parts);
        let mut distance_parts = (
            "Distance:".void(),
            multispace1.void(),
            separated(1.., dec_uint::<_, u16, _>.recognize(), multispace1),
        )
            .map(|((), (), number_parts): ((), (), Vec<&str>)| number_parts);
        let mut lines = s.lines();
        let time_input = lines.next().unwrap();
        let distance_input = lines.next().unwrap();
        let time_parts: Vec<_> = time_parts
            .parse(time_input)
            .map_err(|e: ParseError<_, ContextError>| e.to_string())?;
        let distance_parts: Vec<_> = distance_parts
            .parse(distance_input)
            .map_err(|e: ParseError<_, ContextError>| e.to_string())?;

        let time = time_parts
            .into_iter()
            .fold(String::new(), |acc, part| acc + part)
            .parse()
            .unwrap();
        let distance = distance_parts
            .into_iter()
            .fold(String::new(), |acc, part| acc + part)
            .parse()
            .unwrap();

        Ok(Self {
            race: Race {
                total_ms: time,
                distance_mm: distance,
            },
        })
    }
}

fn part_one_work(input: &str) -> u64 {
    Races::from_str(input)
        .unwrap()
        .races
        .into_iter()
        .map(|race| race.ways_to_beat())
        // .inspect(|ways_to_beat| println!("ways_to_beat={ways_to_beat}"))
        .product()
}

fn part_one(input: &str) {
    let res = part_one_work(input);
    println!("part one: {res}");
}

fn part_two_work(input: &str) -> u64 {
    RacePartTwo::from_str(input).unwrap().race.ways_to_beat()
}

fn part_two(input: &str) {
    let res = part_two_work(input);
    println!("part two: {res}");
}

fn main() {
    let input = get("https://adventofcode.com/2023/day/6/input", MY_COOKIE).unwrap();
    part_one(&input);
    part_two(&input);
}

#[cfg(test)]
mod tests {
    use crate::{part_one_work, part_two_work};

    const TEST_INPUT: &str = "Time:      7  15   30
Distance:  9  40  200";
    #[test]
    fn part_one_works() {
        let moe = part_one_work(TEST_INPUT);
        assert_eq!(moe, 288);
    }
    #[test]
    fn part_two_works() {
        let moe = part_two_work(TEST_INPUT);
        assert_eq!(moe, 71503);
    }
}
