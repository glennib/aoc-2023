use aoc_2023::{day_number, get_input};

type Int = i32;

fn difference(sequence: &[Int]) -> Vec<Int> {
    sequence.windows(2).map(|pair| pair[1] - pair[0]).collect()
}

fn sequences(input: &str) -> impl Iterator<Item = Vec<Int>> + '_ {
    input.lines().map(|line| {
        line.split(' ')
            .map(|number| number.parse().unwrap())
            .collect::<Vec<Int>>()
    })
}

fn pyramid(sequence: Vec<Int>) -> Vec<Vec<Int>> {
    let mut pyramid = vec![sequence];
    loop {
        let next = difference(pyramid.last().unwrap());
        if next.iter().all(|&d| d == 0) {
            break;
        }
        pyramid.push(next);
    }
    pyramid
}

fn part_one_work(input: &str) -> Int {
    let mut sum = 0;
    for sequence in sequences(input) {
        let pyramid = pyramid(sequence);
        let mut prev_diff = 0;
        for sequence in pyramid.into_iter().rev() {
            prev_diff += *sequence.last().unwrap();
        }
        sum += prev_diff;
    }

    sum
}

fn part_one(input: &str) {
    println!("part one: {}", part_one_work(input));
}

fn part_two_work(input: &str) -> Int {
    let mut sum = 0;
    for sequence in sequences(input) {
        let pyramid = pyramid(sequence);
        let mut prev_diff = 0;
        for sequence in pyramid.into_iter().rev() {
            prev_diff = *sequence.first().unwrap() - prev_diff;
        }
        sum += prev_diff;
    }

    sum
}
fn part_two(input: &str) {
    println!("part two: {}", part_two_work(input));
}

fn main() {
    let input = get_input(day_number(file!()));
    part_one(&input);
    part_two(&input);
}

#[cfg(test)]
mod tests {
    use super::*;
    const TEST_INPUT: &str = "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45";
    #[test]
    fn part_one_works() {
        assert_eq!(part_one_work(TEST_INPUT), 114);
    }
    #[test]
    fn part_two_works() {
        assert_eq!(part_two_work(TEST_INPUT), 2);
    }
}
