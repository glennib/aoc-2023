use aoc_2023::{day_number, get_input};
use std::iter::zip;

#[derive(Clone, Copy, Debug)]
enum Pipe {
    NorthSouth,
    WestEast,
    NorthEast,
    NorthWest,
    SouthWest,
    SouthEast,
}

impl Pipe {
    fn valid_from(self, from: Direction) -> bool {
        use Direction::{East, North, South, West};
        match self {
            Pipe::NorthSouth => from == North || from == South,
            Pipe::WestEast => from == West || from == East,
            Pipe::NorthEast => from == North || from == East,
            Pipe::NorthWest => from == North || from == West,
            Pipe::SouthWest => from == West || from == South,
            Pipe::SouthEast => from == East || from == South,
        }
    }
    fn connects_to(self, from: Direction) -> Option<Direction> {
        let dir = match self {
            Pipe::NorthSouth => match from {
                Direction::North => Direction::South,
                Direction::South => Direction::North,
                _ => {
                    return None;
                }
            },
            Pipe::WestEast => match from {
                Direction::West => Direction::East,
                Direction::East => Direction::West,
                _ => {
                    return None;
                }
            },
            Pipe::NorthEast => match from {
                Direction::North => Direction::East,
                Direction::East => Direction::North,
                _ => {
                    return None;
                }
            },
            Pipe::NorthWest => match from {
                Direction::North => Direction::West,
                Direction::West => Direction::North,
                _ => {
                    return None;
                }
            },
            Pipe::SouthWest => match from {
                Direction::South => Direction::West,
                Direction::West => Direction::South,
                _ => {
                    return None;
                }
            },
            Pipe::SouthEast => match from {
                Direction::South => Direction::East,
                Direction::East => Direction::South,
                _ => {
                    return None;
                }
            },
        };
        Some(dir)
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
enum Direction {
    North,
    South,
    East,
    West,
}

impl Direction {
    fn idx_offset(self) -> (isize, isize) {
        match self {
            Direction::North => (-1, 0),
            Direction::South => (1, 0),
            Direction::East => (0, 1),
            Direction::West => (0, -1),
        }
    }
    fn opposite(self) -> Self {
        match self {
            Direction::North => Direction::South,
            Direction::South => Direction::North,
            Direction::East => Direction::West,
            Direction::West => Direction::East,
        }
    }
}

struct Map {
    tiles: Vec<Vec<Tile>>,
}

impl Map {
    fn get(&self, row: usize, col: usize) -> TileRef<'_> {
        TileRef {
            map: self,
            row,
            col,
        }
    }
}

struct MapIter<'map> {
    next_tile_ref: TileRef<'map>,
    came_from: Direction,
}

impl<'map> Iterator for MapIter<'map> {
    type Item = TileRef<'map>;

    fn next(&mut self) -> Option<Self::Item> {
        let current_tile_ref = self.next_tile_ref;
        let next_dir = current_tile_ref.next_dir(self.came_from)?;
        self.next_tile_ref = self
            .next_tile_ref
            .next_by_going_towards(next_dir)
            .expect("next_dir is valid because it was checked in the originating function call");
        self.came_from = next_dir.opposite();
        Some(current_tile_ref)
    }
}

#[derive(Copy, Clone)]
struct TileRef<'map> {
    map: &'map Map,
    row: usize,
    col: usize,
}

impl<'map> PartialEq for TileRef<'map> {
    fn eq(&self, other: &Self) -> bool {
        self.row == other.row && self.col == other.col
    }
}

impl<'map> Eq for TileRef<'map> {}

fn add((row, col): (usize, usize), (row_offset, col_offset): (isize, isize)) -> (usize, usize) {
    (
        usize::try_from(row_offset + isize::try_from(row).unwrap()).unwrap(),
        usize::try_from(col_offset + isize::try_from(col).unwrap()).unwrap(),
    )
}

impl<'map> TileRef<'map> {
    fn tile(self) -> Tile {
        self.map.tiles[self.row][self.col]
    }
    fn next_dir(self, came_from: Direction) -> Option<Direction> {
        let Tile::Pipe(p) = self.tile() else {
            return None;
        };
        if !p.valid_from(came_from) {
            return None;
        }
        // assert!(
        //     p.valid_from(came_from),
        //     "{p:?} cannot be traversed from {came_from:?}"
        // );
        p.connects_to(came_from)
    }
    fn next_by_came_from(self, came_from: Direction) -> Option<Self> {
        let next_direction = self.next_dir(came_from)?;
        self.next_by_going_towards(next_direction)
    }
    fn next_by_going_towards(mut self, going_towards: Direction) -> Option<Self> {
        let (row, col) = add((self.row, self.col), going_towards.idx_offset());
        self.row = row;
        self.col = col;
        match self.tile() {
            Tile::Ground => {
                panic!("this pipe does not lead that way");
            }
            Tile::Start => None,
            Tile::Pipe(_) => Some(self),
        }
    }
    fn into_iter(self, came_from: Direction) -> MapIter<'map> {
        MapIter {
            next_tile_ref: self,
            came_from,
        }
    }
}

