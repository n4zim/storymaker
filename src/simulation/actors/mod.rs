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

use bevy::prelude::*;
use bevy_ecs_tilemap::tiles::{TilePos, TileTextureIndex};

pub mod movement;
pub mod spawner;

#[derive(Component)]
pub struct Actor {
  gender: ActorGender,
  pub direction: Direction,
  pub destination: Option<TilePos>,
}

impl Actor {
  pub fn new(gender: ActorGender) -> Actor {
    Actor {
      gender,
      direction: Direction::Bottom,
      destination: None,
    }
  }

  pub fn get_texture_index(&self) -> TileTextureIndex {
    TileTextureIndex(8 + self.gender as u32 * 32 + self.direction as u32 + 8)
  }
}

#[derive(Clone, Copy)]
pub enum ActorGender {
  Male = 2,
  Female = 1,
  Other = 0,
}

impl ActorGender {
  pub fn index_to_gender(index: i32) -> Option<ActorGender> {
    match index {
      2 => Some(ActorGender::Male),
      1 => Some(ActorGender::Female),
      0 => Some(ActorGender::Other),
      _ => None,
    }
  }
}

#[derive(Clone, Copy)]
pub enum Direction {
  Top = 0,
  TopRight = 1,
  Right = 2,
  BottomRight = 3,
  Bottom = 4,
  BottomLeft = 5,
  Left = 6,
  TopLeft = 7,
}
