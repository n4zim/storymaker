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

use map::TileKind as TK;
use tokio::time::{interval, Duration};

//mod actions;
//mod actors;
mod map;
//mod time;
//mod world;

const INTERVAL: u64 = 1;

pub type Position = (usize, usize);

#[tokio::main]
async fn main() {
  let map = map::Map::new_with_gen();

  /*let map = map::Map::new_with_tile_kinds(vec![
    vec![ TK::Water, TK::Water,  TK::Water,  TK::Water,  TK::Water ],
    vec![ TK::Water, TK::Firm,   TK::House,  TK::Firm,   TK::Water ],
    vec![ TK::Water, TK::House,  TK::House,  TK::House,  TK::Water ],
    vec![ TK::Water, TK::Firm,   TK::Grass,  TK::Firm,   TK::Water ],
    vec![ TK::Water, TK::House,  TK::Forest, TK::House,  TK::Water ],
    vec![ TK::Water, TK::Forest, TK::Mine,   TK::Forest, TK::Water ],
    vec![ TK::Water, TK::Water,  TK::Water,  TK::Water,  TK::Water ],
  ]);

  //map.print();

  let mut world = world::World::new(map);

  for actor in world.actors.iter_mut() {
    actor.action.tick();
  }

  let mut interval = interval(Duration::from_millis(INTERVAL));

  loop {
    interval.tick().await;
    world.tick();
  }*/
}
