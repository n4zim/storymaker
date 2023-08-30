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

use super::Actor;
use crate::simulation::world::WorldMap;
use crate::{game::GameTick, simulation::pathfinding::find};
use bevy::prelude::*;
use bevy_ecs_tilemap::tiles::{TileColor, TilePos};
use rand::Rng;

pub fn random_system(
  mut events: EventReader<GameTick>,
  mut query: Query<(&mut Actor, &mut TileColor, &mut TilePos)>,
  world: Res<WorldMap>,
) {
  for _clock in events.iter() {
    for (mut actor, mut color, mut position) in query.iter_mut() {
      if actor.path.is_empty() {
        let mut rng: rand::rngs::ThreadRng = rand::thread_rng();
        loop {
          let destination = TilePos {
            x: rng.gen_range(0..world.size.x),
            y: rng.gen_range(0..world.size.y),
          };
          if let Some(path) = find(&world, &position, &[destination]) {
            actor.path = path;
            break;
          }
        }
      }

      actor.set_next_posture();

      let destination = actor.path.remove(0);
      position.x = destination.x;
      position.y = destination.y;

      if world.is_walkable(&destination) {
        color.0 = Color::Rgba {
          red: 1.0,
          green: 1.0,
          blue: 1.0,
          alpha: 1.0,
        };
      } else {
        color.0 = Color::RED;
      }
    }
  }
}
