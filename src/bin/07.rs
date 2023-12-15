use aoc_2023::{day_number, MY_COOKIE};
use aoc_cache::get;
use itertools::Itertools;
use std::{
    collections::HashMap,
    fmt::{Debug, Display},
    str::FromStr,
};
use winnow::{
    ascii::{dec_uint, multispace0},
    combinator::{opt, separated_pair},
    error::{ContextError, ParseError},
    token::take,
    Parser,
};

#[derive(Copy, Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
enum Card {
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace,
}

#[derive(Copy, Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
enum Card2 {
    Joker,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Queen,
    King,
    Ace,
}

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
enum Class {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

type FiveCards<C> = [C; 5];

#[must_use]
fn classify(cards: FiveCards<Card>) -> Class {
    let mut count = HashMap::<_, u8>::new();
    for card in cards {
        *count.entry(card).or_default() += 1;
    }
    let count = count;

    if count.len() == 1 {
        return Class::FiveOfAKind;
    }

    if count.values().any(|&c| c == 4u8) {
        return Class::FourOfAKind;
    }

    if count.len() == 2 && count.values().any(|&c| c == 2 || c == 3) {
        return Class::FullHouse;
    }

    if count.values().any(|&c| c == 3) {
        return Class::ThreeOfAKind;
    }

    if count.values().filter(|&&c| c == 2).count() == 2 {
        return Class::TwoPair;
    }

    if count.len() < 5 {
        return Class::OnePair;
    }

    Class::HighCard
}

fn classify_2(cards: FiveCards<Card2>) -> Class {
    let number_of_jokers =
        u8::try_from(cards.iter().filter(|&&c| c == Card2::Joker).count()).unwrap();
    let cards_without_jokers = cards.into_iter().filter(|&c| c != Card2::Joker);
    let mut count = HashMap::<_, u8>::new();

    for card in cards_without_jokers {
        *count.entry(card).or_default() += 1;
    }

    if count.len() == 1 {
        return Class::FiveOfAKind;
    }

    if count.values().any(|&c| c == 4 - number_of_jokers) {
        return Class::FourOfAKind;
    }

    todo!();
    if count.len() == 2 && count.values().any(|&c| c == 3 || c == 2) {
        return Class::FullHouse;
    }

    if count.values().any(|&c| c == 3 - number_of_jokers) {
        return Class::ThreeOfAKind;
    }

    todo!();
    if count.values().filter(|&&c| c == 2).count() == 2 {
        return Class::TwoPair;
    }

    if count.values().any(|&v| v == 2 - number_of_jokers) {
        return Class::OnePair;
    }

    Class::HighCard
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
struct Hand<C> {
    class: Class,
    cards: FiveCards<C>,
}

impl Hand<Card> {
    fn new(cards: FiveCards<Card>) -> Self {
        let class = classify(cards);
        Self { class, cards }
    }
}

impl Hand<Card2> {
    fn new_2(cards: FiveCards<Card2>) -> Self {
        let class = classify_2(cards);
        Self { class, cards }
    }
}

fn parse_cards<C, EC, EV>(s: &str) -> Result<FiveCards<C>, String>
where
    FiveCards<C>: TryFrom<Vec<C>, Error = EV>,
    EV: Debug,
    C: TryFrom<char, Error = EC>,
    EC: Display,
{
    let mut cards = take(5u8).map(|s: &str| {
        s.chars()
            .map(|c| C::try_from(c).map_err(|e| format!("cannot convert '{e}' to Card")))
            .collect()
    });
    let cards: Result<Vec<_>, _> = cards
        .parse(s)
        .map_err(|e: ParseError<_, ContextError>| e.to_string())?;
    let cards = cards?;
    FiveCards::<C>::try_from(cards).map_err(|v| format!("cannot create five cards from {v:?}"))
}

impl FromStr for Hand<Card> {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        parse_cards(s).map(Self::new)
    }
}

impl FromStr for Hand<Card2> {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        parse_cards(s).map(Self::new_2)
    }
}

impl TryFrom<char> for Card {
    type Error = char;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        #[allow(clippy::enum_glob_use)]
        use Card::*;
        let card = match value {
            'A' => Ace,
            'K' => King,
            'Q' => Queen,
            'J' => Jack,
            'T' => Ten,
            '9' => Nine,
            '8' => Eight,
            '7' => Seven,
            '6' => Six,
            '5' => Five,
            '4' => Four,
            '3' => Three,
            '2' => Two,
            c => {
                return Err(c);
            }
        };
        Ok(card)
    }
}
impl TryFrom<char> for Card2 {
    type Error = char;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        #[allow(clippy::enum_glob_use)]
        use Card2::*;
        let card = match value {
            'A' => Ace,
            'K' => King,
            'Q' => Queen,
            'T' => Ten,
            '9' => Nine,
            '8' => Eight,
            '7' => Seven,
            '6' => Six,
            '5' => Five,
            '4' => Four,
            '3' => Three,
            '2' => Two,
            'J' => Joker,
            c => {
                return Err(c);
            }
        };
        Ok(card)
    }
}

struct HandAndBid<C> {
    hand: Hand<C>,
    bid: u32,
}

impl<C> FromStr for HandAndBid<C>
where
    Hand<C>: FromStr<Err = String>,
{
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (cards, bid) = separated_pair(take(5u8), ' ', (dec_uint, opt(multispace0).void()))
            .map(|(cards, (bid, ())): (_, (_, ()))| (cards, bid))
            .parse(s)
            .map_err(|e: ParseError<_, ContextError>| e.to_string())?;
        let hand = cards.parse()?;
        let hab = Self { hand, bid };
        Ok(hab)
    }
}

fn part_one_work(input: &str) -> u32 {
    input
        .lines()
        .map(|line| line.parse::<HandAndBid<Card>>().unwrap())
        .sorted_by(|a, b| a.hand.cmp(&b.hand))
        .enumerate()
        .map(|(rank, hab)| {
            let rank = u32::try_from(rank + 1).unwrap();
            rank * hab.bid
        })
        .sum()
}

fn part_one(input: &str) {
    println!("part one: {}", part_one_work(input));
}

fn part_two_work(_input: &str) -> u32 {
    todo!()
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
    // part_two(&input);
}

#[cfg(test)]
mod tests {
    use super::*;
    const TEST_INPUT: &str = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";
    #[test]
    fn part_one_works() {
        assert_eq!(part_one_work(TEST_INPUT), 6440);
    }
    #[test]
    fn part_two_works() {
        assert_eq!(part_two_work(TEST_INPUT), 0);
    }
    #[test]
    fn classifies() {
        assert_eq!(
            classify([Card::Ace, Card::Ace, Card::Eight, Card::Ace, Card::Ace]),
            Class::FourOfAKind
        );
    }
}
