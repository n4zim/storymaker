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
  tilewidth: u32,
  tileheight: u32,
  layers: Vec<TiledLayer>,
}

#[derive(Deserialize)]
struct TiledLayer {
  name: String,
  data: Vec<u32>,
}

#[derive(Deserialize)]
struct TiledTileSet {
  tilewidth: u32,
  tileheight: u32,
  image: String,
}

#[derive(Serialize)]
struct World {
  size_x: u32,
  size_y: u32,
  grid_x: u32,
  grid_y: u32,
  tile_sets: HashMap<String, TileSet>,
  layers: HashMap<String, Vec<Vec<u32>>>,
}

#[derive(Serialize)]
struct TileSet {
  source: String,
  size_x: u32,
  size_y: u32,
}

fn main() {
  for world in read_dir("tiled").unwrap() {
    let map = world.unwrap();
    let name = map.file_name().into_string().unwrap();
    println!("Building {}...", name);

    let world =
      &read_to_string(map.path().join("world.json").to_str().unwrap()).unwrap();

    let world = serde_json::from_str::<TiledMap>(world).unwrap();

    let mut tile_sets = HashMap::<String, TileSet>::new();

    let terrain =
      &read_to_string(map.path().join("tiles/terrain.json").to_str().unwrap())
        .unwrap();

    let terrain = serde_json::from_str::<TiledTileSet>(terrain).unwrap();

    tile_sets.insert(
      "terrain".to_string(),
      TileSet {
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

    let mut layers = HashMap::<String, Vec<Vec<u32>>>::new();

    for layer in &world.layers {
      let mut columns = Vec::<Vec<u32>>::new();
      for x in 0..world.width {
        let mut row = Vec::<u32>::new();
        for y in 0..world.height {
          let tile =
            layer.data[(x + (world.height - y - 1) * world.width) as usize];
          row.push(tile);
        }
        columns.push(row);
      }
      layers.insert(layer.name.clone(), columns);
    }

    let output = World {
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
