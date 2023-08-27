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

const SECONDS_PER_MINUTE: u32 = 60;
const MINUTES_PER_HOUR: u32 = 60;
const HOURS_PER_DAY: u32 = 24;

const SECONDS_PER_HOUR: u32 = MINUTES_PER_HOUR * SECONDS_PER_MINUTE;
const SECONDS_PER_DAY: u32 = HOURS_PER_DAY * SECONDS_PER_HOUR;

pub struct GamePlugin;

impl Plugin for GamePlugin {
  fn build(&self, app: &mut App) {
    app
      .insert_resource(Time::new())
      .add_state::<SpeedState>()
      .add_systems(Update, clock)
      .add_systems(Update, speed_commands);
  }
}

fn clock(state: Res<State<SpeedState>>, mut time: ResMut<Time>) {
  if !state.is_paused() {
    time.tick();
  }
  println!("{}", time.to_string());
}

fn speed_commands(
  state: Res<State<SpeedState>>,
  mut next_state: ResMut<NextState<SpeedState>>,
  keyboard_input: Res<Input<KeyCode>>,
) {
  if keyboard_input.just_pressed(KeyCode::Space) {
    let current = state.get();
    next_state.set(match current {
      SpeedState::PausedOneMinute => SpeedState::OneMinute,
      SpeedState::PausedOneHour => SpeedState::OneHour,
      SpeedState::PausedOneDay => SpeedState::OneDay,
      SpeedState::OneMinute => SpeedState::PausedOneMinute,
      SpeedState::OneHour => SpeedState::PausedOneHour,
      SpeedState::OneDay => SpeedState::PausedOneDay,
    });
  }

  if keyboard_input.just_pressed(KeyCode::Key1) {
    next_state.set(SpeedState::OneMinute);
  }

  if keyboard_input.just_pressed(KeyCode::Key2) {
    next_state.set(SpeedState::OneHour);
  }

  if keyboard_input.just_pressed(KeyCode::Key3) {
    next_state.set(SpeedState::OneDay);
  }
}

#[derive(Resource)]
pub struct Time {
  timer: Timer,
  day: i32,
  hour: u32,
  minute: u32,
  second: u32,
}

impl Time {
  fn new() -> Time {
    Time {
      timer: Timer::from_seconds(1.0 / 60.0, TimerMode::Repeating),
      day: 1,
      hour: 0,
      minute: 0,
      second: 0,
    }
  }

  fn tick(&mut self) {
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

  fn to_string(&self) -> String {
    format!(
      "Day {} - {:02}:{:02}:{:02}",
      self.day, self.hour, self.minute, self.second,
    )
  }
}

#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
enum SpeedState {
  #[default]
  OneMinute,
  PausedOneMinute,
  OneHour,
  PausedOneHour,
  OneDay,
  PausedOneDay,
}

impl SpeedState {
  fn is_paused(&self) -> bool {
    match self {
      SpeedState::PausedOneMinute => true,
      SpeedState::PausedOneHour => true,
      SpeedState::PausedOneDay => true,
      _ => false,
    }
  }
}
