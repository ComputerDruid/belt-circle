use std::array::from_fn;

use util::grid::{Direction, Grid, GridPos};

struct Circle {
    width: usize,
}

fn clockwise_x4(mut dir: Direction) -> [Direction; 4] {
    dir = dir.turn_left_90();
    let dir = &mut dir;
    std::array::from_fn(|_| {
        *dir = dir.turn_right_90();
        *dir
    })
}

impl Circle {
    pub fn from_width(width: usize) -> Self {
        Self { width }
    }

    fn inside_circle(&self, pos: GridPos) -> bool {
        let radius = i64::try_from(self.width).unwrap();
        // work in doubled coordinate space to handle even widths
        let transformed = GridPos {
            row_idx: pos.row_idx * 2 - radius + 1,
            col_idx: pos.col_idx * 2 - radius + 1,
        };
        let radius = radius;
        (transformed.row_idx * transformed.row_idx + transformed.col_idx * transformed.col_idx)
            < (radius * radius)
    }

    fn draw(&self) -> Grid {
        let mut grid = Grid::builder()
            .rows(self.width)
            .cols(self.width)
            .fill_with('.')
            .build();
        let width = i64::try_from(self.width).unwrap();
        let start = GridPos {
            row_idx: 0,
            col_idx: width / 2,
        };
        let mut dir = Direction::East;
        let mut pos = start;
        loop {
            let c = match (dir.turn_right_90().turn_right_90()) {
                Direction::North => '^',
                Direction::East => '>',
                Direction::South => 'v',
                Direction::West => '<',
                _ => unreachable!(),
            };
            if grid.get(pos).is_some() {
                grid[pos] = c;
            }
            dir = dir.turn_left_90();
            dir = clockwise_x4(dir)
                .into_iter()
                .find(|&dir| self.inside_circle(pos + dir))
                .expect("there should exist a valid direction to go");
            pos += dir;
            if pos == start {
                break;
            }
        }
        grid
    }

    fn fill(&self) -> Grid {
        let mut grid = Grid::builder()
            .rows(self.width + 2)
            .cols(self.width + 2)
            .fill_with('.')
            .build();
        for (r, row) in grid.0.iter_mut().enumerate() {
            for (c, ch) in row.iter_mut().enumerate() {
                let pos = GridPos {
                    row_idx: i64::try_from(r).unwrap() - 1,
                    col_idx: i64::try_from(c).unwrap() - 1,
                };
                if self.inside_circle(pos) {
                    *ch = '*';
                }
            }
        }
        grid
    }
}

fn main() {
    let width: usize = std::env::args().nth(1).unwrap().parse().unwrap();
    //println!("{}", Circle::from_width(width).fill());
    let grid = Circle::from_width(width).draw();
    println!("{grid}");
}
