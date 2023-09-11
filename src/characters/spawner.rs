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

use super::component::*;
use crate::brain;
use crate::time::history::History;
use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;
use bevy_ecs_tilemap::tiles::TilePos;
use bevy_turborand::prelude::*;
use std::collections::HashMap;
use std::fs::read_to_string;

#[derive(Resource)]
pub struct CharactersSpawner {
  storage: TileStorage,
  tile_id: TilemapId,
  firstnames: HashMap<CharacterGender, Vec<String>>,
  lastnames: Vec<String>,
}

impl CharactersSpawner {
  pub fn new(
    size: TilemapSize,
    grid_size: TilemapGridSize,
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
  ) -> CharactersSpawner {
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

    let mut males = Vec::new();
    for name in read_to_string("assets/names/males.txt")
      .unwrap()
      .split("\n")
    {
      if name.len() == 0 {
        continue;
      }
      males.push(name.to_string());
    }

    let mut females = Vec::new();
    for name in read_to_string("assets/names/females.txt")
      .unwrap()
      .split("\n")
    {
      if name.len() == 0 {
        continue;
      }
      females.push(name.to_string());
    }

    let mut lastnames = Vec::new();
    for name in read_to_string("assets/names/family.txt")
      .unwrap()
      .split("\n")
    {
      if name.len() == 0 {
        continue;
      }
      lastnames.push(name.to_string());
    }

    let mut firstnames = HashMap::new();
    firstnames.insert(CharacterGender::Male, males);
    firstnames.insert(CharacterGender::Female, females);

    CharactersSpawner {
      storage,
      tile_id: TilemapId(entity_id),
      firstnames,
      lastnames,
    }
  }

  pub fn insert(
    &mut self,
    commands: &mut Commands,
    position: TilePos,
    global_rng: &mut GlobalRng,
  ) {
    let mut rng_component = RngComponent::from(global_rng);
    let rng = rng_component.get_mut();

    let gender = CharacterGender::new_with_index(rng.i32(0..2)).unwrap();

    let names = self
      .firstnames
      .get(match gender {
        CharacterGender::Other => {
          if rng.bool() {
            &CharacterGender::Male
          } else {
            &CharacterGender::Female
          }
        }
        _ => &gender,
      })
      .unwrap();
    let firstname = names.get(rng.usize(0..names.len())).unwrap().to_string();
    let lastname = self
      .lastnames
      .get(rng.usize(0..self.lastnames.len()))
      .unwrap()
      .to_string();

    let entity_id = commands.spawn_empty().id();

    let character = Character::new(entity_id, firstname, lastname, gender);
    let texture_index = character.get_texture_index();

    let mut entity = commands.entity(entity_id);

    entity.insert((
      character,
      TileBundle {
        position,
        tilemap_id: self.tile_id,
        texture_index,
        ..Default::default()
      },
      rng_component,
      History::new(),
    ));

    brain::insert_bundle(&mut entity);

    self.storage.set(&position, entity_id);
  }
}
