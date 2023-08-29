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

use super::super::world::World;
use super::{Actor, ActorDirection};
use crate::game::GameTick;
use bevy::prelude::*;
use bevy_ecs_tilemap::tiles::{TilePos, TileTextureIndex};
use rand::Rng;

pub fn move_system(
  mut events: EventReader<GameTick>,
  mut query: Query<(&mut Actor, &mut TilePos)>,
  world: Res<World>,
) {
  for clock in events.iter() {
    for (mut actor, mut position) in query.iter_mut() {
      if clock.total % 20 != 0 {
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

      if leave {
        actor.destination = None;
      }
    }
  }
}

pub fn directions_system(mut query: Query<(&mut Actor, &mut TilePos)>) {
  for (mut actor, position) in query.iter_mut() {
    if actor.destination.is_some() {
      let destination = actor.destination.unwrap();
      actor.direction = if position.x < destination.x {
        if position.y < destination.y {
          ActorDirection::BottomRight
        } else if position.y > destination.y {
          ActorDirection::Bottom
        } else {
          ActorDirection::Right
        }
      } else if position.x > destination.x {
        if position.y < destination.y {
          ActorDirection::Top
        } else if position.y > destination.y {
          ActorDirection::TopLeft
        } else {
          ActorDirection::Left
        }
      } else if position.y < destination.y {
        ActorDirection::TopRight
      } else {
        ActorDirection::BottomLeft
      };
    }
  }
}

pub fn animations_system(
  mut query: Query<(&mut Actor, &mut TileTextureIndex)>,
) {
  for (actor, mut texture_index) in query.iter_mut() {
    texture_index
      .set(Box::new(actor.get_texture_index()))
      .unwrap()
  }
}
