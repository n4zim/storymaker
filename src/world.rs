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

use crate::{actor::Actor, time::Time, map::Map};

pub struct World {
  time: Time,
  #[allow(dead_code)]
  map: Map,
  #[allow(dead_code)]
  actors: Vec<Actor>,
}

impl World {
  pub fn new(map: Map) -> World {
    World {
      time: Time::new(),
      map,
      actors: Vec::new(),
    }
  }

  pub fn tick(&mut self) {
    if self.time.minute == 0 && self.time.second == 0 {
      println!("[[ DAY {} - HOUR {} ]]", self.time.day, self.time.hour);
    }
    self.time.next();
  }
}
