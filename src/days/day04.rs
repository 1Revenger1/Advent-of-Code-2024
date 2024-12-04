use std::ops;

use crate::{Solution, SolutionPair};


#[derive(Clone, Copy)]
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

struct Grid {
    grid: Vec<Vec<char>>,
    max_size: Coordinates
}

impl Grid {
    fn get_grid(str: &str) -> Grid {
        let grid: Vec<Vec<char>> = str.lines().map(|l| l.chars().collect()).collect();
        let max_size = Coordinates(grid[0].len() as i32, grid.len() as i32);

        Grid {
            grid,
            max_size
        }
    }

    fn get_point(&self, coord: Coordinates) -> Option<char> {
        if !(0..self.max_size.0).contains(&coord.0) ||
            !(0..self.max_size.1).contains(&coord.1) {
            return None;
        }

        Some(self.grid[coord.1 as usize][coord.0 as usize])
    }
}

fn find_xmas(grid: &Grid, start: Coordinates) -> i32 {
    const DELTAS: [Coordinates; 8] = [
        Coordinates(1, 0),
        Coordinates(1, 1),
        Coordinates(0, 1),
        Coordinates(-1, 1),
        Coordinates(-1, 0),
        Coordinates(-1, -1),
        Coordinates(0, -1),
        Coordinates(1, -1)
    ];

    let mut count: i32 = 0;

    'delta: for d in DELTAS {
        let mut cur = start;
        for c in "MAS".chars() {
            cur += d;
            let cur_c = match grid.get_point(cur) {
                Some(c) => c,
                None => {
                    continue 'delta;
                }
            };

            if cur_c != c {
                continue 'delta;
            }
        }

        count += 1;
    }

    count
}

fn check_x_mas_pair(grid: &Grid, cor1: Coordinates, cor2: Coordinates) -> bool {
    let r1 = grid.get_point(cor1);
    let r2 = grid.get_point(cor2);
    if let None = r1 {
        return false;
    }

    if let None = r2 {
        return false;
    }

    let r1 = r1.unwrap();
    let r2 = r2.unwrap();
    (r1 == 'M' && r2 == 'S') || (r1 == 'S' && r2 == 'M')
}

fn find_x_mas(grid: &Grid, start: Coordinates) -> bool {
    const UPPER_LEFT: Coordinates = Coordinates(-1, 1);
    const UPPER_RIGHT: Coordinates = Coordinates(1, 1);
    const LOWER_LEFT: Coordinates = Coordinates(-1, -1);
    const LOWER_RIGHT: Coordinates = Coordinates(1, -1);

    check_x_mas_pair(grid, start + UPPER_LEFT, start + LOWER_RIGHT) &&
    check_x_mas_pair(grid, start + UPPER_RIGHT, start + LOWER_LEFT)
}

pub fn solve(str: String) -> SolutionPair {
    let mut sol1: i32 = 0;
    let mut sol2: u64 = 0;

    let grid = Grid::get_grid(&str);
    let Coordinates (max_y, max_x) = grid.max_size;

    for y in 0..max_y {
        for x in 0..max_x {
            let c = match grid.get_point(Coordinates(x, y)) {
                Some(c) => c,
                None => {
                    continue;
                }
            };

            if c != 'X' {
                continue;
            }

            sol1 += find_xmas(&grid, Coordinates(x, y));            
        }
    }

    for y in 0..max_y {
        for x in 0..max_x {
            let c = match grid.get_point(Coordinates(x, y)) {
                Some(c) => c,
                None => {
                    continue;
                }
            };

            if c != 'A' || !find_x_mas(&grid, Coordinates(x, y)) {
                continue;
            }

            sol2 += 1;
        }
    }

    (Solution::from(sol1), Solution::from(sol2))
}
