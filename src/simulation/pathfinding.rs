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

use crate::simulation::world::WorldMap;
use bevy::utils::HashSet;
use bevy_ecs_tilemap::tiles::TilePos;
use pathfinding::prelude::astar;

const DIRECTIONS: [(isize, isize); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];

pub fn find(
  world: &WorldMap,
  start: &TilePos,
  targets: &[TilePos],
) -> Option<Vec<TilePos>> {
  //let now = std::time::Instant::now();

  let targets = targets
    .iter()
    .cloned()
    .filter(|t| *t != *start && world.is_walkable(t))
    .collect::<HashSet<TilePos>>();

  if targets.is_empty() {
    return None;
  }

  let result = astar(
    start,
    |&node| {
      let mut successors = Vec::new();
      for (dx, dy) in DIRECTIONS {
        let x = (node.x as isize + dx) as u32;
        let y = (node.y as isize + dy) as u32;
        if x < world.size.x && y < world.size.y {
          let position = TilePos { x, y };
          if world.is_walkable(&position) {
            successors.push((position, 1));
          }
        }
      }
      successors
    },
    |&node| {
      let mut cost = 0;
      for target in targets.iter() {
        let dx = if node.x > target.x {
          node.x - target.x
        } else {
          target.x - node.x
        };
        let dy = if node.y > target.y {
          node.y - target.y
        } else {
          target.y - node.y
        };
        cost += dx + dy;
      }
      cost
    },
    |&node| targets.contains(&node),
  );

  //println!("Pathfinding took {}mic", now.elapsed().as_micros());

  if let Some((path, _cost)) = result {
    Some(path)
  } else {
    None
  }
}
