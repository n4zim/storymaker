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

pub const SECONDS_PER_MINUTE: u32 = 60;
pub const MINUTES_PER_HOUR: u32 = 60;
pub const HOURS_PER_DAY: u32 = 24;

pub const SECONDS_PER_HOUR: u32 = MINUTES_PER_HOUR * SECONDS_PER_MINUTE;
pub const SECONDS_PER_DAY: u32 = HOURS_PER_DAY * SECONDS_PER_HOUR;

#[derive(Clone)]
pub struct Time {
  pub day: i32,
  pub hour: u32,
  pub minute: u32,
  pub second: u32,
}

impl Time {
  pub fn new() -> Time {
    Time {
      day: 1,
      hour: 0,
      minute: 0,
      second: 0,
    }
  }

  pub fn next(&mut self) {
    self.second += 1;

    if self.second == SECONDS_PER_MINUTE {
      self.second = 0;
      self.minute += 1;
    }

    if self.minute == MINUTES_PER_HOUR {
      self.minute = 0;
      self.hour += 1;
    }

    if self.hour == HOURS_PER_DAY {
      self.hour = 0;
      self.day += 1;
    }
  }

  pub fn elapsed(&self, other: &Time) -> u32 {
    let mut elapsed = 0;
    elapsed += (self.day - other.day) as u32 * SECONDS_PER_DAY;
    elapsed += (self.hour - other.hour) * SECONDS_PER_HOUR;
    elapsed += (self.minute - other.minute) * SECONDS_PER_MINUTE;
    elapsed += self.second - other.second;
    elapsed
  }

  pub fn to_string(&self) -> String {
    format!(
      "Day {} - {:02}:{:02}:{:02}",
      self.day,
      self.hour,
      self.minute,
      self.second,
    )
  }
}