impl TryFrom<char> for Pipe {
    type Error = ();
    fn try_from(value: char) -> Result<Self, Self::Error> {
        use Pipe::{NorthEast, NorthSouth, NorthWest, SouthEast, SouthWest, WestEast};
        let pipe = match value {
            '|' => NorthSouth,
            '-' => WestEast,
            'L' => NorthEast,
            'J' => NorthWest,
            '7' => SouthWest,
            'F' => SouthEast,
            _ => {
                return Err(());
            }
        };
        Ok(pipe)
    }
}

#[derive(Copy, Clone, Debug)]
enum Tile {
    Ground,
    Start,
    Pipe(Pipe),
}

impl TryFrom<char> for Tile {
    type Error = ();

    fn try_from(value: char) -> Result<Self, Self::Error> {
        let tile = match value {
            '.' => Tile::Ground,
            'S' => Tile::Start,
            _ => Tile::Pipe(Pipe::try_from(value)?),
        };
        Ok(tile)
    }
}

fn part_one_work(input: &str) -> u32 {
    let map: Vec<Vec<_>> = input
        .lines()
        .map(|line| line.chars().map(|c| Tile::try_from(c).unwrap()).collect())
        .collect();
    let map = Map { tiles: map };

    let rows = map.tiles.len();
    let cols = map.tiles.first().unwrap().len();

    // Find start
    let (start_row_idx, start_col_idx) = map
        .tiles
        .iter()
        .enumerate()
        .flat_map(|(row_idx, row)| {
            row.iter()
                .enumerate()
                .map(move |(col_idx, tile)| (row_idx, col_idx, tile))
        })
        .find_map(|(row_idx, col_idx, tile)| match tile {
            Tile::Start => Some((row_idx, col_idx)),
            _ => None,
        })
        .unwrap();

    let directions = [
        Direction::North,
        Direction::South,
        Direction::West,
        Direction::East,
    ];
    let mut connects_to_start = directions
        .into_iter()
        .map(|direction| (direction, direction.idx_offset()))
        .filter_map(|(direction, (row_offset, col_offset))| {
            let (new_row, new_col) = (
                row_offset + isize::try_from(start_row_idx).unwrap(),
                col_offset + isize::try_from(start_col_idx).unwrap(),
            );
            if new_row < 0
                || new_col < 0
                || new_row >= isize::try_from(rows).unwrap()
                || new_col >= isize::try_from(cols).unwrap()
            {
                return None;
            }
            Some((
                direction.opposite(),
                (
                    usize::try_from(new_row).unwrap(),
                    usize::try_from(new_col).unwrap(),
                ),
            ))
        })
        .filter(|(direction, (row, col))| {
            let tile = map.tiles[*row][*col];
            match tile {
                Tile::Ground => false,
                Tile::Start => {
                    unreachable!()
                }
                Tile::Pipe(pipe) => pipe.valid_from(*direction),
            }
        });
    let first_1 = connects_to_start.next().unwrap();
    let first_2 = connects_to_start.next().unwrap();
    assert_eq!(connects_to_start.next(), None);

    let first_1 = (first_1.0, map.get(first_1.1 .0, first_1.1 .1));
    let first_2 = (first_2.0, map.get(first_2.1 .0, first_2.1 .1));

    let distance = zip(
        first_1.1.into_iter(first_1.0),
        first_2.1.into_iter(first_2.0),
    )
    .enumerate()
    .find_map(|(n, (a, b))| (a == b).then_some(n + 1))
    .unwrap();

    u32::try_from(distance).unwrap()
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
    const TEST_INPUT: &str = "-L|F7
7S-7|
L|7||
-L-J|
L|-JF";
    #[test]
    fn part_one_works() {
        assert_eq!(part_one_work(TEST_INPUT), 4);
    }
    #[test]
    fn part_two_works() {
        assert_eq!(part_two_work(TEST_INPUT), 0);
    }
}
