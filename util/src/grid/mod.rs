use std::{
    collections::{HashMap, HashSet, VecDeque},
    convert::Infallible,
    fmt::{Debug, Display},
    ops::{Index, IndexMut},
    str::FromStr,
};

pub struct Input {
    pub grid: Grid,
}

impl FromStr for Input {
    type Err = Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            grid: Grid::from_str(s)?,
        })
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash, PartialOrd, Ord)]
pub struct GridPos {
    pub row_idx: i64,
    pub col_idx: i64,
}

impl GridPos {
    pub const TOP_LEFT: Self = Self {
        row_idx: 0,
        col_idx: 0,
    };

    pub fn neighbors_within_manhattan_distance(self, i: i64) -> impl Iterator<Item = GridPos> {
        assert!(i >= 0);
        (-i..=i).flat_map(move |row_offset| {
            let distance_remaining = i - row_offset.abs();
            (-distance_remaining..=distance_remaining)
                .filter(move |&col_offset| (row_offset, col_offset) != (0, 0))
                .map(move |col_offset| {
                    let row_idx = self.row_idx + row_offset;
                    let col_idx = self.col_idx + col_offset;
                    GridPos { row_idx, col_idx }
                })
        })
    }

    pub fn manhattan_distance(self, other: GridPos) -> i64 {
        (self.row_idx - other.row_idx).abs() + (self.col_idx - other.col_idx).abs()
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash, PartialOrd, Ord, enum_iterator::Sequence)]
pub enum Direction {
    North,
    NorthEast,
    East,
    SouthEast,
    South,
    SouthWest,
    West,
    NorthWest,
}

impl Direction {
    pub fn all() -> impl Iterator<Item = Direction> {
        enum_iterator::all::<Self>()
    }
    pub const DIAGONALS: [Self; 4] = [
        Self::NorthEast,
        Self::SouthEast,
        Self::SouthWest,
        Self::NorthWest,
    ];

    pub const ORTHOGONALS: [Self; 4] = [Self::North, Self::East, Self::South, Self::West];

    pub fn as_offset(self) -> (i64, i64) {
        match self {
            Direction::North => (-1, 0),
            Direction::NorthEast => (-1, 1),
            Direction::East => (0, 1),
            Direction::SouthEast => (1, 1),
            Direction::South => (1, 0),
            Direction::SouthWest => (1, -1),
            Direction::West => (0, -1),
            Direction::NorthWest => (-1, -1),
        }
    }

    pub fn turn_right_90(self) -> Direction {
        match self {
            Self::North => Self::East,
            Self::NorthEast => Self::SouthEast,
            Self::East => Self::South,
            Self::SouthEast => Self::SouthWest,
            Self::South => Self::West,
            Self::SouthWest => Self::NorthWest,
            Self::West => Self::North,
            Self::NorthWest => Self::NorthEast,
        }
    }

    pub fn turn_left_90(self) -> Direction {
        // sorry
        self.turn_right_90().turn_right_90().turn_right_90()
    }

    pub fn as_char(self) -> char {
        match self {
            Direction::North => '^',
            Direction::East => '>',
            Direction::South => 'v',
            Direction::West => '<',
            _ => unimplemented!(),
        }
    }
}

impl std::ops::Add<Direction> for GridPos {
    type Output = GridPos;
    fn add(self, rhs: Direction) -> Self::Output {
        let offset = rhs.as_offset();
        GridPos {
            row_idx: self.row_idx + offset.0,
            col_idx: self.col_idx + offset.1,
        }
    }
}

impl std::ops::Sub<Direction> for GridPos {
    type Output = GridPos;
    fn sub(self, rhs: Direction) -> Self::Output {
        let offset = rhs.as_offset();
        GridPos {
            row_idx: self.row_idx - offset.0,
            col_idx: self.col_idx - offset.1,
        }
    }
}

