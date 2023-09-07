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

#[derive(Component)]
pub struct Character {
  firstname: String,
  lastname: String,
  gender: CharacterGender,
  direction: CharacterDirection,
  posture: CharacterPosture,
  posture_reverse: bool,
}

impl Character {
  pub fn new(
    firstname: String,
    lastname: String,
    gender: CharacterGender,
  ) -> Character {
    Character {
      firstname,
      lastname,
      gender,
      direction: CharacterDirection::Bottom,
      posture: CharacterPosture::Idle,
      posture_reverse: false,
    }
  }

  pub fn get_texture_index(&self) -> TileTextureIndex {
    TileTextureIndex(
      8 + self.gender.to_u32() * 32
        + self.direction.to_u32()
        + 8 * self.posture.to_u32(),
    )
  }

  pub fn set_next_posture(&mut self) {
    self.posture = match self.posture {
      CharacterPosture::LeftFoot => CharacterPosture::Idle,
      CharacterPosture::Idle => {
        self.posture_reverse = !self.posture_reverse;
        if self.posture_reverse {
          CharacterPosture::LeftFoot
        } else {
          CharacterPosture::RightFoot
        }
      }
      CharacterPosture::RightFoot => CharacterPosture::Idle,
    };
  }

  pub fn set_next_direction(&mut self, from: &TilePos, to: &TilePos) {
    self.direction = if from.x < to.x {
      if from.y < to.y {
        CharacterDirection::BottomRight
      } else if from.y > to.y {
        CharacterDirection::Bottom
      } else {
        CharacterDirection::Right
      }
    } else if from.x > to.x {
      if from.y < to.y {
        CharacterDirection::Top
      } else if from.y > to.y {
        CharacterDirection::TopLeft
      } else {
        CharacterDirection::Left
      }
    } else if from.y < to.y {
      CharacterDirection::TopRight
    } else {
      CharacterDirection::BottomLeft
    };
  }

  pub fn get_name(&self) -> String {
    format!("{} {}", self.firstname, self.lastname)
  }

  pub fn get_gender(&self) -> String {
    self.gender.to_string()
  }
}

#[derive(PartialEq, Eq, Hash, Debug)]
pub enum CharacterGender {
  Male,
  Female,
  Other,
}

impl CharacterGender {
  pub fn new_with_index(index: i32) -> Option<Self> {
    match index {
      2 => Some(Self::Male),
      1 => Some(Self::Female),
      0 => Some(Self::Other),
      _ => None,
    }
  }

  fn to_u32(&self) -> u32 {
    match self {
      Self::Male => 2,
      Self::Female => 1,
      Self::Other => 0,
    }
  }

  fn to_string(&self) -> String {
    match self {
      Self::Male => "male",
      Self::Female => "female",
      Self::Other => "other",
    }
    .to_string()
  }
}

pub enum CharacterDirection {
  Top,
  TopRight,
  Right,
  BottomRight,
  Bottom,
  BottomLeft,
  Left,
  TopLeft,
}

impl CharacterDirection {
  fn to_u32(&self) -> u32 {
    match self {
      CharacterDirection::Top => 0,
      CharacterDirection::TopRight => 1,
      CharacterDirection::Right => 2,
      CharacterDirection::BottomRight => 3,
      CharacterDirection::Bottom => 4,
      CharacterDirection::BottomLeft => 5,
      CharacterDirection::Left => 6,
      CharacterDirection::TopLeft => 7,
    }
  }
}

pub enum CharacterPosture {
  LeftFoot,
  Idle,
  RightFoot,
}

impl CharacterPosture {
  fn to_u32(&self) -> u32 {
    match self {
      CharacterPosture::LeftFoot => 0,
      CharacterPosture::Idle => 1,
      CharacterPosture::RightFoot => 2,
    }
  }
}
