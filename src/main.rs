use std::{
    io::{BufWriter, Write},
    os::unix::process::CommandExt,
    process::Command,
};

use serde_json::json;
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

    fn path(&self) -> Vec<(GridPos, Direction)> {
        let mut result = vec![];
        let width = i64::try_from(self.width).unwrap();
        let start = GridPos {
            row_idx: 0,
            col_idx: width / 2,
        };
        let mut dir = Direction::East;
        let mut pos = start;
        loop {
            result.push((pos, dir.turn_right_90().turn_right_90()));
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
        result
    }

    fn draw(&self) -> Grid {
        let mut grid = Grid::builder()
            .rows(self.width)
            .cols(self.width)
            .fill_with('.')
            .build();
        for (pos, dir) in self.path() {
            let c = match dir {
                Direction::North => '^',
                Direction::East => '>',
                Direction::South => 'v',
                Direction::West => '<',
                _ => unreachable!(),
            };
            if grid.get(pos).is_some() {
                grid[pos] = c;
            }
        }
        grid
    }
}

fn main() {
    let width: usize = std::env::args().nth(1).unwrap().parse().unwrap();
    let grid = Circle::from_width(width).draw();
    eprintln!("{grid}");

    let result = Circle::from_width(width).path();

    let entities = result
        .into_iter()
        .enumerate()
        .map(|(idx, (pos, dir))| {
            let dir = match dir {
                Direction::North => 0,
                Direction::East => 4,
                Direction::South => 8,
                Direction::West => 12,
                _ => unreachable!(),
            };
            json!({
                "entity_number": idx + 1,
                "name": "transport-belt",
                "position": {
                    "x": pos.col_idx as f64 + 0.5,
                    "y": pos.row_idx as f64 + 0.5,
                },
                "direction": dir,
            })
        })
        .collect::<Vec<_>>();
    let blueprint = json!({
        "blueprint": {
            "entities": entities,
            "item": "blueprint",
            "version": 562954249961473i64,
        },
    });
    let (bpfile, bpfilepath) = tempfile::Builder::new()
        .prefix("bp")
        .suffix(".txt")
        .tempfile()
        .unwrap()
        .keep()
        .unwrap();
    let mut bufwriter = BufWriter::new(bpfile);
    serde_json::to_writer(&mut bufwriter, &blueprint).expect("writing json");
    bufwriter.flush().expect("writing json");
    drop(bufwriter);
    eprintln!("blueprint in {}", bpfilepath.display());
    Command::new("ripdrag")
        .args(["-s", "64", "-x"])
        .arg(bpfilepath)
        .spawn()
        .expect("running ripdrag")
        .wait();
}
