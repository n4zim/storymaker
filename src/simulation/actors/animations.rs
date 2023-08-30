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

use super::{Actor, ActorDirection};
use bevy::prelude::*;
use bevy_ecs_tilemap::tiles::{TilePos, TileTextureIndex};

pub fn directions_system(mut query: Query<(&mut Actor, &mut TilePos)>) {
  for (mut actor, position) in query.iter_mut() {
    if !actor.path.is_empty() {
      let destination = actor.path[0];
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

pub fn texture_system(mut query: Query<(&mut Actor, &mut TileTextureIndex)>) {
  for (actor, mut texture_index) in query.iter_mut() {
    texture_index
      .set(Box::new(actor.get_texture_index()))
      .unwrap()
  }
}
