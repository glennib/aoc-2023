use aoc_2023::MY_COOKIE;
use aoc_cache::get;
use rayon::iter::{IntoParallelIterator, ParallelIterator};
use std::ops::Range;
use std::str::FromStr;
use winnow::combinator::separated_pair;
use winnow::{
    ascii::dec_uint,
    combinator::{alt, rest, separated},
    error::{ContextError, ParseError},
    token::take_until1,
    PResult, Parser,
};

#[derive(Debug)]
struct Special {
    source_start: u32,
    destination_start: u32,
    length: u32,
}

impl Special {
    fn get(&self, source: u32) -> Option<u32> {
        if source < self.source_start {
            None
        } else {
            let diff = source - self.source_start;
            if diff >= self.length {
                None
            } else {
                Some(self.destination_start + diff)
            }
        }
    }
}

impl FromStr for Special {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (destination_start, _, source_start, _, length): (u32, _, u32, _, _) =
            (dec_uint, ' ', dec_uint, ' ', dec_uint)
                .parse(s)
                .map_err(|e: ParseError<_, ContextError>| e.to_string())?;
        Ok(Self {
            source_start,
            destination_start,
            length,
        })
    }
}

#[derive(Default, Debug)]
struct Map {
    special: Vec<Special>,
}

impl Map {
    fn get(&self, key: u32) -> u32 {
        self.special
            .iter()
            .find_map(|special| special.get(key))
            .unwrap_or(key)
    }
}

impl FromStr for Map {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut map = Self::default();
        for line in s.lines() {
            map.special.push(line.parse()?);
        }
        Ok(map)
    }
}

fn get_closest_location(seeds: impl IntoParallelIterator<Item = u32>, mut input: &str) -> u32 {
    fn a_map(input: &mut &str) -> PResult<Map> {
        let map_header = ("\n\n", take_until1(":"), ":\n").void();
        let map = alt((take_until1("\n\n"), rest));
        (map_header, map)
            .map(|((), map)| map)
            .map(|input| Map::from_str(input).unwrap())
            .parse_next(input)
    }

    let input = &mut input;

    let seed_to_soil = a_map.parse_next(input).unwrap();
    let soil_to_fertilizer = a_map.parse_next(input).unwrap();
    let fertilizer_to_water = a_map.parse_next(input).unwrap();
    let water_to_light = a_map.parse_next(input).unwrap();
    let light_to_temperature = a_map.parse_next(input).unwrap();
    let temperature_to_humidity = a_map.parse_next(input).unwrap();
    let humidity_to_location = a_map.parse_next(input).unwrap();

    seeds
        .into_par_iter()
        .map(|seed| seed_to_soil.get(seed))
        .map(|soil| soil_to_fertilizer.get(soil))
        .map(|fertilizer| fertilizer_to_water.get(fertilizer))
        .map(|water| water_to_light.get(water))
        .map(|light| light_to_temperature.get(light))
        .map(|temperature| temperature_to_humidity.get(temperature))
        .map(|humidity| humidity_to_location.get(humidity))
        .min()
        .unwrap()
}

fn seeds_singles(input: &mut &str) -> Vec<u32> {
    fn seeds(input: &mut &str) -> PResult<Vec<u32>> {
        ("seeds: ".void(), separated(1.., dec_uint::<_, u32, _>, ' '))
            .map(|((), seeds)| seeds)
            .parse_next(input)
    }
    seeds.parse_next(input).unwrap()
}

fn seeds_ranges(input: &mut &str) -> impl ParallelIterator<Item = u32> {
    fn parser(input: &mut &str) -> PResult<Vec<Range<u32>>> {
        let range =
            separated_pair(dec_uint, ' ', dec_uint).map(|(start, length): (_, u32)| Range::<u32> {
                start,
                end: start + length,
            });
        ("seeds: ".void(), separated(1.., range, ' '))
            .map(|((), ranges)| ranges)
            .parse_next(input)
    }
    let ranges = parser.parse_next(input).unwrap();
    ranges.into_par_iter().flatten()
}

fn part_one_work(mut input: &str) -> u32 {
    let input = &mut input;
    let seeds = seeds_singles(input);
    get_closest_location(seeds, input)
}

fn part_one(input: &str) {
    let location = part_one_work(input);
    println!("part one: {location}");
}

fn part_two_work(mut input: &str) -> u32 {
    let input = &mut input;
    let seeds = seeds_ranges(input);
    get_closest_location(seeds, input)
}

fn part_two(input: &str) {
    let location = part_two_work(input);
    println!("part two: {location}");
}

fn main() {
    let input = get("https://adventofcode.com/2023/day/5/input", MY_COOKIE).unwrap();
    part_one(&input);
    part_two(&input);
}

#[cfg(test)]
mod tests {
    use crate::{part_one_work, part_two_work};

    const TEST_INPUT: &str = "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4";
    #[test]
    fn part_one_works() {
        assert_eq!(part_one_work(TEST_INPUT), 35);
    }
    #[test]
    fn part_two_works() {
        assert_eq!(part_two_work(TEST_INPUT), 46);
    }
}
