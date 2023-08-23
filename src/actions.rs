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

use std::rc::Rc;
use crate::actors::Actor;

pub trait Action {
  fn tick(&mut self) -> Box<dyn Action> {
    Box::new(Idle)
  }
}

pub struct Idle;

impl Action for Idle {}

pub struct Eat {
  actor: Rc<Actor>,
}

impl Action for Eat {
  fn tick(&mut self) -> Box<dyn Action> {
    println!("{} is eating", self.actor.name);
    Box::new(Idle)
  }
}

pub struct Sleep {
  actor: Rc<Actor>,
  duration: u64,
  started: bool,
}

impl Action for Sleep {
  fn tick(&mut self) -> Box<dyn Action> {
    if !self.started {
      println!("{} is sleeping", self.actor.name);
      return Box::new(Sleep {
        actor: Rc::clone(&self.actor),
        duration: self.duration,
        started: true,
      });
    }

    if self.duration > 0 {
      return Box::new(Sleep {
        actor: Rc::clone(&self.actor),
        duration: self.duration - 1,
        started: true,
      });
    }

    println!("{} is waking up", self.actor.name);
    Box::new(Idle)
  }
}
