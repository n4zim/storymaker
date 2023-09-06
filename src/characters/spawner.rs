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
use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;
use bevy_ecs_tilemap::tiles::TilePos;
use bevy_turborand::GlobalRng;
use rand::Rng;
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

  pub fn insert_with_random_gender(
    &mut self,
    commands: &mut Commands,
    position: TilePos,
    rng: &mut GlobalRng,
  ) {
    let index = rand::thread_rng().gen_range(0..=2);
    if let Some(gender) = CharacterGender::new_with_index(index) {
      self.insert(commands, position, gender, rng);
    }
  }

  fn insert(
    &mut self,
    commands: &mut Commands,
    position: TilePos,
    gender: CharacterGender,
    rng: &mut GlobalRng,
  ) {
    let names = self.firstnames.get(&gender).unwrap();
    let firstname = names
      .get(rng.gen_range(0..names.len()))
      .unwrap()
      .to_string();

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
