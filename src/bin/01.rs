use aoc_2023::MY_COOKIE;
use aoc_cache::get;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::IResult;

fn part_one(input: &str) {

    let mut s = String::with_capacity(2);

    let sum: u32 = input
        .lines()
        .filter(|&l| !l.is_empty())
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
            s.clear();
            s.push(first);
            s.push(last);
            s.parse::<u32>().unwrap()
        })
        .sum();
    println!("{sum}");
}

fn digit_word(input: &str) -> IResult<&str, &str> {
    alt((
        tag("one"),
        tag("two"),
        tag("three"),
        tag("four"),
        tag("five"),
        tag("six"),
        tag("seven"),
        tag("eight"),
        tag("nine"),
    ))(input)
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

fn digit_char(input: &str) -> IResult<&str, char>
{
    let (remaining, parsed) = take_
}

fn part_two(input: &str) {
    let sum: u32 = input
        .lines()
        .filter(|&l| !l.is_empty())
        .map(|line| {
            alt((digit_word_as_char, char::is_ascii_digit()))
        }).sum()
}

fn main() {
    let input = get("https://adventofcode.com/2023/day/1/input", MY_COOKIE).unwrap();
    part_one(&input);
    part_two(&input);
}
