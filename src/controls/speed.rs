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

use crate::time::{speed::GameSpeed, timer::GameTimer};
use bevy::prelude::*;

pub fn speed_commands(
  mut timer: ResMut<GameTimer>,
  current_state: Res<State<GameSpeed>>,
  mut next_state: ResMut<NextState<GameSpeed>>,
  keyboard_input: Res<Input<KeyCode>>,
) {
  if keyboard_input.just_pressed(KeyCode::Space) {
    if timer.0.paused() {
      timer.0.unpause();
    } else {
      timer.0.pause();
    }
    return;
  }

  let current_state = *current_state.get();

  if keyboard_input.just_pressed(KeyCode::Key1) {
    if current_state != GameSpeed::OneMinute {
      next_state.set(GameSpeed::OneMinute);
      if timer.0.paused() {
        timer.0.unpause();
      }
    }
  } else if keyboard_input.just_pressed(KeyCode::Key2) {
    if current_state != GameSpeed::OneHour {
      next_state.set(GameSpeed::OneHour);
      if timer.0.paused() {
        timer.0.unpause();
      }
    }
  } else if keyboard_input.just_pressed(KeyCode::Key3) {
    if current_state != GameSpeed::OneDay {
      next_state.set(GameSpeed::OneDay);
      if timer.0.paused() {
        timer.0.unpause();
      }
    }
  }
}
