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

use crate::{game::GameTick, pathfinding::paths, world::WorldMap, characters::{CharacterDirection, Character}};
use bevy::prelude::*;
use bevy_ecs_tilemap::tiles::{TileColor, TilePos, TileTextureIndex};
use big_brain::prelude::*;
use rand::Rng;

#[derive(Component, ActionBuilder, Clone, Debug)]
pub struct Action {
  path: Vec<TilePos>,
}

pub fn system(
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
          if let Some(path) = paths(&world, &position, &[destination]) {
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

pub fn directions_system(mut query: Query<(&mut Character, &mut TilePos, &)>) {
  for (mut actor, position) in query.iter_mut() {
    if !actor.path.is_empty() {
      let destination = actor.path[0];
      actor.direction = if position.x < destination.x {
        if position.y < destination.y {
          CharacterDirection::BottomRight
        } else if position.y > destination.y {
          CharacterDirection::Bottom
        } else {
          CharacterDirection::Right
        }
      } else if position.x > destination.x {
        if position.y < destination.y {
          CharacterDirection::Top
        } else if position.y > destination.y {
          CharacterDirection::TopLeft
        } else {
          CharacterDirection::Left
        }
      } else if position.y < destination.y {
        CharacterDirection::TopRight
      } else {
        CharacterDirection::BottomLeft
      };
    }
  }
}

pub fn texture_system(mut query: Query<(&mut Character, &mut TileTextureIndex)>) {
  for (actor, mut texture_index) in query.iter_mut() {
    texture_index
      .set(Box::new(actor.get_texture_index()))
      .unwrap()
  }
}
