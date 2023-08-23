use map::TileKind as TK;

fn main() {
  let map = map::Map::new_with_tile_kinds(vec![
    vec![TK::Water, TK::Water, TK::Water, TK::Water, TK::Water],
    vec![TK::Water, TK::Firm, TK::House, TK::Firm, TK::Water],
    vec![TK::Water, TK::House, TK::House, TK::House, TK::Water],
    vec![TK::Water, TK::Firm, TK::Grass, TK::Firm, TK::Water],
    vec![TK::Water, TK::House, TK::Forest, TK::House, TK::Water],
    vec![TK::Water, TK::Forest, TK::Mine, TK::Forest, TK::Water],
    vec![TK::Water, TK::Water, TK::Water, TK::Water, TK::Water],
  ]);

  map.print();
}
