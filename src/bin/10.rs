use aoc_2023::{day_number, get_input};
use std::{collections::HashSet, iter::zip, str::FromStr};

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

    fn directions(self) -> (Direction, Direction) {
        use Direction::{East, North, South, West};
        match self {
            Pipe::NorthSouth => (North, South),
            Pipe::WestEast => (West, East),
            Pipe::NorthEast => (North, East),
            Pipe::NorthWest => (North, West),
            Pipe::SouthWest => (South, West),
            Pipe::SouthEast => (South, East),
        }
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

    /// Finds the start tile and the two valid directions to go towards
    fn start(&self) -> (TileRef<'_>, Direction, Direction) {
        const DIRECTIONS: [Direction; 4] = [
            Direction::North,
            Direction::West,
            Direction::South,
            Direction::East,
        ];
        let (start_row_idx, start_col_idx) = self
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

        let tile_ref = self.get(start_row_idx, start_col_idx);
        let mut connecting_directions = DIRECTIONS
            .into_iter()
            .filter(|&dir| tile_ref.next_by_going_towards(dir).is_some());
        let dir_1 = connecting_directions.next().unwrap();
        let dir_2 = connecting_directions.next().unwrap();
        assert_eq!(connecting_directions.next(), None);
        (tile_ref, dir_1, dir_2)
    }

    fn sanitize(mut self) -> Self {
        let (start, d, _) = self.start();
        let route: HashSet<_> = start
            .into_iter(d)
            .map(|tile| (tile.row, tile.col))
            .collect();

        let n_rows = self.tiles.len();
        let n_cols = self.tiles.first().unwrap().len();

        for row in 0..n_rows {
            for col in 0..n_cols {
                if !route.contains(&(row, col)) {
                    self.tiles[row][col] = Tile::Ground;
                }
            }
        }
        self
    }
}

impl FromStr for Map {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let map = s
            .lines()
            .map(|line| line.chars().map(|c| Tile::try_from(c).unwrap()).collect())
            .collect();
        Ok(Self { tiles: map })
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
        if let Tile::Pipe(p) = self.tile() {
            if p.valid_from(going_towards.opposite()) {
                Some(self)
            } else {
                None
            }
        } else {
            None
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
    let map: Map = input.parse().unwrap();

    let (start, dir_1_towards, dir_2_towards) = map.start();

    let first_1 = start.next_by_going_towards(dir_1_towards).unwrap();
    let first_2 = start.next_by_going_towards(dir_2_towards).unwrap();

    let distance = zip(
        first_1.into_iter(dir_1_towards.opposite()),
        first_2.into_iter(dir_2_towards.opposite()),
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
    let map: Map = input.parse().unwrap();
    let map = map.sanitize();

    let (start, dir_1, dir_2) = map.start();

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
