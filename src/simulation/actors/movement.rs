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

use super::super::world::WorldMap;
use super::Actor;
use crate::game::GameTick;
use bevy::prelude::*;
use bevy_ecs_tilemap::tiles::{TileColor, TilePos};
use rand::Rng;

pub fn random_system(
  mut events: EventReader<GameTick>,
  mut query: Query<(&mut Actor, &mut TileColor, &mut TilePos)>,
  world: Res<WorldMap>,
) {
  for clock in events.iter() {
    for (mut actor, mut color, mut position) in query.iter_mut() {
      if clock.total % 10 != 0 {
        continue;
      }

      if actor.destination.is_none() {
        let mut rng: rand::rngs::ThreadRng = rand::thread_rng();
        actor.destination = Some(TilePos {
          x: rng.gen_range(10..world.size.x - 10),
          y: rng.gen_range(10..world.size.y - 10),
        });
      }

      actor.set_next_posture();

      let destination = actor.destination.unwrap();
      let mut leave = false;

      if position.x != destination.x {
        if position.x < destination.x {
          position.x += 1;
        } else {
          position.x -= 1;
        }
      } else {
        leave = true;
      }

      if position.y != destination.y {
        if position.y < destination.y {
          position.y += 1;
        } else {
          position.y -= 1;
        }
        leave = false;
      }

      if world.is_walkable(&position) {
        color.0 = Color::Rgba {
          red: 1.0,
          green: 1.0,
          blue: 1.0,
          alpha: 1.0,
        };
      } else {
        color.0 = Color::RED;
      }

      if leave {
        actor.destination = None;
      }
    }
  }
}
