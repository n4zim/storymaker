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

use crate::{actors::Actor, time::Time, map::{Map, BuildingKind}};

pub struct World {
  time: Time,
  #[allow(dead_code)]
  map: Map,
  pub actors: Vec<Actor>,
}

impl World {
  pub fn new(map: Map) -> World {
    let time = Time::new();
    let mut actors = Vec::new();
    let mut index = 1;
    for (y, tiles) in map.buildings.iter().enumerate() {
      for (x, building) in tiles.iter().enumerate() {
        if building == &BuildingKind::House {
          actors.push(Actor::new(
            &time,
            format!("Actor {}", index),
            (x, y),
          ));
          index += 1;
        }
      }
    }
    World { time, map, actors }
  }

  pub fn tick(&mut self) {
    if self.time.minute == 0 && self.time.second == 0 {
      println!("[[ DAY {} - HOUR {} ]]", self.time.day, self.time.hour);
    }

    for actor in self.actors.iter_mut() {
      actor.tick(&self.time);
    }

    self.time.next();
  }
}
