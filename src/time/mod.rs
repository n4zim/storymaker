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

use self::{
  clock::GameClock, event::GameTick, speed::GameSpeed, timer::GameTimer,
};
use bevy::prelude::*;

pub mod clock;
pub mod event;
pub mod history;
pub mod speed;
pub mod timer;

pub struct TimePlugin;

impl Plugin for TimePlugin {
  fn build(&self, app: &mut App) {
    app
      .insert_resource(GameTimer::default())
      .insert_resource(GameClock::default())
      .add_state::<GameSpeed>()
      .add_event::<GameTick>()
      .add_systems(Update, tick);
  }
}

fn tick(
  mut game_time: ResMut<GameTimer>,
  mut game_clock: ResMut<GameClock>,
  time: Res<Time>,
  speed: Res<State<GameSpeed>>,
  mut events: EventWriter<GameTick>,
) {
  game_time.0.tick(time.delta());
  if game_time.0.finished() {
    if !game_time.0.paused() {
      let iterations = match speed.get() {
        GameSpeed::OneMinute => 1,
        GameSpeed::OneHour => 60,
        GameSpeed::OneDay => 1440,
      };
      for _ in 0..iterations {
        events.send(GameTick {
          day: game_clock.day,
          hour: game_clock.hour,
          minute: game_clock.minute,
          second: game_clock.second,
          total: game_clock.total,
        });
        game_clock.tick();
      }
    }
  }
}
