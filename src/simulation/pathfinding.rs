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
use bevy_ecs_tilemap::tiles::TilePos;
use pathfinding::prelude::astar;

const DIRECTIONS: [(isize, isize); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];

pub fn find_target_path(
  world: &WorldMap,
  start: TilePos,
  target: TilePos,
) -> Option<Vec<TilePos>> {
  if !world.is_walkable(&target) {
    return None;
  }
  let result = astar(
    &start,
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
      dx + dy
    },
    |&node| node == target,
  );
  if let Some((path, _cost)) = result {
    Some(path.into_iter().skip(1).collect())
  } else {
    None
  }
}
