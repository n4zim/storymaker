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
    vec![
      false, false, false, false, false, false, false, false, false, false,
    ],
    vec![false, true, true, true, true, true, true, true, true, false],
    vec![
      false, true, false, false, false, false, false, false, true, false,
    ],
    vec![
      false, true, false, false, true, true, true, false, true, false,
    ],
    vec![
      false, true, false, false, true, false, true, false, true, false,
    ],
    vec![
      false, true, false, false, true, false, true, false, true, false,
    ],
    vec![
      false, true, false, false, true, false, true, false, true, false,
    ],
    vec![
      false, true, false, false, true, false, true, false, true, false,
    ],
    vec![
      false, true, false, false, true, false, true, false, true, false,
    ],
    vec![false, true, true, true, true, true, true, true, true, false],
    vec![
      false, false, false, false, false, false, false, false, false, false,
    ],
  ];

  let start = (1, 1);

  test("1 (NOK)", &grid, start, (8, 2));
  test("2 (NOK)", &grid, start, (7, 2));
  test("3 (OK)", &grid, start, (8, 8));
  test("4 (OK)", &grid, start, (1, 8));
  test("5 (OK)", &grid, start, (1, 4));
  test("6 (OK)", &grid, start, (1, 3));
  test("7 (OK)", &grid, start, (3, 4));
  test("8 (OK)", &grid, start, (6, 6));
  test("8 (OK)", &grid, start, (6, 7));
}

const DIRECTIONS: [(isize, isize); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];

fn find_path(
  grid: &Vec<Vec<bool>>,
  start: (usize, usize),
  goal: (usize, usize),
) -> Option<Vec<(usize, usize)>> {
  let result = astar(
    &start,
    |&node| {
      let mut successors = Vec::new();
      for (dx, dy) in DIRECTIONS {
        let x = (node.0 as isize + dx) as usize;
        let y = (node.1 as isize + dy) as usize;
        if x < grid.len() && y < grid[0].len() && grid[x][y] {
          successors.push(((x, y), 1));
        }
      }
      successors
    },
    |&node| {
      let dx = if node.0 > goal.0 {
        node.0 - goal.0
      } else {
        goal.0 - node.0
      };
      let dy = if node.1 > goal.1 {
        node.1 - goal.1
      } else {
        goal.1 - node.1
      };
      dx + dy
    },
    |&node| node == goal,
  );
  if let Some((path, _cost)) = result {
    Some(path.into_iter().skip(1).collect())
  } else {
    None
  }
}

fn print_grid_path(
  grid: &Vec<Vec<bool>>,
  path: &Vec<(usize, usize)>,
  start: (usize, usize),
  goal: (usize, usize),
) {
  for (x, row) in grid.iter().enumerate() {
    for (y, col) in row.iter().enumerate() {
      if !col {
        if path.contains(&(x, y)) {
          print!("E");
        } else {
          print!("#");
        }
      } else if path.contains(&(x, y)) {
        if (x, y) == goal {
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
  grid: &Vec<Vec<bool>>,
  start: (usize, usize),
  goal: (usize, usize),
) {
  let path = find_path(grid, start, goal);
  println!("Path {} {:?}", name, path);
  if let Some(path) = path {
    print_grid_path(grid, &path, start, goal);
  }
}
