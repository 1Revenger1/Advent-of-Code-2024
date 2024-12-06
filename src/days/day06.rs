use std::{collections::HashSet, ops};

use crate::{Solution, SolutionPair};

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
struct Coordinates(i32, i32);

impl ops::AddAssign<Coordinates> for Coordinates {
    fn add_assign(&mut self, rhs: Coordinates) {
        self.0 += rhs.0;
        self.1 += rhs.1;
    }

}

impl ops::Add<Coordinates> for Coordinates {
    type Output = Coordinates;

    fn add(self, _rhs: Coordinates) -> Coordinates {
        return Coordinates(self.0 + _rhs.0, self.1 + _rhs.1)
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Tile {
    Obstacle,
    FreeSpace
}

struct Grid {
    grid: Vec<Vec<Tile>>,
    max_size: Coordinates,
    start: Coordinates
}

impl Grid {
    fn get_grid(str: &str) -> Grid {
        let mut start = Coordinates(0, 0);
        let grid: Vec<Vec<Tile>> = str.lines().enumerate().map(|(y, l)| l.chars().enumerate().map(|(x, c)| {
            match c {
                '#' => Tile::Obstacle,
                '.' => Tile::FreeSpace,
                '^' => {
                    start = Coordinates(x as i32, y as i32);
                    Tile::FreeSpace
                }
                _ => panic!()
            }
        }).collect()).collect();
        let max_size = Coordinates(grid[0].len() as i32, grid.len() as i32);

        Grid {
            grid,
            max_size,
            start
        }
    }

    fn get_point(&self, coord: Coordinates) -> Option<Tile> {
        if !(0..self.max_size.0).contains(&coord.0) ||
            !(0..self.max_size.1).contains(&coord.1) {
            return None;
        }

        Some(self.grid[coord.1 as usize][coord.0 as usize])
    }
}

pub fn solve(str: String) -> SolutionPair {
    const DIRS: [Coordinates; 4] = [
        Coordinates(0, -1),
        Coordinates(1, 0),
        Coordinates(0, 1),
        Coordinates(-1, 0)
    ];

    let mut grid = Grid::get_grid(&str);
    let mut visited: HashSet<Coordinates> = HashSet::new();
    let mut dir = 0;
    let mut loc = grid.start;
    loop {
        visited.insert(loc);
        match grid.get_point(loc + DIRS[dir]) {
            Some(t) => {
                if t == Tile::Obstacle {
                    dir = (dir + 1) % DIRS.len();
                } else {
                    loc = loc + DIRS[dir];
                }
            },
            None => break
        }
    }

    let mut sol2: u64 = 0;
    for &Coordinates (x, y) in &visited {
        if Coordinates(x, y) == grid.start {
            continue;
        }

        grid.grid[y as usize][x as usize] = Tile::Obstacle;

        let mut visited: HashSet<(Coordinates, usize)> = HashSet::new();
        let mut dir = 0;
        let mut loc = grid.start;
        loop {
            if !visited.insert((loc, dir)) {
                sol2 += 1;
                break;
            }

            match grid.get_point(loc + DIRS[dir]) {
                Some(t) => {
                    if t == Tile::Obstacle {
                        dir = (dir + 1) % DIRS.len();
                    } else {
                        loc = loc + DIRS[dir];
                    }
                },
                None => break
            }
        }
            
        grid.grid[y as usize][x as usize] = Tile::FreeSpace;
    }

    (Solution::from(visited.len()), Solution::from(sol2))
}
