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
use pathfinding::prelude::bfs;

fn main() {
  let grid = vec![
    vec![
      0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
      0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
      0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
      0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    ],
    vec![
      0, 1, 1, 1, 1, 1, 4, 1, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
      0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
      0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
      0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    ],
    vec![
      0, 1, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
      0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
      0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
      0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    ],
    vec![
      0, 1, 0, 0, 1, 1, 1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
      0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
      0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
      0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    ],
    vec![
      0, 1, 0, 0, 1, 0, 1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
      0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
      0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
      0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    ],
    vec![
      0, 1, 0, 0, 5, 0, 5, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
      0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
      0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
      0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    ],
    vec![
      0, 1, 0, 0, 2, 0, 1, 0, 3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
      0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
      0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
      0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    ],
    vec![
      0, 4, 0, 0, 1, 0, 1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
      0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
      0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
      0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    ],
    vec![
      0, 1, 0, 0, 1, 0, 1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
      0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
      0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
      0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    ],
    vec![
      0, 3, 1, 1, 1, 1, 1, 1, 5, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
      0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
      0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
      0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    ],
    vec![
      0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
      0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
      0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
      0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    ],
  ];

  let start = (1, 1);

  test_bfs("=7 (NOK)", &grid, start, 7);
  test_astar("=7 (NOK)", &grid, start, vec![]);
  test_bfs("=2 (OK)", &grid, start, 2);
  test_astar("=2 (OK)", &grid, start, vec![(1, 8), (6, 4)]);
  test_bfs("=3 (OK)", &grid, start, 3);
  test_astar("=3 (OK)", &grid, start, vec![(6, 8), (9, 1)]);
  test_bfs("=4 (OK)", &grid, start, 4);
  test_astar("=4 (OK)", &grid, start, vec![(1, 6), (7, 1)]);
  test_bfs("=5 (OK)", &grid, start, 5);
  test_astar("=5 (OK)", &grid, start, vec![(5, 4), (5, 6), (9, 8)]);
}

const DIRECTIONS: [(isize, isize); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];

fn run_bfs(
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

fn run_astar(
  grid: &Vec<Vec<usize>>,
  start: (usize, usize),
  targets: Vec<(usize, usize)>,
) -> Option<Vec<(usize, usize)>> {
  let result = astar(
    &start,
    |&node| {
      let mut successors = Vec::new();
      for (dx, dy) in DIRECTIONS {
        let x = (node.0 as isize + dx) as u32;
        let y = (node.1 as isize + dy) as u32;
        if (x as usize) < grid.len() && (y as usize) < grid[0].len() {
          if grid[x as usize][y as usize] != 0 {
            successors.push(((x as usize, y as usize), 1));
          }
        }
      }
      successors
    },
    |&node| {
      let mut cost = 0;
      for target in &targets {
        let dx = if node.0 > target.0 {
          node.0 - target.0
        } else {
          target.0 - node.0
        };
        let dy = if node.1 > target.1 {
          node.1 - target.1
        } else {
          target.1 - node.1
        };
        cost += dx + dy;
      }
      cost
    },
    |&node| targets.contains(&node),
  );
  if let Some((path, _cost)) = result {
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

fn test_bfs(
  name: &str,
  grid: &Vec<Vec<usize>>,
  start: (usize, usize),
  target: usize,
) {
  let now = std::time::Instant::now();
  let path = run_bfs(grid, start, target);
  println!(
    "[BFS] Path {} ({}ns) : {:?}",
    name,
    now.elapsed().as_nanos(),
    path,
  );
  /*if let Some(path) = path {
    print_grid_path(grid, &path, start);
  }*/
}

fn test_astar(
  name: &str,
  grid: &Vec<Vec<usize>>,
  start: (usize, usize),
  targets: Vec<(usize, usize)>,
) {
  let now = std::time::Instant::now();
  let path = run_astar(grid, start, targets);
  println!(
    "[A* ] Path {} ({}ns) : {:?}",
    name,
    now.elapsed().as_nanos(),
    path,
  );
  /*if let Some(path) = path {
    print_grid_path(grid, &path, start);
  }*/
}
