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

use crate::{characters, markers};
use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;
use std::{collections::HashMap, fs::read_to_string};

#[derive(Resource)]
pub struct WorldMap {
  pub size: TilemapSize,
  pub grid: TilemapGridSize,
  terrain_texture: TilemapTexture,
  terrain_size: TilemapTileSize,
  layers: Vec<WorldMapLayer>,
}

struct WorldMapLayer {
  name: String,
  tiles: Vec<Vec<Option<WorldMapTile>>>,
}

impl WorldMap {
  pub fn new(asset_server: &Res<AssetServer>, name: &str) -> WorldMap {
    let world = serde_json::from_str::<WorldConfig>(
      &read_to_string(format!("assets/worlds/{}.json", name)).unwrap(),
    )
    .unwrap();

    let mut tiles = Vec::<WorldMapLayer>::new();

    for layer in world.layers.iter() {
      let mut grid = Vec::new();
      for row in layer.tiles.iter() {
        let mut row_tiles = Vec::new();
        for tile in row {
          row_tiles.push(WorldMapTile::from_index(*tile));
        }
        grid.push(row_tiles);
      }
      tiles.push(WorldMapLayer {
        name: layer.name.clone(),
        tiles: grid,
      });
    }

    let terrain = world.tile_sets.get("terrain").unwrap();

    WorldMap {
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
      layers: tiles,
    }
  }

  pub fn render(
    &self,
    commands: &mut Commands,
    spawner: &mut characters::Spawner,
  ) {
    let map_type = TilemapType::Isometric(IsoCoordSystem::Diamond);
    for (layer_index, tiles) in self.layers.iter().enumerate() {
      //println!("Rendering layer {}", tiles.name);

      let mut storage = TileStorage::empty(self.size);
      let layer_entity = commands.spawn_empty().id();

      for (x, row) in tiles.tiles.iter().enumerate() {
        for (y, tile) in row.iter().enumerate() {
          if let Some(tile) = tile {
            let position = TilePos {
              x: x as u32,
              y: y as u32,
            };
            let mut entity = commands.spawn(TileBundle {
              position,
              tilemap_id: TilemapId(layer_entity),
              texture_index: tile.clone().to_texture_index(),
              ..Default::default()
            });
            if tile == &WorldMapTile::Water {
              entity.insert(markers::Water);
            }
            storage.set(&position, entity.id());
            if tile == &WorldMapTile::House {
              spawner.insert_with_random_gender(commands, position);
            }
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

  pub fn is_walkable(&self, position: &TilePos) -> bool {
    let mut walkable = true;
    for layer in self.layers.iter() {
      if layer.name != "terrain" && layer.name != "nature" {
        continue;
      }
      walkable = if let Some(tile) = layer.tiles.get(position.x as usize) {
        if let Some(tile) = tile.get(position.y as usize) {
          if layer.name == "terrain" {
            if let Some(tile) = tile {
              tile.clone().is_terrain_walkable()
            } else {
              false
            }
          } else {
            tile.is_none()
          }
        } else {
          false
        }
      } else {
        false
      };
      if !walkable {
        break;
      }
    }
    walkable
  }
}

// -----------------------------------------------------------------------------

#[derive(PartialEq, Clone)]
pub enum WorldMapTile {
  Dirt = 0,
  Grass = 1,
  Rock = 2,
  Sand = 3,
  Water = 7,
  Stone = 9,
  WaterPlant = 10,
  Bush = 19,
  Tree = 20,
  Workplace = 32,
  House = 43,
  BridgeLeft = 96,
  BridgeRight = 97,
}

impl WorldMapTile {
  fn from_index(index: u32) -> Option<Self> {
    if index == 0 {
      return None;
    }
    let index = index - 1;
    match index {
      0 => Some(Self::Dirt),
      1 => Some(Self::Grass),
      2 => Some(Self::Rock),
      3 => Some(Self::Sand),
      7 => Some(Self::Water),
      9 => Some(Self::Stone),
      10 => Some(Self::WaterPlant),
      19 => Some(Self::Bush),
      20 => Some(Self::Tree),
      32 => Some(Self::Workplace),
      43 => Some(Self::House),
      96 => Some(Self::BridgeLeft),
      97 => Some(Self::BridgeRight),
      _ => panic!("Unknown world map tile index {}", index),
    }
  }
  fn to_texture_index(self) -> TileTextureIndex {
    TileTextureIndex(self as u32)
  }
  fn is_terrain_walkable(self) -> bool {
    self == Self::Grass
      || self == Self::Sand
      || self == Self::BridgeLeft
      || self == Self::BridgeRight
  }
}

// -----------------------------------------------------------------------------

#[derive(serde::Deserialize)]
struct WorldConfig {
  size_x: u32,
  size_y: u32,
  grid_x: f32,
  grid_y: f32,
  tile_sets: HashMap<String, WorldConfigTileSet>,
  layers: Vec<WorldConfigLayer>,
}

#[derive(serde::Deserialize)]
struct WorldConfigTileSet {
  source: String,
  size_x: f32,
  size_y: f32,
}

#[derive(serde::Deserialize)]
struct WorldConfigLayer {
  name: String,
  tiles: Vec<Vec<u32>>,
}
