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

use crate::{actors::Actor, time::Time};

pub trait Action {
  #[allow(unused_variables)]
  fn tick(&self, actor: &Actor, time: &Time) -> Option<(Box<dyn Action>, Box<dyn Action>)> {
    None
  }
}

pub struct Idle;

impl Action for Idle {}

pub struct Eat;

impl Action for Eat {
  fn tick(&self, actor: &Actor, _time: &Time) -> Option<(Box<dyn Action>, Box<dyn Action>)> {
    println!("{} is eating", actor.name);
    Some((Box::new(Eat), Box::new(Idle)))
  }
}


pub struct Sleep {
  time: Time,
  duration: u32,
}

impl Sleep {
  pub fn new(time: &Time, duration: u32) -> Sleep {
    Sleep {
      time: time.clone(),
      duration,
    }
  }
}

impl Action for Sleep {
  fn tick(&self, actor: &Actor, time: &Time) -> Option<(Box<dyn Action>, Box<dyn Action>)> {
    let elapsed = time.elapsed(&self.time);
    if elapsed == 0 {
      println!("{} starts to sleep at {}", actor.name, time.to_string());
    } else if elapsed >= self.duration {
      println!("{} wakes up", actor.name);
      return Some((
        Box::new(Sleep{ time: time.clone(), duration: self.duration }),
        Box::new(Idle)
      ));
    }
    None
  }
}
