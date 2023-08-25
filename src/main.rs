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

use std::fs;

use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;

mod camera;

fn startup(mut commands: Commands, asset_server: Res<AssetServer>) {
  commands.spawn(Camera2dBundle::default());

  let map: serde_json::Value = serde_json::from_str(
    &fs::read_to_string("assets/island1/map/world.json").unwrap(),
  )
  .unwrap();

  let map_size = TilemapSize {
    x: map.get("width").unwrap().as_u64().unwrap() as u32,
    y: map.get("height").unwrap().as_u64().unwrap() as u32,
  };

  let grid_size = TilemapGridSize {
    x: map.get("tilewidth").unwrap().as_u64().unwrap() as f32,
    y: map.get("tileheight").unwrap().as_u64().unwrap() as f32,
  };

  let terrain: serde_json::Value = serde_json::from_str(
    &fs::read_to_string("assets/island1/map/tiles/terrain1.json").unwrap(),
  )
  .unwrap();

  let tile_size = TilemapTileSize {
    x: terrain.get("tilewidth").unwrap().as_u64().unwrap() as f32,
    y: terrain.get("tileheight").unwrap().as_u64().unwrap() as f32,
  };

  let texture: Handle<Image> = asset_server
    .load("island1/sprites/GustavoVituri/Isometric_MedievalFantasy_Tiles.png");
  let texture = TilemapTexture::Single(texture);

  let map_type = TilemapType::Isometric(IsoCoordSystem::Diamond);

  for (layer_index, layer) in map
    .get("layers")
    .unwrap()
    .as_array()
    .unwrap()
    .iter()
    .enumerate()
  {
    let tiles = layer.get("data").unwrap().as_array().unwrap();

    let mut storage = TileStorage::empty(map_size);
    let layer_entity = commands.spawn_empty().id();

    for x in 0..map_size.x {
      for y in 0..map_size.y {
        let tile_id = tiles[(x + (map_size.y - y - 1) * map_size.x) as usize]
          .as_u64()
          .unwrap() as u32;
        if tile_id == 0 {
          continue;
        }
        let tile_pos = TilePos { x, y };
        storage.set(
          &tile_pos,
          commands
            .spawn(TileBundle {
              position: tile_pos,
              tilemap_id: TilemapId(layer_entity),
              texture_index: TileTextureIndex(tile_id - 1),
              ..Default::default()
            })
            .id(),
        );
      }
    }

    commands.entity(layer_entity).insert(TilemapBundle {
      size: map_size,
      grid_size,
      tile_size,
      texture: texture.clone(),
      map_type,
      storage,
      transform: get_tilemap_center_transform(
        &map_size,
        &grid_size,
        &map_type,
        layer_index as f32,
      ),
      ..Default::default()
    });
  }
}

fn main() {
  App::new()
    .insert_resource(TilemapRenderSettings {
      render_chunk_size: UVec2::new(3, 1),
      y_sort: true,
    })
    .add_plugins(
      DefaultPlugins
        .set(WindowPlugin {
          primary_window: Some(Window {
            title: String::from("StoryMaker"),
            ..Default::default()
          }),
          ..default()
        })
        .set(ImagePlugin::default_nearest()),
    )
    .add_plugins(TilemapPlugin)
    .add_systems(Startup, startup)
    .add_systems(Update, camera::movement)
    .add_systems(Update, camera::zoom)
    .add_systems(Update, camera::right_click_movement)
    .run();
}
