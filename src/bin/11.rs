use aoc_2023::{day_number, get_input};
use ndarray::{Array2, ArrayView};
use std::str::FromStr;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Default)]
enum GridCell {
    #[default]
    Empty,
    DoubleEmpty,
    Galaxy,
}

impl TryFrom<char> for GridCell {
    type Error = ();

    fn try_from(value: char) -> Result<Self, Self::Error> {
        let gc = match value {
            '.' => GridCell::Empty,
            '#' => GridCell::Galaxy,
            _ => {
                return Err(());
            }
        };
        Ok(gc)
    }
}

struct Grid {
    array: Array2<GridCell>,
}

impl FromStr for Grid {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut array = Array2::default((0, 0));

        for line in s.lines() {
            array
                .push_row(ArrayView::from(
                    &line
                        .chars()
                        .map(|c| GridCell::try_from(c).unwrap())
                        .collect::<Vec<_>>(),
                ))
                .unwrap();
        }

        for mut col in array.columns_mut() {
            if col.iter().all(|&gc| gc == GridCell::Empty) {
                for gc in &mut col {
                    *gc = GridCell::DoubleEmpty;
                }
            }
        }

        for mut row in array.rows_mut() {
            if row.iter().all(|&gc| gc == GridCell::Empty) {
                for gc in &mut row {
                    *gc = GridCell::DoubleEmpty;
                }
            }
        }

        Ok(Self { array })
    }
}

fn part_one_work(input: &str) -> u32 {}

fn part_one(input: &str) {
    println!("part one: {}", part_one_work(input));
}

fn part_two_work(input: &str) -> u32 {
    todo!()
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
    const TEST_INPUT: &str = "...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....";
    #[test]
    fn part_one_works() {
        assert_eq!(part_one_work(TEST_INPUT), 374);
    }
    #[test]
    fn part_two_works() {
        assert_eq!(part_two_work(TEST_INPUT), 0);
    }
}