impl std::ops::AddAssign<Direction> for GridPos {
    fn add_assign(&mut self, rhs: Direction) {
        *self = *self + rhs
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub struct GridOffset {
    pub row_offset: i64,
    pub col_offset: i64,
}

impl GridOffset {
    pub fn scale(self, scale: i64) -> Self {
        Self {
            row_offset: self.row_offset * scale,
            col_offset: self.col_offset * scale,
        }
    }
}

impl std::ops::Sub<GridPos> for GridPos {
    type Output = GridOffset;
    fn sub(self, rhs: GridPos) -> Self::Output {
        GridOffset {
            row_offset: self.row_idx - rhs.row_idx,
            col_offset: self.col_idx - rhs.col_idx,
        }
    }
}

impl std::ops::Add<GridOffset> for GridPos {
    type Output = GridPos;
    fn add(self, rhs: GridOffset) -> Self::Output {
        Self {
            row_idx: self.row_idx + rhs.row_offset,
            col_idx: self.col_idx + rhs.col_offset,
        }
    }
}
impl std::ops::AddAssign<GridOffset> for GridPos {
    fn add_assign(&mut self, rhs: GridOffset) {
        *self = *self + rhs;
    }
}

pub struct GridBuilder<Rows = (), Cols = (), Char = ()> {
    rows: Rows,
    cols: Cols,
    c: Char,
}

impl Default for GridBuilder<(), (), ()> {
    fn default() -> Self {
        Self {
            rows: (),
            cols: (),
            c: (),
        }
    }
}

impl<Cols, Char> GridBuilder<(), Cols, Char> {
    pub fn rows(self, rows: usize) -> GridBuilder<usize, Cols, Char> {
        GridBuilder {
            rows,
            cols: self.cols,
            c: self.c,
        }
    }
}
impl<Rows, Char> GridBuilder<Rows, (), Char> {
    pub fn cols(self, cols: usize) -> GridBuilder<Rows, usize, Char> {
        GridBuilder {
            rows: self.rows,
            cols,
            c: self.c,
        }
    }
}

impl<Rows, Cols> GridBuilder<Rows, Cols, ()> {
    pub fn fill_with(self, character: char) -> GridBuilder<Rows, Cols, char> {
        GridBuilder {
            rows: self.rows,
            cols: self.cols,
            c: character,
        }
    }
}

impl GridBuilder<usize, usize, char> {
    pub fn build(self) -> Grid {
        Grid((0..self.rows).map(|_| vec![self.c; self.cols]).collect())
    }
}

#[derive(PartialEq, Eq, Clone)]
pub struct Grid(pub Vec<Vec<char>>);
impl Grid {
    pub fn builder() -> GridBuilder {
        GridBuilder::default()
    }
    pub fn enumerate_chars(&self) -> impl Iterator<Item = (GridPos, char)> + use<'_> {
        self.0.iter().enumerate().flat_map(|(row_idx, row)| {
            row.iter().enumerate().map(move |(col_idx, &c)| {
                (
                    GridPos {
                        row_idx: row_idx.try_into().unwrap(),
                        col_idx: col_idx.try_into().unwrap(),
                    },
                    c,
                )
            })
        })
    }

    pub fn get(&self, pos: GridPos) -> Option<char> {
        let row_idx = usize::try_from(pos.row_idx).ok()?;
        let col_idx = usize::try_from(pos.col_idx).ok()?;
        self.0.get(row_idx)?.get(col_idx).copied()
    }

    pub fn search(
        &self,
        start: GridPos,
        direction: Direction,
    ) -> impl Iterator<Item = char> + use<'_> {
        self.enumerate_search(start, direction).map(|(_pos, c)| c)
    }

    pub fn enumerate_search(
        &self,
        start: GridPos,
        direction: Direction,
    ) -> impl Iterator<Item = (GridPos, char)> + use<'_> {
        struct SearchIterator<'a> {
            grid: &'a Grid,
            pos: GridPos,
            direction: Direction,
        }
        impl Iterator for SearchIterator<'_> {
            type Item = (GridPos, char);
            fn next(&mut self) -> Option<Self::Item> {
                let result = (self.pos, self.grid.get(self.pos)?);
                self.pos += self.direction;
                Some(result)
            }
        }
        SearchIterator {
            grid: self,
            pos: start,
            direction,
        }
    }

    pub fn columns(&self) -> impl Iterator<Item = impl Iterator<Item = char> + use<'_>> + use<'_> {
        self.enumerate_search(GridPos::TOP_LEFT, Direction::East)
            .map(move |(top_pos, _c)| self.search(top_pos, Direction::South))
    }

    pub fn take(&mut self, c: char) -> GridPos {
        let positions = self
            .enumerate_chars()
            .filter(|&(_pos, grid_c)| grid_c == c)
            .map(|(pos, _c)| pos)
            .collect::<Vec<_>>();
        let [unique_position] = positions
            .try_into()
            .unwrap_or_else(|e| panic!("expected exactly one '{c}', got {e:?}"));
        self[unique_position] = '.';
        unique_position
    }

    pub fn breadth_first_search(
        &self,
        visited: &mut HashMap<GridPos, i64>,
        allowed_directions: &[Direction],
        allowed_characters: &[char],
        start: GridPos,
        goal: Option<GridPos>,
    ) {
        let mut queue = VecDeque::<(i64, GridPos)>::from([(0, start)]);
        while let Some((steps, pos)) = queue.pop_front() {
            let Some(c) = self.get(pos) else {
                continue;
            };
            if !allowed_characters.contains(&c) {
                continue;
            }
            if visited.contains_key(&pos) {
                // because this is BFS, we should always find a shortest path first
                assert!(visited[&pos] <= steps);
                continue;
            }
            visited.insert(pos, steps);
            if Some(pos) == goal {
                return;
            }
            for &direction in allowed_directions {
                queue.push_back((steps + 1, pos + direction));
            }
        }
    }

    pub fn flood_fill(
        &self,
        start: GridPos,
        allowed_directions: &[Direction],
        allowed_characters: &[char],
        visited: &mut HashSet<GridPos>,
    ) {
        let mut todo = vec![start];
        while let Some(pos) = todo.pop() {
            let Some(c) = self.get(pos) else {
                continue;
            };
            if !allowed_characters.contains(&c) {
                continue;
            }
            if visited.contains(&pos) {
                continue;
            }
            visited.insert(pos);
            for &direction in allowed_directions {
                todo.push(pos + direction);
            }
        }
    }
}

