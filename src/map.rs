
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
  tiles: Vec<Vec<Tile>>,
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

  #[allow(dead_code)]
  pub fn get_tile(&self, x: usize, y: usize) -> &Tile {
    &self.tiles[x][y]
  }

  fn distance(from: (usize, usize), to: (usize, usize)) -> i32 {
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
