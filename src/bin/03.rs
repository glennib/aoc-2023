use aoc_2023::MY_COOKIE;
use aoc_cache::get;
use std::ops::{Range, RangeInclusive, Sub};
use winnow::ascii::dec_uint;
use winnow::combinator::{alt, repeat, repeat_till0};
use winnow::error::ContextError;
use winnow::token::any;
use winnow::{Located, Parser};

#[derive(Debug)]
struct NumberLocation {
    row: usize,
    columns_plus: RangeInclusive<usize>,
}

#[derive(Debug)]
struct Number {
    number: u32,
    location: NumberLocation,
}

fn to_inclusive<Idx: Sub<Idx, Output = Idx> + From<u8>>(range: Range<Idx>) -> RangeInclusive<Idx> {
    RangeInclusive::new(range.start, range.end - 1u8.into())
}

impl NumberLocation {
    fn from(row: usize, columns: RangeInclusive<usize>) -> Self {
        Self {
            row,
            columns_plus: {
                RangeInclusive::new(columns.start().saturating_sub(1), columns.end() + 1)
            },
        }
    }
    fn adjacent_to(&self, symbol_location: &SymbolLocation) -> bool {
        let row_adjacent = self.row == symbol_location.row
            || (self.row + 1) == symbol_location.row
            || self.row == (symbol_location.row + 1);
        row_adjacent && self.columns_plus.contains(&symbol_location.column)
    }
}

impl SymbolLocation {
    fn adjacent_to(&self, number_location: &NumberLocation) -> bool {
        number_location.adjacent_to(self)
    }
}

#[derive(Debug)]
struct SymbolLocation {
    row: usize,
    column: usize,
}

#[derive(Debug)]
struct Symbol {
    symbol: char,
    location: SymbolLocation,
}

fn get_part_numbers_and_symbols(input: &str) -> (Vec<Number>, Vec<Symbol>) {
    enum Entity {
        Symbol(Symbol),
        Number(Number),
    }
    let entities = input
        .lines()
        .enumerate()
        .map(|(row, line)| {
            let mut line = Located::new(line);
            let symbol = any::<_, ContextError>
                .verify(|&c| c != '.')
                .with_span()
                .map(|(symbol, range)| {
                    Entity::Symbol(Symbol {
                        symbol,
                        location: SymbolLocation {
                            row,
                            column: range.start,
                        },
                    })
                });
            let part_number = dec_uint.with_span().map(|(number, span)| {
                Entity::Number(Number {
                    number,
                    location: NumberLocation::from(row, to_inclusive(span)),
                })
            });
            let nothing = '.'.void();
            let something = alt((part_number, symbol));
            let entities: Vec<Entity> = repeat(
                ..,
                repeat_till0(nothing, something).map(|((), entity): ((), _)| entity),
            )
            .parse_next(&mut line)
            .unwrap();
            entities
        })
        .reduce(|mut acc, entities| {
            acc.extend(entities);
            acc
        })
        .unwrap();
    let mut symbols = Vec::new();
    let mut part_numbers = Vec::new();
    for entity in entities {
        match entity {
            Entity::Symbol(symbol) => {
                symbols.push(symbol);
            }
            Entity::Number(number) => {
                part_numbers.push(number);
            }
        }
    }
    (part_numbers, symbols)
}

fn get_sum_of_part_numbers(input: &str) -> u32 {
    let (part_numbers, symbols) = get_part_numbers_and_symbols(input);

    // dbg!(&part_numbers);
    // dbg!(&symbols);

    part_numbers
        .into_iter()
        .filter_map(|number| {
            symbols
                .iter()
                .any(|symbol| number.location.adjacent_to(&symbol.location))
                .then_some(number.number)
        })
        .sum()
}

fn get_sum_of_gear_ratios(input: &str) -> u32 {
    let (part_numbers, symbols) = get_part_numbers_and_symbols(input);
    symbols
        .into_iter()
        .filter_map(|symbol| (symbol.symbol == '*').then_some(symbol.location))
        .filter_map(|symbol_location| {
            let mut gear_ratio = 1;
            let mut count = 0;
            for part_number in &part_numbers {
                if !part_number.location.adjacent_to(&symbol_location) {
                    continue;
                }
                count += 1;
                if count > 2 {
                    return None;
                }
                gear_ratio *= part_number.number;
            }
            if count == 2 {
                Some(gear_ratio)
            } else {
                None
            }
        })
        .sum()
}

fn part_one(input: &str) {
    let sum = get_sum_of_part_numbers(input);
    println!("part one: {sum}");
}

fn part_two(input: &str) {
    let sum = get_sum_of_gear_ratios(input);
    println!("part one: {sum}");
}

fn main() {
    let input = get("https://adventofcode.com/2023/day/3/input", MY_COOKIE).unwrap();
    part_one(&input);
    part_two(&input);
}

#[cfg(test)]
mod tests {
    use crate::{get_sum_of_gear_ratios, get_sum_of_part_numbers, to_inclusive};
    use std::ops::{Range, RangeInclusive};

    const TEST_INPUT: &str = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";

    #[test]
    fn to_inclusive_works() {
        let original = Range { start: 10, end: 20 };
        assert_eq!(to_inclusive(original), RangeInclusive::new(10, 19));
    }
    #[test]
    fn part_one_works() {
        assert_eq!(get_sum_of_part_numbers(TEST_INPUT), 4361);
    }

    #[test]
    fn part_two_works() {
        assert_eq!(get_sum_of_gear_ratios(TEST_INPUT), 467835);
    }
}
