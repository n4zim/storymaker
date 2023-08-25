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

use serde::Deserialize;
use std::{
  collections::HashMap,
  fs::{read_dir, read_to_string},
  path::Path,
};

#[derive(Deserialize)]
struct TiledMap {
  width: u32,
  height: u32,
  tilewidth: f32,
  tileheight: f32,
  layers: Vec<TiledLayer>,
}

#[derive(Deserialize)]
struct TiledLayer {
  name: String,
  data: Vec<u32>,
}

#[derive(Deserialize)]
struct TiledTileSet {
  tilewidth: f32,
  tileheight: f32,
  image: String,
}

fn main() {
  for world in read_dir("tiled").unwrap() {
    let map = world.unwrap();
    let name = map.file_name().into_string().unwrap();
    println!("Building {}...", name);

    let world =
      &read_to_string(map.path().join("world.json").to_str().unwrap()).unwrap();

    let world = serde_json::from_str::<TiledMap>(world).unwrap();

    let mut tile_sets = HashMap::<String, WorldConfigTileSet>::new();

    let terrain =
      &read_to_string(map.path().join("tiles/terrain.json").to_str().unwrap())
        .unwrap();

    let terrain = serde_json::from_str::<TiledTileSet>(terrain).unwrap();

    tile_sets.insert(
      "terrain".to_string(),
      TileSet {
        source: Path::new(&terrain.image)
          .components()
          .skip(4)
          .map(|c| c.as_os_str().to_str().unwrap())
          .collect::<Vec<&str>>()
          .join("/"),
        size_x: terrain.tilewidth,
        size_y: terrain.tileheight,
      },
    );

    let mut layers = Vec::<Layer>::new();

    for layer in &world.layers {
      let mut tiles = Vec::<Vec<u32>>::new();
      for x in 0..world.width {
        let mut row = Vec::<u32>::new();
        for y in 0..world.height {
          let tile =
            layer.data[(x + (world.height - y - 1) * world.width) as usize];
          row.push(tile);
        }
        tiles.push(row);
      }
      layers.push(Layer {
        name: layer.name.clone(),
        tiles,
      });
    }

    let output = World {
      size_x: world.width,
      size_y: world.height,
      grid_x: world.tilewidth,
      grid_y: world.tileheight,
      tile_sets,
      layers,
    };

    let output: String = serde_json::to_string(&output).unwrap();

    std::fs::write(
      format!("assets/worlds/{}.json", name),
      format!("{}\n", output),
    )
    .unwrap();
  }
}
