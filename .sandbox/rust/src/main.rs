use map::{Tile, TileKind};
use tokio::time::{interval, Duration};

mod actor;
mod map;
mod time;
mod world;

const INTERVAL: u64 = 1;

#[tokio::main]
async fn main() {
  let map = map::Map::new(vec![
    vec![Tile::new(TileKind::House), Tile::new(TileKind::House)],
    vec![Tile::new(TileKind::Workplace), Tile::new(TileKind::Workplace)],
  ]);

  let mut world = world::World::new(map);

  let mut interval = interval(Duration::from_millis(INTERVAL));

  loop {
    interval.tick().await;
    world.tick();
  }
}
