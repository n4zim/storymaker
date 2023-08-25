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
use bevy_ecs_tilemap::prelude::*;
use std::{collections::HashMap, fs::read_to_string};

pub struct MapPlugin;

impl Plugin for MapPlugin {
  fn build(&self, app: &mut App) {
    app
      .insert_resource(TilemapRenderSettings {
        render_chunk_size: UVec2::new(3, 1),
        y_sort: true,
      })
      .add_plugins(TilemapPlugin)
      .add_systems(Startup, init);
  }
}

const WORLD_NAME: &str = "island1";

fn init(mut commands: Commands, asset_server: Res<AssetServer>) {
  let world = World::new(asset_server, WORLD_NAME);
  world.render(&mut commands);
  //commands.insert_resource(world);
}

#[derive(Resource)]
pub struct World {
  size: TilemapSize,
  grid: TilemapGridSize,
  terrain_texture: TilemapTexture,
  terrain_size: TilemapTileSize,
  tiles: Vec<WorldTiles>,
}

impl World {
  fn new(asset_server: Res<AssetServer>, name: &str) -> World {
    let world = serde_json::from_str::<WorldSource>(
      &read_to_string(format!("assets/worlds/{}.json", name)).unwrap(),
    )
    .unwrap();

    let mut tiles = Vec::<WorldTiles>::new();

    for (name, layer) in world.layers.iter() {
      let mut grid = Vec::new();
      for row in layer {
        let mut row_tiles = Vec::new();
        for tile in row {
          row_tiles.push(if *tile == 0 { None } else { Some(*tile - 1) });
        }
        grid.push(row_tiles);
      }
      tiles.push(WorldTiles {
        name: name.clone(),
        grid,
      });
    }

    let terrain = world.tile_sets.get("terrain").unwrap();

    World {
      size: TilemapSize {
        x: world.size_x,
        y: world.size_y,
      },
      grid: TilemapGridSize {
        x: world.grid_x,
        y: world.grid_y,
      },
      terrain_size: TilemapTileSize {
        x: terrain.size_x,
        y: terrain.size_y,
      },
      terrain_texture: TilemapTexture::Single(
        asset_server.load(&terrain.source),
      ),
      tiles,
    }
  }

  fn render(&self, commands: &mut Commands) {
    let map_type = TilemapType::Isometric(IsoCoordSystem::Diamond);
    for (layer_index, tiles) in self.tiles.iter().enumerate() {
      let mut storage = TileStorage::empty(self.size);
      let layer_entity = commands.spawn_empty().id();

      for (x, row) in tiles.grid.iter().enumerate() {
        for (y, tile) in row.iter().enumerate() {
          if let Some(tile) = tile {
            let position = TilePos {
              x: x as u32,
              y: y as u32,
            };
            storage.set(
              &position,
              commands
                .spawn(TileBundle {
                  position,
                  tilemap_id: TilemapId(layer_entity),
                  texture_index: TileTextureIndex(*tile),
                  ..Default::default()
                })
                .id(),
            );
          }
        }
      }

      commands.entity(layer_entity).insert(TilemapBundle {
        size: self.size,
        grid_size: self.grid,
        tile_size: self.terrain_size,
        texture: self.terrain_texture.clone(),
        map_type,
        storage,
        transform: get_tilemap_center_transform(
          &self.size,
          &self.grid,
          &map_type,
          layer_index as f32,
        ),
        ..Default::default()
      });
    }
  }
}

type WorldTilesGrid = Vec<Vec<Option<u32>>>;

pub struct WorldTiles {
  name: String,
  grid: WorldTilesGrid,
}

#[derive(serde::Deserialize)]
struct WorldSource {
  size_x: u32,
  size_y: u32,
  grid_x: f32,
  grid_y: f32,
  tile_sets: HashMap<String, TileSetSource>,
  layers: HashMap<String, Vec<Vec<u32>>>,
}

#[derive(serde::Deserialize)]
struct TileSetSource {
  source: String,
  size_x: f32,
  size_y: f32,
}
