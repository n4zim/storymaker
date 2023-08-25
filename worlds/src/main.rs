use serde::{Deserialize, Serialize};
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

#[derive(Serialize)]
struct WorldConfig {
  size_x: u32,
  size_y: u32,
  grid_x: f32,
  grid_y: f32,
  tile_sets: HashMap<String, WorldConfigTileSet>,
  layers: Vec<WorldConfigLayer>,
}

#[derive(Serialize)]
struct WorldConfigTileSet {
  source: String,
  size_x: f32,
  size_y: f32,
}

#[derive(Serialize)]
struct WorldConfigLayer {
  name: String,
  tiles: Vec<Vec<u32>>,
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
      WorldConfigTileSet {
        source: Path::new(&terrain.image)
          .components()
          .skip(5)
          .map(|c| c.as_os_str().to_str().unwrap())
          .collect::<Vec<&str>>()
          .join("/"),
        size_x: terrain.tilewidth,
        size_y: terrain.tileheight,
      },
    );

    let mut layers = Vec::<WorldConfigLayer>::new();

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
      layers.push(WorldConfigLayer {
        name: layer.name.clone(),
        tiles,
      });
    }

    let output = WorldConfig {
      size_x: world.width,
      size_y: world.height,
      grid_x: world.tilewidth,
      grid_y: world.tileheight,
      tile_sets,
      layers,
    };

    let output = serde_json::to_string(&output).unwrap();

    std::fs::write(
      format!("../assets/worlds/{}.json", name),
      format!("{}\n", output),
    )
    .unwrap();
  }
}
