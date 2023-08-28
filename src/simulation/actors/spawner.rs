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

use super::{Actor, ActorGender};
use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;
use rand::Rng;

#[derive(Resource)]
pub struct ActorsSpawner {
  storage: TileStorage,
  tile_id: TilemapId,
}

impl ActorsSpawner {
  pub fn new(
    size: TilemapSize,
    grid_size: TilemapGridSize,
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
  ) -> ActorsSpawner {
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
        &size, &grid_size, &map_type, 2.0,
      ),
      ..Default::default()
    });

    ActorsSpawner {
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
    if let Some(gender) = ActorGender::index_to_gender(index) {
      self.insert(commands, position, gender);
    }
  }

  pub fn insert(
    &mut self,
    commands: &mut Commands,
    position: TilePos,
    gender: ActorGender,
  ) {
    let actor = Actor::new(gender);
    let texture_index = actor.get_texture_index();
    self.storage.set(
      &position,
      commands
        .spawn((
          actor,
          TileBundle {
            position,
            tilemap_id: self.tile_id,
            texture_index,
            ..Default::default()
          },
        ))
        .id(),
    );
  }
}
