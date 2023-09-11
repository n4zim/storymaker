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

use crate::time::event::GameTick;
use bevy::prelude::*;

#[derive(Component, Debug)]
pub struct Sociability {
  pub current: f32,
  pub speed: f32,
}

impl Sociability {
  pub fn new(current: f32, speed: f32) -> Self {
    Self { current, speed }
  }
}

pub fn state_system(
  mut events: EventReader<GameTick>,
  mut sociability: Query<&mut Sociability>,
) {
  for _ in events.iter() {
    for mut current in &mut sociability {
      current.current += current.speed;
      if current.current >= 100.0 {
        current.current = 100.0;
      }
    }
  }
}
