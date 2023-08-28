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
    app
      .add_plugins(
        DefaultPlugins
          .set(WindowPlugin {
            primary_window: Some(Window {
              title: String::from("StoryMaker"),
              ..Default::default()
            }),
            ..default()
          })
          .set(ImagePlugin::default_nearest()),
      )
      .insert_resource(GameTime {
        timer: Timer::from_seconds(1.0 / 60.0, TimerMode::Repeating),
        day: 1,
        hour: 0,
        minute: 0,
        second: 0,
      })
      .add_event::<GameTick>()
      .add_state::<SpeedState>()
      .add_systems(Update, tick)
      .add_systems(Update, speed_commands);
  }
}

fn tick(
  mut game_time: ResMut<GameTime>,
  time: Res<Time>,
  speed: Res<State<SpeedState>>,
  mut events: EventWriter<GameTick>,
) {
  game_time.timer.tick(time.delta());
  if game_time.timer.finished() {
    if !game_time.timer.paused() {
      let iterations = match speed.get() {
        SpeedState::OneMinute => 1,
        SpeedState::OneHour => 60,
        SpeedState::OneDay => 1440,
      };
      for _ in 0..iterations {
        game_time.tick();
        events.send(GameTick);
      }
    }
  }
}

fn speed_commands(
  mut time: ResMut<GameTime>,
  current_state: Res<State<SpeedState>>,
  mut next_state: ResMut<NextState<SpeedState>>,
  keyboard_input: Res<Input<KeyCode>>,
) {
  if keyboard_input.just_pressed(KeyCode::Space) {
    if time.timer.paused() {
      time.timer.unpause();
    } else {
      time.timer.pause();
    }
    return;
  }

  let current_state = *current_state.get();

  if keyboard_input.just_pressed(KeyCode::Key1) {
    if current_state != SpeedState::OneMinute {
      next_state.set(SpeedState::OneMinute);
      if time.timer.paused() {
        time.timer.unpause();
      }
    }
  } else if keyboard_input.just_pressed(KeyCode::Key2) {
    if current_state != SpeedState::OneHour {
      next_state.set(SpeedState::OneHour);
      if time.timer.paused() {
        time.timer.unpause();
      }
    }
  } else if keyboard_input.just_pressed(KeyCode::Key3) {
    if current_state != SpeedState::OneDay {
      next_state.set(SpeedState::OneDay);
      if time.timer.paused() {
        time.timer.unpause();
      }
    }
  }
}

#[derive(Event)]
struct GameTick;

#[derive(Resource)]
pub struct GameTime {
  timer: Timer,
  day: i32,
  hour: u32,
  minute: u32,
  second: u32,
}

impl GameTime {
  fn tick(&mut self) {
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
  }

  pub fn to_string(&self) -> String {
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
  OneHour,
  OneDay,
}
