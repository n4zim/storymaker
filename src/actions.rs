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

use crate::actors::Actor;

pub trait Action {
  fn tick(self, actor: Actor) -> Self {
    Idle
  }
}

pub struct Idle;

impl Action for Idle {}

pub struct Eat;

impl Action for Eat {
  fn tick(self, actor: Actor) -> Self {
    println!("{} is eating", actor.name);
    Idle
  }
}

pub struct Sleep {
  duration: u64,
  current: u64,
}

impl Sleep {
  pub fn new(duration: u64) -> Self {
    Sleep {
      duration,
      current: 0,
    }
  }
}

impl Action for Sleep {
  fn tick(self, actor: Actor) -> Box<dyn Action> {
    if self.current == 0 {
      println!("{} is going to sleeping", actor.name);
      return Box::new(Sleep {
        duration: self.duration,
        current: 1,
      });
    }

    if self.duration > self.current {
      return Box::new(Sleep {
        duration: self.duration,
        current: self.current + 1,
      });
    }

    println!("{} is waking up", actor.name);
    Box::new(Idle)
  }
}
