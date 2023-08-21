
pub enum TileKind {
  House,
  Workplace,
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
  tiles: Vec<Vec<Tile>>,
}

impl Map {
  pub fn new(tiles: Vec<Vec<Tile>>) -> Map {
    Map { tiles }
  }
}
