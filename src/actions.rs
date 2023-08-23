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

pub trait Action: Clone {
  fn tick(self, _actor: Actor) -> Box<Self> {
    Box::new(Idle)
  }
}

impl Clone for Box<dyn Action> {
  fn clone(&self) -> Box<dyn Action> {
    self.clone_box()
  }
}

#[derive(Clone)]
pub struct Idle;

impl Action for Idle {}

#[derive(Clone)]
pub struct Eat;

impl Action for Eat {
  fn tick(self, actor: Actor) -> Box<Self> {
    println!("{} is eating", actor.name);
    Box::new(Idle)
  }
}

#[derive(Clone)]
pub struct Sleep {
  duration: u64,
  started: bool,
}

impl Action for Sleep {
  fn tick(self, actor: Actor) -> Box<Self> {
    if !self.started {
      println!("{} is sleeping", actor.name);
      return Box::new(Sleep {
        duration: self.duration,
        started: true,
      });
    }

    if self.duration > 0 {
      return Box::new(Sleep {
        duration: self.duration - 1,
        started: true,
      });
    }

    println!("{} is waking up", actor.name);
    Box::new(Idle)
  }
}