impl FromStr for Grid {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(s.lines().map(|line| line.chars().collect()).collect()))
    }
}

impl Display for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in &self.0 {
            let row = row.iter().copied().collect::<String>();
            writeln!(f, "{}", row)?;
        }
        Ok(())
    }
}

impl Debug for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if !f.alternate() {
            #[allow(clippy::recursive_format_impl)]
            return write!(f, "{:#?}", self);
        }
        struct DisplayAsDebug<'a>(&'a Grid);
        impl Debug for DisplayAsDebug<'_> {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                Display::fmt(self.0, f)
            }
        }
        f.debug_tuple("Grid").field(&DisplayAsDebug(self)).finish()
    }
}

impl Index<GridPos> for Grid {
    type Output = char;
    fn index(&self, index: GridPos) -> &Self::Output {
        let row_idx: usize = index.row_idx.try_into().expect("nonnegative row index");
        let col_idx: usize = index.col_idx.try_into().expect("nonnegative column index");
        &self.0[row_idx][col_idx]
    }
}

impl IndexMut<GridPos> for Grid {
    fn index_mut(&mut self, index: GridPos) -> &mut Self::Output {
        let row_idx: usize = index.row_idx.try_into().expect("in range row index");
        let col_idx: usize = index.col_idx.try_into().expect("in range column index");
        &mut self.0[row_idx][col_idx]
    }
}

pub trait IteratorExt {
    fn filter_char_eq(&mut self, c: char) -> impl Iterator<Item = GridPos>
    where
        Self: Iterator<Item = (GridPos, char)>,
    {
        self.filter_map(move |(pos, grid_c)| (c == grid_c).then_some(pos))
    }
}
impl<T: Iterator> IteratorExt for T {}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    #[test]
    fn test_grid_display() {
        let text = indoc! {"
            AB
            CD
        "};
        let grid: Grid = text.parse().unwrap();
        assert_eq!(grid.to_string(), text)
    }

    #[test]
    fn test_grid_debug() {
        let text = indoc! {"
            AB
            CD
        "};
        let grid: Grid = text.parse().unwrap();
        let expected = indoc! {"
            Grid(
                AB
                CD
                ,
            )"};
        assert_eq!(format!("{:?}", grid), expected);
    }

    #[test]
    fn test_neigbors_within_manhattan_distance() {
        let mut start_grid: Grid = indoc! {"
            .......
            .......
            ...@...
            .......
            .......
        "}
        .parse()
        .unwrap();
        let start = start_grid.take('@');
        // but actually, put it back lol
        start_grid[start] = '@';
        assert_eq!(start.neighbors_within_manhattan_distance(0).count(), 0);
        {
            let mut grid = start_grid.clone();
            for pos in start.neighbors_within_manhattan_distance(1) {
                grid[pos] = '#'
            }
            let expected: Grid = indoc! {"
                .......
                ...#...
                ..#@#..
                ...#...
                .......
            "}
            .parse()
            .unwrap();
            assert_eq!(grid, expected);
        }
        {
            let mut grid = start_grid.clone();
            for pos in start.neighbors_within_manhattan_distance(2) {
                grid[pos] = '#'
            }
            let expected: Grid = indoc! {"
                ...#...
                ..###..
                .##@##.
                ..###..
                ...#...
            "}
            .parse()
            .unwrap();
            assert_eq!(grid, expected);
        }
    }
}
