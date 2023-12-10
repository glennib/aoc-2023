use aoc_2023::{match_and_move_1, MY_COOKIE};
use aoc_cache::get;
use winnow::combinator::repeat_till0;
use winnow::stream::Accumulate;
use winnow::token::any;
use winnow::{
    combinator::{repeat, Alt},
    token::one_of,
    PResult, Parser,
};

fn part_one(input: &str) {
    let mut buffer = String::with_capacity(2);

    let sum: u32 = input
        .lines()
        .map(|line| {
            let first = line
                .chars()
                .filter(char::is_ascii_digit)
                .take(1)
                .next()
                .unwrap();
            let last = line
                .chars()
                .rev()
                .filter(char::is_ascii_digit)
                .take(1)
                .next()
                .unwrap();
            buffer.clear();
            buffer.push(first);
            buffer.push(last);
            buffer.parse::<u32>().unwrap()
        })
        .sum();
    println!("part one: {sum}");
}

fn digit_char(input: &mut &str) -> PResult<char> {
    one_of(('0'..='9',)).parse_next(input)
}

fn digit_word(input: &mut &str) -> PResult<char> {
    (
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    )
        .choice(input)
        .map(|word| {
            digit_word_as_char(word)
                .expect("a successful match on the preceding choice will return a char")
        })
}

fn digit_word_as_char(word: &str) -> Option<char> {
    match word {
        "one" => Some('1'),
        "two" => Some('2'),
        "three" => Some('3'),
        "four" => Some('4'),
        "five" => Some('5'),
        "six" => Some('6'),
        "seven" => Some('7'),
        "eight" => Some('8'),
        "nine" => Some('9'),
        _ => None,
    }
}

fn digit(input: &mut &str) -> PResult<char> {
    (digit_char, digit_word).choice(input)
}

struct FirstAndLast<T> {
    first: Option<T>,
    last: Option<T>,
}

impl<T: Copy> FirstAndLast<T> {
    fn first(&self) -> T {
        self.first.unwrap()
    }
    fn last(&self) -> T {
        self.last.unwrap_or(self.first())
    }
}

impl<T> Accumulate<T> for FirstAndLast<T> {
    fn initial(_capacity: Option<usize>) -> Self {
        Self {
            first: None,
            last: None,
        }
    }

    fn accumulate(&mut self, acc: T) {
        if self.first.is_none() {
            self.first = Some(acc);
        } else {
            self.last = Some(acc);
        }
    }
}

fn part_two(input: &str) {
    let mut buffer = String::with_capacity(2);
    let sum: u32 = input
        .lines()
        .map(|mut line| {
            let first_and_last: FirstAndLast<_> = repeat(
                1..,
                repeat_till0(any, match_and_move_1(digit)).map(|((), c)| c),
            )
            .parse_next(&mut line)
            .unwrap();
            first_and_last
        })
        .map(|digits| {
            buffer.clear();
            buffer.push(digits.first());
            buffer.push(digits.last());
            buffer.parse::<u32>().unwrap()
        })
        .sum();
    println!("part two: {sum}");
}

fn main() {
    let input = get("https://adventofcode.com/2023/day/1/input", MY_COOKIE).unwrap();
    part_one(&input);
    part_two(&input);
}

#[cfg(test)]
mod tests {
    use crate::digit_word;
    use winnow::Parser;

    #[test]
    fn digit_word_err() {
        let input = &mut "";
        assert!(digit_word.parse_next(input).is_err());
    }
    #[test]
    fn digit_word_ok() {
        assert_eq!(digit_word.parse_next(&mut "one"), Ok('1'));
    }
}
