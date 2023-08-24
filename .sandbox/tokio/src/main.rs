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

use std::time::Duration;
use tokio::time::interval;

mod actions;
mod actors;
mod map;
mod tiled;
mod time;
mod world;

const INTERVAL: u64 = 1;

#[tokio::main]
async fn main() {
  let map = tiled::read_map("island1");

  map.print();

  let mut world = world::World::new(map);

  let mut interval = interval(Duration::from_millis(INTERVAL));

  loop {
    interval.tick().await;
    world.tick();
  }
}
