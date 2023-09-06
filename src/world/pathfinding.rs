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

use super::map::WorldMap;
use bevy_ecs_tilemap::tiles::TilePos;
use pathfinding::prelude::astar;

const DIRECTIONS: [(isize, isize); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];

pub fn path_from_to(
  world: &WorldMap,
  start: &TilePos,
  targets: &Vec<TilePos>,
) -> Option<Vec<TilePos>> {
  //let now = std::time::Instant::now();

  /*let targets = targets
  .iter()
  .cloned()
  //.filter(|t| *t != *start && world.is_walkable(t))
  .collect::<HashSet<TilePos>>();*/

  if targets.is_empty() {
    //println!("No targets");
    return None;
  }

  let mut targets = targets.clone();
  targets.sort_by(|a, b| {
    let da = distance(start, a);
    let db = distance(start, b);
    da.cmp(&db)
  });
  // To speed up the pathfinding, we only consider the closest target
  let target = targets[0];

  let result = astar(
    start,
    |&node| {
      let mut successors = Vec::new();
      for (dx, dy) in DIRECTIONS {
        let x = (node.x as isize + dx) as u32;
        let y = (node.y as isize + dy) as u32;
        if x < world.size.x && y < world.size.y {
          let position = TilePos { x, y };
          if world.is_walkable(&position) || position == target {
            successors.push((position, 1));
          }
        }
      }
      successors
    },
    |&node| {
      distance(&node, &target)
      /*let mut cost = 0;
      for target in targets.iter() {
        cost += distance(&node, &target);
      }
      cost*/
    },
    |&node| node == target,
    //|&node| targets.contains(&node),
  );

  //println!("Pathfinding took {}ms", now.elapsed().as_millis());
  //println!("Pathfinding result: {:?}", result);

  if let Some((path, _cost)) = result {
    Some(path)
  } else {
    None
  }
}

fn distance(a: &TilePos, b: &TilePos) -> u32 {
  let dx = if a.x > b.x { a.x - b.x } else { b.x - a.x };
  let dy = if a.y > b.y { a.y - b.y } else { b.y - a.y };
  dx + dy
}
