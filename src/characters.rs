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

use crate::brain;
use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;
use bevy_ecs_tilemap::tiles::{TilePos, TileTextureIndex};
use rand::Rng;

// -----------------------------------------------------------------------------

#[derive(Component)]
pub struct Character {
  gender: CharacterGender,
  direction: CharacterDirection,
  posture: CharacterPosture,
  posture_reverse: bool,
}

impl Character {
  fn new(gender: CharacterGender) -> Character {
    Character {
      gender,
      direction: CharacterDirection::Bottom,
      posture: CharacterPosture::Idle,
      posture_reverse: false,
    }
  }

  fn get_texture_index(&self) -> TileTextureIndex {
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
}

// -----------------------------------------------------------------------------

pub enum CharacterGender {
  Male,
  Female,
  Other,
}

impl CharacterGender {
  fn new_with_index(index: i32) -> Option<Self> {
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

#[derive(Resource)]
pub struct Spawner {
  storage: TileStorage,
  tile_id: TilemapId,
}

// -----------------------------------------------------------------------------

impl Spawner {
  pub fn new(
    size: TilemapSize,
    grid_size: TilemapGridSize,
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
  ) -> Spawner {
    let texture =
      TilemapTexture::Single(asset_server.load(
        "sprites/AlexDreamer/Small-8-Direction-Characters_by_AxulArt.png",
      ));

    let storage = TileStorage::empty(size);
    let entity_id = commands.spawn_empty().id();

    let map_type = TilemapType::Isometric(IsoCoordSystem::Diamond);

    commands.entity(entity_id).insert(TilemapBundle {
      size,
      grid_size,
      map_type,
      tile_size: TilemapTileSize { x: 16.0, y: 22.0 },
      storage: storage.clone(),
      texture: texture.clone(),
      transform: get_tilemap_center_transform(
        &size, &grid_size, &map_type, 5.0,
      ),
      ..Default::default()
    });

    Spawner {
      storage,
      tile_id: TilemapId(entity_id),
    }
  }

  pub fn insert_with_random_gender(
    &mut self,
    commands: &mut Commands,
    position: TilePos,
  ) {
    let index = rand::thread_rng().gen_range(0..=2);
    if let Some(gender) = CharacterGender::new_with_index(index) {
      self.insert(commands, position, gender);
    }
  }

  fn insert(
    &mut self,
    commands: &mut Commands,
    position: TilePos,
    gender: CharacterGender,
  ) {
    let character = Character::new(gender);

    let texture_index = character.get_texture_index();

    let mut entity = commands.spawn((
      character,
      TileBundle {
        position,
        tilemap_id: self.tile_id,
        texture_index,
        ..Default::default()
      },
    ));

    brain::insert_bundle(&mut entity);

    self.storage.set(&position, entity.id());
  }
}
