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

use crate::map::{Map, BuildingKind, TerrainKind};
use serde::Deserialize;
use serde_xml_rs::from_str;
use std::fs::read_to_string;

pub fn read_map(name: &str) -> Map {
  let path = format!("assets/{}/map/{}.tmx", name, name);

  let map = read_to_string(path).unwrap();
  let map = from_str::<TiledMap>(&map).unwrap();

  let mut terrain = Vec::<Vec<TerrainKind>>::new();
  let mut buildings = Vec::<Vec<BuildingKind>>::new();

  for layer in map.layers {
    for column in layer.data.value.split("\n").collect::<Vec<&str>>() {
      if layer.name == "terrain" {
        terrain.push(
          column
            .split(",")
            .filter(|tile| tile != &"")
            .map(|tile| {
              match tile.parse::<usize>().unwrap() {
                8 => crate::map::TerrainKind::Water,
                4 => crate::map::TerrainKind::Sand,
                _ => crate::map::TerrainKind::Grass,
              }
            })
            .collect::<Vec<TerrainKind>>(),
        );
      } else if layer.name == "buildings" {
        buildings.push(
          column
            .split(",")
            .filter(|tile| tile != &"")
            .map(|tile| {
              match tile.parse::<usize>().unwrap() {
                44 => crate::map::BuildingKind::House,
                33 => crate::map::BuildingKind::Workplace,
                _ => crate::map::BuildingKind::None,
              }
            })
            .collect::<Vec<BuildingKind>>(),
        );
      }
    }
  }

  Map::new(terrain, buildings)
}

#[derive(Debug, Deserialize)]
struct TiledMap {
  #[serde(rename = "layer")]
  layers: Vec<TiledLayer>,
}

#[derive(Debug, Deserialize)]
struct TiledLayer {
  name: String,
  data: TiledData,
}

#[derive(Debug, Deserialize)]
struct TiledData {
  #[serde(rename = "$value")]
  value: String,
}
