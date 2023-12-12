use aoc_2023::MY_COOKIE;
use aoc_cache::get;
use std::collections::{HashMap, HashSet, VecDeque};
use std::str::FromStr;
use winnow::ascii::{dec_uint, space1};
use winnow::combinator::separated;
use winnow::{PResult, Parser};

struct WinningNumbers(HashSet<u32>);
struct OwnNumbers(HashSet<u32>);

struct CardWithId {
    id: u32,
    card: Card,
}

struct Card {
    winning_numbers: WinningNumbers,
    own_numbers: OwnNumbers,
}

impl Card {
    fn winning(&self) -> u32 {
        u32::try_from(
            self.own_numbers
                .0
                .intersection(&self.winning_numbers.0)
                .count(),
        )
        .unwrap()
    }
    fn points(&self) -> u32 {
        let winning = self.winning();
        if winning == 0 {
            return 0;
        }
        2u32.pow(winning - 1)
    }
}

impl FromStr for CardWithId {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        fn hs(input: &mut &str) -> PResult<HashSet<u32>> {
            separated(1.., dec_uint, space1).parse_next(input)
        }
        let winning = hs.map(WinningNumbers);
        let own = hs.map(OwnNumbers);
        (
            "Card".void(),
            space1.void(),
            dec_uint,
            ":".void(),
            space1.void(),
            winning,
            space1.void(),
            "|".void(),
            space1.void(),
            own,
        )
            .map(
                |((), (), id, (), (), winning_numbers, (), (), (), own_numbers)| CardWithId {
                    id,
                    card: Card {
                        winning_numbers,
                        own_numbers,
                    },
                },
            )
            .parse(s)
            .map_err(|e| e.to_string())
    }
}

fn parse_cards(input: &str) -> Vec<CardWithId> {
    input.lines().map(|line| line.parse().unwrap()).collect()
}

fn points(cards: &[CardWithId]) -> u32 {
    cards
        .iter()
        .map(|card_with_id| card_with_id.card.points())
        .sum()
}

fn to_map(cards: impl IntoIterator<Item = CardWithId>) -> HashMap<u32, Card> {
    let cards = cards.into_iter();
    let mut map = if let Some(size) = cards.size_hint().1 {
        HashMap::with_capacity(size)
    } else {
        HashMap::new()
    };
    for card in cards {
        map.insert(card.id, card.card);
    }
    map
}

fn part_one(input: &str) {
    let points = points(&parse_cards(input));
    println!("part one: {points}");
}

fn count_cards(input: &str) -> u32 {
    let cards = to_map(parse_cards(input));
    let mut count = 0;
    let mut unchecked: VecDeque<_> = cards.keys().copied().collect();

    while let Some(id) = unchecked.pop_front() {
        count += 1;
        let wins = cards.get(&id).unwrap().winning();
        for idx in (1..=wins).rev() {
            let copied_id = id + idx;
            unchecked.push_front(copied_id);
        }
    }
    count
}

fn part_two(input: &str) {
    let cards = count_cards(input);
    println!("part two: {cards}");
}

fn main() {
    let input = get("https://adventofcode.com/2023/day/4/input", MY_COOKIE).unwrap();
    part_one(&input);
    part_two(&input);
}

#[cfg(test)]
mod tests {
    use crate::{count_cards, parse_cards, points};

    const TEST_INPUT: &str = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";
    #[test]
    fn test_parse_cards() {
        let cards = parse_cards(TEST_INPUT);
        assert_eq!(cards.len(), 6);
    }

    #[test]
    fn part_one_works() {
        let points = points(&parse_cards(TEST_INPUT));
        assert_eq!(points, 13);
    }

    #[test]
    fn part_two_works() {
        let cards = count_cards(TEST_INPUT);
        assert_eq!(cards, 30);
    }
}
