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

use crate::game::GameTick;
use bevy::prelude::*;

#[derive(Component, Debug)]
pub struct Thirst {
  pub speed: f32,
  pub thirst: f32,
}

impl Thirst {
  pub fn new(thirst: f32, speed: f32) -> Self {
    Self { thirst, speed }
  }
}

pub fn system(
  mut events: EventReader<GameTick>,
  mut thirsts: Query<&mut Thirst>,
) {
  for _ in events.iter() {
    for mut thirst in &mut thirsts {
      thirst.thirst += thirst.speed;
      if thirst.thirst >= 100.0 {
        thirst.thirst = 100.0;
      }
      trace!("Thirst: {}", thirst.thirst);
    }
  }
}
