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

use std::time::Duration;

use bevy::prelude::*;

const SECONDS_PER_MINUTE: u32 = 60;
const MINUTES_PER_HOUR: u32 = 60;
const HOURS_PER_DAY: u32 = 24;

//const SECONDS_PER_HOUR: u32 = MINUTES_PER_HOUR * SECONDS_PER_MINUTE;
//const SECONDS_PER_DAY: u32 = HOURS_PER_DAY * SECONDS_PER_HOUR;

pub struct GamePlugin;

impl Plugin for GamePlugin {
  fn build(&self, app: &mut App) {
    app
      .insert_resource(GameTime::new())
      .add_systems(Update, clock)
      .add_systems(Update, speed_commands);
  }
}

fn clock(mut game_time: ResMut<GameTime>, time: Res<Time>) {
  game_time.timer.tick(time.delta());
  if game_time.timer.finished() {
    if !game_time.timer.paused() {
      game_time.tick();
    }
    println!("{}", game_time.to_string());
  }
}

fn speed_commands(
  mut time: ResMut<GameTime>,
  keyboard_input: Res<Input<KeyCode>>,
) {
  if keyboard_input.just_pressed(KeyCode::Space) {
    if time.timer.paused() {
      time.timer.unpause();
    } else {
      time.timer.pause();
    }
  }

  if keyboard_input.just_pressed(KeyCode::Key1) {
    time.set_to_one_minute();
  }

  if keyboard_input.just_pressed(KeyCode::Key2) {
    time.set_to_one_hour();
  }

  if keyboard_input.just_pressed(KeyCode::Key3) {
    time.set_to_one_day();
  }
}

#[derive(Resource)]
pub struct GameTime {
  timer: Timer,
  day: i32,
  hour: u32,
  minute: u32,
  second: u32,
}

impl GameTime {
  fn new() -> GameTime {
    GameTime {
      timer: Timer::from_seconds(1.0 / 60.0, TimerMode::Repeating),
      day: 1,
      hour: 0,
      minute: 0,
      second: 0,
    }
  }

  fn set_to_one_minute(&mut self) {
    println!("set to one minute");
    self.timer.set_duration(Duration::from_secs_f32(1.0 / 60.0));
  }

  fn set_to_one_hour(&mut self) {
    println!("set to one hour");
    self
      .timer
      .set_duration(Duration::from_secs_f32(1.0 / 60.0 / 60.0));
  }

  fn set_to_one_day(&mut self) {
    println!("set to one day");
    self
      .timer
      .set_duration(Duration::from_secs_f32(1.0 / 60.0 * 60.0 * 24.0));
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
