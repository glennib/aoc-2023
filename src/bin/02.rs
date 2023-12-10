use aoc_2023::MY_COOKIE;
use aoc_cache::get;

#[derive(Debug)]
struct Game {
    id: u32,
    max: Max,
}

impl Game {
    fn from(id: u32, sets: &[Set]) -> Self {
        Self {
            id,
            max: Max::from_sets(sets),
        }
    }

    fn valid(&self, total_reds: u32, total_greens: u32, total_blues: u32) -> bool {
        self.max.reds <= total_reds
            && self.max.greens <= total_greens
            && self.max.blues <= total_blues
    }

    fn power(&self) -> u32 {
        self.max.reds * self.max.greens * self.max.blues
    }
}

#[derive(Default, Debug)]
struct Set {
    reds: u32,
    greens: u32,
    blues: u32,
}

#[derive(Default, Debug)]
struct Max {
    reds: u32,
    greens: u32,
    blues: u32,
}

impl Max {
    fn from_sets(sets: &[Set]) -> Self {
        let mut max = Self::default();
        for set in sets {
            max.reds = max.reds.max(set.reds);
            max.greens = max.greens.max(set.greens);
            max.blues = max.blues.max(set.blues);
        }
        max
    }
}

mod w {
    use super::{Game, Set};
    use winnow::{
        ascii::dec_uint,
        combinator::{alt, separated},
        PResult, Parser,
    };

    pub fn count<'c>(color: &'c str) -> impl FnMut(&mut &str) -> PResult<u32> + 'c {
        move |input| {
            (dec_uint, " ", color)
                .map(|(count, _, _)| count)
                .parse_next(input)
        }
    }

    pub fn set(input: &mut &str) -> PResult<Set> {
        enum ColorCount {
            Red(u32),
            Green(u32),
            Blue(u32),
        }
        let blue = count("blue").map(ColorCount::Blue);
        let red = count("red").map(ColorCount::Red);
        let green = count("green").map(ColorCount::Green);
        let mut colors = separated(1..=3, alt((blue, red, green)), ", ");
        let counts: Vec<ColorCount> = colors.parse_next(input)?;
        let mut set = Set::default();

        for count in counts {
            match count {
                ColorCount::Red(c) => {
                    set.reds = c;
                }
                ColorCount::Green(c) => {
                    set.greens = c;
                }
                ColorCount::Blue(c) => {
                    set.blues = c;
                }
            }
        }

        Ok(set)
    }

    pub fn sets(input: &mut &str) -> PResult<Vec<Set>> {
        separated(1.., set, "; ").parse_next(input)
    }

    pub fn id(input: &mut &str) -> PResult<u32> {
        ("Game ", dec_uint).map(|(_, id)| id).parse_next(input)
    }

    pub fn game(input: &mut &str) -> PResult<Game> {
        (id, ": ", sets)
            .map(|(id, _, sets)| Game::from(id, &sets))
            .parse_next(input)
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn parses_id() {
            assert_eq!(id(&mut "Game 42"), Ok(42));
        }

        #[test]
        fn parses_count() {
            let mut red = count("red");
            assert_eq!(red.parse_next(&mut "30 red"), Ok(30));
        }
    }
}

fn get_sum_of_valid_game_ids(input: &str) -> u32 {
    input
        .lines()
        .map(|mut line| w::game(&mut line).unwrap())
        .filter_map(|game| game.valid(12, 13, 14).then_some(game.id))
        .sum()
}

fn get_power_of_all_games(input: &str) -> u32 {
    input
        .lines()
        .map(|mut line| w::game(&mut line).unwrap())
        .map(|game| game.power())
        .sum()
}

fn part_one(input: &str) {
    let s = get_sum_of_valid_game_ids(input);
    println!("part one: {s}");
}
fn part_two(input: &str) {
    let p = get_power_of_all_games(input);
    println!("part two: {p}");
}

fn main() {
    let input = get("https://adventofcode.com/2023/day/2/input", MY_COOKIE).unwrap();
    part_one(&input);
    part_two(&input);
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";
    #[test]
    fn part_one_works() {
        assert_eq!(get_sum_of_valid_game_ids(TEST_INPUT), 8);
    }

    #[test]
    fn part_two_works() {
        assert_eq!(get_power_of_all_games(TEST_INPUT), 2286);
    }
}
