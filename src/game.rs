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

pub struct GamePlugin;

impl Plugin for GamePlugin {
  fn build(&self, app: &mut App) {
    app.add_systems(Update, clock);
  }
}

#[derive(Default, Debug)]
enum ClockSpeed {
  Paused,
  #[default]
  OneMinute,
  OneHour,
  OneDay,
}

#[derive(Resource, Default)]
struct ClockState {
  speed: ClockSpeed,
  previous_speed: ClockSpeed,
}

fn clock(state: Res<ClockState>, time: Res<Time>) {
  println!("{:?}", state.speed);
}

fn speed_commands(
  mut state: ResMut<ClockState>,
  keyboard_input: Res<Input<KeyCode>>,
) {
  if keyboard_input.just_pressed(KeyCode::Space) {
    state.speed = match state.speed {
      ClockSpeed::Paused => state.previous_speed,
      _ => ClockSpeed::Paused,
    }
  }
}
