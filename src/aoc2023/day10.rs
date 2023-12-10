use rayon::prelude::{IndexedParallelIterator, IntoParallelRefIterator, ParallelIterator};

use crate::util::Puzzles;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Tile {
    Nothing,
    Start,
    EastSouth,
    WestSouth,
    EastNorth,
    WestNorth,
    EastWest,
    NorthSouth,
}

impl Tile {
    fn connections(&self) -> Option<[Direction; 2]> {
        match *self {
            Self::Nothing => None,
            Self::Start => None,
            Self::EastSouth => Some([Direction::East, Direction::South]),
            Self::WestSouth => Some([Direction::West, Direction::South]),
            Self::EastNorth => Some([Direction::East, Direction::North]),
            Self::WestNorth => Some([Direction::West, Direction::North]),
            Self::EastWest => Some([Direction::East, Direction::West]),
            Self::NorthSouth => Some([Direction::North, Direction::South]),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    const VALUES: [Direction; 4] = [Self::North, Self::East, Self::South, Self::West];

    fn reverse(&self) -> Self {
        match *self {
            Self::North => Self::South,
            Self::South => Self::North,
            Self::East => Self::West,
            Self::West => Self::East,
        }
    }

    fn offset(&self, pos: (usize, usize)) -> Option<(usize, usize)> {
        match *self {
            Direction::North => Some((pos.0, pos.1.checked_sub(1)?)),
            Direction::East => Some((pos.0.checked_add(1)?, pos.1)),
            Direction::South => Some((pos.0, pos.1.checked_add(1)?)),
            Direction::West => Some((pos.0.checked_sub(1)?, pos.1)),
        }
    }
}

fn parse_tile(symbol: char) -> Tile {
    match symbol {
        '.' => Tile::Nothing,
        'S' => Tile::Start,
        '|' => Tile::NorthSouth,
        '-' => Tile::EastWest,
        'L' => Tile::EastNorth,
        'J' => Tile::WestNorth,
        '7' => Tile::WestSouth,
        'F' => Tile::EastSouth,
        _ => unreachable!(),
    }
}

struct Grid {
    start: (usize, usize),
    tiles: Vec<Vec<Tile>>,
}

fn parse_grid(input: &str) -> Grid {
    let mut start = Option::None;
    let mut grid = Vec::new();
    for (y, line) in input.lines().enumerate() {
        let mut vec = vec![Tile::Nothing; line.len()];
        for (x, symbol) in line.chars().enumerate() {
            let tile = parse_tile(symbol);
            if matches!(tile, Tile::Start) {
                start = Some((x, y));
            }
            vec[x] = tile;
        }
        grid.push(vec);
    }
    return Grid {
        start: start.unwrap(),
        tiles: grid,
    };
}

pub fn register_all(puzzles: &mut Puzzles) {
    puzzles.register(crate::util::Puzzle {
        name: "Pipe Maze".to_string(),
        year: 2023,
        day: 10,
        solver: Box::new(|input| {
            let grid = parse_grid(&input.text());
            let mut size = 0;
            let mut current_dir = None;
            let mut current_pos = grid.start;
            while current_dir.is_none() || current_pos != grid.start {
                size += 1;
                let (x, y) = current_pos;
                let dir = current_dir;
                let dir = if grid.tiles[y][x] == Tile::Start {
                    *Direction::VALUES
                        .iter()
                        .find(|dir| {
                            let Some(pos) = dir.offset((x, y)) else {
                                return false;
                            };
                            let Some(conn) = grid.tiles[pos.1][pos.0].connections() else {
                                return false;
                            };
                            conn.contains(&dir.reverse())
                        })
                        .unwrap()
                } else {
                    let conn = grid.tiles[y][x].connections().unwrap();
                    *conn
                        .iter()
                        .find(|d| {
                            if d.reverse() == current_dir.unwrap() {
                                return false;
                            }
                            let Some(pos) = d.offset(current_pos) else {
                                return false;
                            };
                            grid.tiles.len() > y && grid.tiles[y].len() > x
                        })
                        .unwrap()
                };
                current_dir = Some(dir);
                current_pos = dir.offset(current_pos).unwrap();
            }

            Ok((size / 2).to_string())
        }),
    });
    puzzles.register(crate::util::Puzzle {
        name: "Part Two".to_string(),
        year: 2023,
        day: 10,
        solver: Box::new(|input| {
            let grid = parse_grid(&input.text());
            let mut loop_grid = vec![vec![false; grid.tiles[0].len()]; grid.tiles.len()];
            let mut current_dir = None;
            let mut current_pos = grid.start;
            while current_dir.is_none() || current_pos != grid.start {
                loop_grid[current_pos.1][current_pos.0] = true;
                let (x, y) = current_pos;
                let dir = current_dir;
                let dir = if grid.tiles[y][x] == Tile::Start {
                    *Direction::VALUES
                        .iter()
                        .find(|dir| {
                            let Some(pos) = dir.offset((x, y)) else {
                                return false;
                            };
                            let Some(conn) = grid.tiles[pos.1][pos.0].connections() else {
                                return false;
                            };
                            conn.contains(&dir.reverse())
                        })
                        .unwrap()
                } else {
                    let conn = grid.tiles[y][x].connections().unwrap();
                    *conn
                        .iter()
                        .find(|d| {
                            if d.reverse() == current_dir.unwrap() {
                                return false;
                            }
                            let Some(pos) = d.offset(current_pos) else {
                                return false;
                            };
                            grid.tiles.len() > y && grid.tiles[y].len() > x
                        })
                        .unwrap()
                };
                current_dir = Some(dir);
                current_pos = dir.offset(current_pos).unwrap();
            }

            //for line in &loop_grid {
            //   for s in line {
            //      print!("{:?}", *s as u8);
            // }
            //println!("");
            //}

            let size = grid
                .tiles
                .par_iter()
                .enumerate()
                .flat_map(|(y, v)| v.par_iter().enumerate().map(move |(x, _)| (x, y)))
                .filter(|(x, y)| !loop_grid[*y][*x])
                .filter(|(x, y)| {
                    let mut pos = (*x, *y);
                    let mut ortho = None;
                    let mut cuts = 0;
                    while pos.0 > 0 {
                        pos = (pos.0 - 1, pos.1);
                        if !loop_grid[pos.1][pos.0] {
                            continue;
                        }
                        match grid.tiles[pos.1][pos.0] {
                            Tile::EastWest | Tile::Nothing => {}
                            Tile::NorthSouth => cuts += 1,
                            Tile::Start => {
                                let dirs = Direction::VALUES
                                    .iter()
                                    .filter(|dir| {
                                        let Some(pos) = dir.offset(pos) else {
                                            return false;
                                        };
                                        let Some(conn) = grid.tiles[pos.1][pos.0].connections()
                                        else {
                                            return false;
                                        };
                                        conn.contains(&dir.reverse())
                                    })
                                    .collect::<Vec<_>>();
                                match &dirs[..] {
                                    &[Direction::North, Direction::South] => cuts += 1,
                                    &[Direction::North, _] => {
                                        if let Some(o) = ortho {
                                            if o != Direction::North {
                                                cuts += 1;
                                            }
                                            ortho = None;
                                        } else {
                                            ortho = Some(Direction::North);
                                        }
                                    }
                                    &[Direction::East, Direction::West] => {}
                                    _ => {
                                        if let Some(o) = ortho {
                                            if o != Direction::South {
                                                cuts += 1;
                                            }
                                            ortho = None;
                                        } else {
                                            ortho = Some(Direction::South);
                                        }
                                    }
                                }
                            }
                            tile => {
                                if let Some(o) = ortho {
                                    if o != tile.connections().unwrap()[1] {
                                        cuts += 1;
                                    }
                                    ortho = None;
                                } else {
                                    ortho = Some(tile.connections().unwrap()[1])
                                }
                            }
                        }
                    }
                    //println!("{:?} | {:?}", (x, y), cuts);
                    cuts % 2 == 1
                })
                .count();

            Ok(size.to_string())
        }),
    });
}
