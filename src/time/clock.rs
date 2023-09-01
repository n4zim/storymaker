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

use bevy::prelude::*;

#[derive(Resource)]
pub struct GameClock {
  pub day: i32,
  pub hour: u32,
  pub minute: u32,
  pub second: u32,
  pub total: u32,
}

impl GameClock {
  pub fn tick(&mut self) {
    if self.second == 59 {
      self.second = 0;
      if self.minute == 59 {
        self.minute = 0;
        if self.hour == 23 {
          self.hour = 0;
          self.day += 1;
        } else {
          self.hour += 1;
        }
      } else {
        self.minute += 1;
      }
    } else {
      self.second += 1;
    }
    self.total += 1;
  }

  pub fn to_string(&self) -> String {
    format!(
      "Day {} - {:02}:{:02}:{:02}",
      self.day, self.hour, self.minute, self.second,
    )
  }
}

impl Default for GameClock {
  fn default() -> Self {
    GameClock {
      day: 1,
      hour: 0,
      minute: 0,
      second: 0,
      total: 0,
    }
  }
}
