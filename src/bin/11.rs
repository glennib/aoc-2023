use aoc_2023::{day_number, get_input};
use itertools::Itertools;
use ndarray::{Array2, ArrayView};
use pathfinding::prelude::astar;
use std::collections::{HashMap, HashSet};
use std::fmt::{Debug, Display, Formatter, Write};
use std::str::FromStr;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Default)]
enum GridCell {
    #[default]
    Empty,
    DoubleEmpty,
    Galaxy,
}

impl GridCell {
    fn to_char(self) -> char {
        match self {
            GridCell::Empty => '⋅',
            GridCell::DoubleEmpty => '⋄',
            GridCell::Galaxy => '⋇',
        }
    }
}

type GalaxyId = usize;
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
struct GalaxyIdPair {
    lower: GalaxyId,
    higher: GalaxyId,
}

impl GalaxyIdPair {
    fn new(id_1: GalaxyId, id_2: GalaxyId) -> Self {
        Self {
            lower: id_1.min(id_2),
            higher: id_1.max(id_2),
        }
    }
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

impl Display for Grid {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for row in self.array.rows() {
            for gc in row {
                f.write_char(gc.to_char())?;
            }
            f.write_char('\n')?;
        }
        Ok(())
    }
}

impl Grid {
    fn galaxy_idx(&self) -> impl Iterator<Item = (usize, usize)> + '_ {
        self.array
            .rows()
            .into_iter()
            .enumerate()
            .flat_map(|(row_idx, row)| {
                row.into_iter()
                    .enumerate()
                    .filter_map(move |(col_idx, gc)| {
                        if let GridCell::Galaxy = gc {
                            Some((row_idx, col_idx))
                        } else {
                            None
                        }
                    })
            })
    }
    fn galaxy_id_idx(&self) -> HashMap<GalaxyId, (usize, usize)> {
        self.galaxy_idx().enumerate().collect()
    }
}

impl FromStr for Grid {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let cols = s.lines().next().unwrap().len();
        let mut array = Array2::default((0, cols));

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

        let empty_cols: Vec<_> = array
            .columns()
            .into_iter()
            .enumerate()
            .filter_map(|(col_idx, col)| {
                col.iter()
                    .all(|&gc| gc == GridCell::Empty)
                    .then_some(col_idx)
            })
            .collect();

        let empty_rows: Vec<_> = array
            .rows()
            .into_iter()
            .enumerate()
            .filter_map(|(row_idx, row)| {
                row.iter()
                    .all(|&gc| gc == GridCell::Empty)
                    .then_some(row_idx)
            })
            .collect();

        for col in empty_cols {
            for gc in array.column_mut(col) {
                *gc = GridCell::DoubleEmpty;
            }
        }

        for row in empty_rows {
            for gc in array.row_mut(row) {
                *gc = GridCell::DoubleEmpty;
            }
        }

        Ok(Self { array })
    }
}

fn shortest_path_length(
    start: (usize, usize),
    goal: (usize, usize),
    grid: &Grid,
    double_cost: u32,
) -> u32 {
    let (_, length) = astar(
        &start,
        |&idx| {
            const DIRECTIONS: [isize; 3] = [-1_isize, 0, 1];
            fn add(
                (row, col): (usize, usize),
                (row_d, col_d): (isize, isize),
            ) -> Option<(usize, usize)> {
                fn add(a: usize, r: isize) -> Option<usize> {
                    usize::try_from(isize::try_from(a).unwrap() + r).ok()
                }
                Some((add(row, row_d)?, add(col, col_d)?))
            }
            let directions = DIRECTIONS.into_iter();
            let directions = directions
                .clone()
                .cartesian_product(directions)
                .filter(|(row, col)| row != col);
            directions.filter_map(move |d| {
                let new_idx = add(idx, d)?;
                let gc = grid.array.get(new_idx)?;
                let cost = if let GridCell::DoubleEmpty = gc {
                    double_cost
                } else {
                    1
                };
                Some((new_idx, cost))
            })
        },
        |_| 0,
        |&current| current == goal,
    )
    .unwrap();
    length
}

fn part_one_work(input: &str) -> u32 {
    let grid: Grid = input.parse().unwrap();
    println!("{grid}");
    let galaxy_id_idx = grid.galaxy_id_idx();
    let galaxy_ids = galaxy_id_idx.keys().copied();
    let pairs: HashSet<_> = galaxy_ids
        .clone()
        .cartesian_product(galaxy_ids)
        .map(|(id_1, id_2)| GalaxyIdPair::new(id_1, id_2))
        .collect();

    pairs
        .into_iter()
        .map(
            |GalaxyIdPair {
                 lower: a,
                 higher: b,
             }| {
                let a = *galaxy_id_idx.get(&a).unwrap();
                let b = *galaxy_id_idx.get(&b).unwrap();
                shortest_path_length(a, b, &grid, 2)
            },
        )
        .sum()
}

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
    // part_two(&input);
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
