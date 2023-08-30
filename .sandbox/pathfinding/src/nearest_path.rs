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

use pathfinding::prelude::bfs;

fn main() {
  let grid = vec![
    vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    vec![0, 1, 1, 1, 1, 1, 4, 1, 2, 0],
    vec![0, 1, 0, 0, 0, 0, 0, 0, 1, 0],
    vec![0, 1, 0, 0, 1, 1, 1, 0, 1, 0],
    vec![0, 1, 0, 0, 1, 0, 1, 0, 1, 0],
    vec![0, 1, 0, 0, 5, 0, 5, 0, 1, 0],
    vec![0, 1, 0, 0, 2, 0, 1, 0, 3, 0],
    vec![0, 4, 0, 0, 1, 0, 1, 0, 1, 0],
    vec![0, 1, 0, 0, 1, 0, 1, 0, 1, 0],
    vec![0, 3, 1, 1, 1, 1, 1, 1, 5, 0],
    vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
  ];

  let start = (1, 1);

  test("=7 (NOK)", &grid, start, 7);
  test("=2 (OK)", &grid, start, 2);
  test("=3 (OK)", &grid, start, 3);
  test("=4 (OK)", &grid, start, 4);
  test("=5 (OK)", &grid, start, 5);
}

const DIRECTIONS: [(isize, isize); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];

fn find_nearest(
  grid: &Vec<Vec<usize>>,
  start: (usize, usize),
  target: usize,
) -> Option<Vec<(usize, usize)>> {
  let result = bfs(
    &start,
    |&node| {
      let mut successors = Vec::new();
      for &(x, y) in DIRECTIONS.iter() {
        let x = node.0 as isize + x;
        let y = node.1 as isize + y;
        if x >= 0
          && x < grid.len() as isize
          && y >= 0
          && y < grid[0].len() as isize
        {
          let nx = x as usize;
          let ny = y as usize;
          if grid[nx][ny] != 0 {
            successors.push((nx, ny));
          }
        }
      }
      successors
    },
    |&(x, y)| grid[x][y] == target,
  );
  if let Some(path) = result {
    Some(path.into_iter().skip(1).collect())
  } else {
    None
  }
}

fn print_grid_path(
  grid: &Vec<Vec<usize>>,
  path: &Vec<(usize, usize)>,
  start: (usize, usize),
) {
  let last = path.last().unwrap();
  for (x, row) in grid.iter().enumerate() {
    for (y, col) in row.iter().enumerate() {
      if *col == 0 {
        if path.contains(&(x, y)) {
          print!("E");
        } else {
          print!("#");
        }
      } else if path.contains(&(x, y)) {
        if (x, y) == *last {
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
    print_grid_path(grid, &path, start);
  }
}
