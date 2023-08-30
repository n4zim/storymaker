/*
 * StoryMaker - Living world generation tool
 * Copyright © 2022-2023 Nazim Lachter
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU Affero General Public License as published
 * by the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
 * GNU Affero General Public License for more details.
 *
 * You should have received a copy of the GNU Affero General Public License
 * along with this program. If not, see <https://www.gnu.org/licenses/>.
 */

use pathfinding::prelude::astar;

fn main() {
  let grid = vec![
    vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    vec![0, 1, 1, 1, 1, 1, 1, 1, 2, 0],
    vec![0, 1, 0, 0, 0, 0, 0, 0, 1, 0],
    vec![0, 1, 0, 0, 1, 1, 1, 0, 1, 0],
    vec![0, 1, 0, 0, 1, 0, 1, 0, 1, 0],
    vec![0, 1, 0, 0, 1, 0, 1, 0, 1, 0],
    vec![0, 1, 0, 0, 2, 0, 1, 0, 3, 0],
    vec![0, 1, 0, 0, 1, 0, 1, 0, 1, 0],
    vec![0, 1, 0, 0, 1, 0, 1, 0, 1, 0],
    vec![0, 3, 1, 1, 1, 1, 1, 1, 1, 0],
    vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
  ];

  let start = (1, 1);

  test("=7 (NOK)", &grid, start, 7);
  test("=2 (OK)", &grid, start, 2);
  test("=3 (OK)", &grid, start, 3);
}

const DIRECTIONS: [(isize, isize); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];

fn find_nearest(
  grid: &Vec<Vec<usize>>,
  start: (usize, usize),
  target: usize,
) -> Option<Vec<(usize, usize)>> {
  let result = astar(
    &start,
    |&node| {
      let (x, y) = node;
      let mut neighbors = Vec::new();
      for (dx, dy) in DIRECTIONS {
        let new_x = (x as isize + dx) as usize;
        let new_y = (y as isize + dy) as usize;
        if new_x < grid.len()
          && new_y < grid[0].len()
          && grid[new_x][new_y] != 0
        {
          neighbors.push(((new_x, new_y), 1));
        }
      }
      neighbors
    },
    |&node| {
      let (x, y) = node;
      let target_pos = find_position(&grid, target).unwrap_or((0, 0));
      let dx = if x > target_pos.0 {
        x - target_pos.0
      } else {
        target_pos.0 - x
      };
      let dy = if y > target_pos.1 {
        y - target_pos.1
      } else {
        target_pos.1 - y
      };
      dx + dy
    },
    |&node| grid[node.0][node.1] == target,
  );
  if let Some((path, _cost)) = result {
    Some(path.into_iter().skip(1).collect())
  } else {
    None
  }
}

fn find_position(
  grid: &Vec<Vec<usize>>,
  value: usize,
) -> Option<(usize, usize)> {
  for (x, row) in grid.iter().enumerate() {
    for (y, &cell) in row.iter().enumerate() {
      if cell == value {
        return Some((x, y));
      }
    }
  }
  None
}

fn print_grid_path(
  grid: &Vec<Vec<usize>>,
  path: &Vec<(usize, usize)>,
  start: (usize, usize),
  target: usize,
) {
  for (x, row) in grid.iter().enumerate() {
    for (y, col) in row.iter().enumerate() {
      if *col == 0 {
        if path.contains(&(x, y)) {
          print!("E");
        } else {
          print!("#");
        }
      } else if path.contains(&(x, y)) {
        if (x, y) == find_position(&grid, target).unwrap_or((0, 0)) {
          print!("G");
        } else {
          print!("*");
        }
      } else if (x, y) == start {
        print!("S");
      } else {
        print!(" ");
      }
    }
    println!();
  }
  println!();
}

fn test(
  name: &str,
  grid: &Vec<Vec<usize>>,
  start: (usize, usize),
  target: usize,
) {
  let path = find_nearest(grid, start, target);
  println!("Path {} {:?}", name, path);
  if let Some(path) = path {
    print_grid_path(grid, &path, start, target);
  }
}
