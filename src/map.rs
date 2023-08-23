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

use image::{Rgb, ImageBuffer, RgbImage};
use world_map_gen::RandomBoardGen;

use crate::Position;

#[derive(Debug, PartialEq)]
pub enum TileKind {
  Grass,
  House,
  Firm,
  Mine,
  Forest,
  Water,
}

pub struct Tile {
  pub kind: TileKind,
}

impl Tile {
  pub fn new(kind: TileKind) -> Tile {
    Tile { kind }
  }
}

pub struct Map {
  pub tiles: Vec<Vec<Tile>>,
}

impl Map {
  #[allow(dead_code)]
  pub fn new(tiles: Vec<Vec<Tile>>) -> Map {
    Map { tiles }
  }

  pub fn new_with_tile_kinds(tile_kinds: Vec<Vec<TileKind>>) -> Map {
    let mut tiles = Vec::new();
    for column in tile_kinds {
      let mut row = Vec::new();
      for tile_kind in column {
        row.push(Tile::new(tile_kind));
      }
      tiles.push(row);
    }
    Map { tiles }
  }

  pub fn new_with_gen() -> Map {
    let mut generator = RandomBoardGen::default();
    let board = generator.gen_large(1000, 1000);

    let mut image = RgbImage::new(board.width() as u32, board.height() as u32);

    for (y, column) in board.rows().enumerate() {
      for (x, row) in column.iter().enumerate() {
        image.put_pixel(
          x as u32,
          y as u32,
          Rgb(
            match row.kind {
              world_map_gen::LandKind::Sea => [95, 215, 255],
              world_map_gen::LandKind::Mountain => [135, 95, 0],
              world_map_gen::LandKind::Forest => [0, 95, 0],
              world_map_gen::LandKind::Plain => [135, 255, 0],
              world_map_gen::LandKind::Town => [255, 255, 0],
              world_map_gen::LandKind::Top => [135, 135, 135],
              world_map_gen::LandKind::Highland => [95, 95, 0],
              world_map_gen::LandKind::DeepSea => [95, 95, 255],
              world_map_gen::LandKind::Path => [215, 255, 175],
            },
          ),
        );
      }
    }

    image.save("map.png").unwrap();

    Map { tiles: Vec::new() }
  }

  #[allow(dead_code)]
  pub fn get_tile(&self, x: usize, y: usize) -> &Tile {
    &self.tiles[x][y]
  }

  fn distance(from: Position, to: Position) -> i32 {
    let x = (from.0 as i32 - to.0 as i32).abs();
    let y = (from.1 as i32 - to.1 as i32).abs();
    x + y
  }

  #[allow(dead_code)]
  pub fn print(&self) {
    for (y, column) in self.tiles.iter().enumerate() {
      for (x, row) in column.iter().enumerate() {
        if row.kind == TileKind::Grass {
          continue;
        }
        println!("# {:?}({}, {})", row.kind, x, y);
        for (y2, column2) in self.tiles.iter().enumerate() {
          for (x2, row2) in column2.iter().enumerate() {
            if row2.kind == TileKind::Grass {
              continue;
            }
            if x == x2 && y == y2 {
              continue;
            }
            let distance = Map::distance((x, y), (x2, y2));
            println!(
              "  - {:?}({}, {}) is {} unit(s) away",
              row2.kind, x2, y2, distance,
            );
          }
        }
        println!();
      }
    }
  }
}
