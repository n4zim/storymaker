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

pub type Position = (usize, usize);

#[derive(Debug, PartialEq)]
pub enum TerrainKind {
  Water,
  Sand,
  Grass,
  //Forest,
}

#[derive(Debug, PartialEq)]
pub enum BuildingKind {
  None,
  House,
  Workplace,
  //Mine,
}

pub struct Map {
  pub terrain: Vec<Vec<TerrainKind>>,
  pub buildings: Vec<Vec<BuildingKind>>,
}

impl Map {
  pub fn new(terrain: Vec<Vec<TerrainKind>>, buildings: Vec<Vec<BuildingKind>>) -> Map {
    Map { terrain, buildings }
  }

  /*pub fn new_with_tile_kinds(tile_kinds: Vec<Vec<TerrainKind>>) -> Map {
    let mut tiles = Vec::new();
    for column in tile_kinds {
      let mut row = Vec::new();
      for tile_kind in column {
        row.push(Terrain::new(tile_kind));
      }
      tiles.push(row);
    }
    Map { tiles }
  }*/

  #[allow(dead_code)]
  pub fn get_terrain_kind(&self, x: usize, y: usize) -> &TerrainKind {
    &self.terrain[x][y]
  }

  fn distance(from: Position, to: Position) -> i32 {
    let x = (from.0 as i32 - to.0 as i32).abs();
    let y = (from.1 as i32 - to.1 as i32).abs();
    x + y
  }

  pub fn print(&self) {
    for (y, column) in self.buildings.iter().enumerate() {
      for (x, building) in column.iter().enumerate() {
        if building == &BuildingKind::None {
          continue;
        }
        println!("# {:?}({}, {})", building, x, y);
        for (y2, column2) in self.buildings.iter().enumerate() {
          for (x2, building2) in column2.iter().enumerate() {
            if building2 == &BuildingKind::None {
              continue;
            }
            if x == x2 && y == y2 {
              continue;
            }
            let distance = Map::distance((x, y), (x2, y2));
            println!(
              "  - {:?}({}, {}) is {} unit(s) away",
              building2, x2, y2, distance,
            );
          }
        }
        println!();
      }
    }
  }
}
