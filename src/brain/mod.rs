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

use self::actions::*;
use self::scorers::*;
use self::states::*;
use bevy::ecs::system::EntityCommands;
use bevy::prelude::*;
use big_brain::prelude::*;

mod actions;
mod scorers;
mod states;

pub struct BrainPlugin;

impl Plugin for BrainPlugin {
  fn build(&self, app: &mut App) {
    app
      .add_plugins(BigBrainPlugin::new(PreUpdate))
      .add_systems(First, thirsty::system)
      .add_systems(Update, thirst::system)
      .add_systems(
        PreUpdate,
        (drink::system, move_to_water::system).in_set(BigBrainSet::Actions),
      );
  }
}

pub fn insert_bundle(entity: &mut EntityCommands) {
  entity.insert((
    thirst::State::new(75.0, 1.0),
    Thinker::build()
      .picker(FirstToScore { threshold: 0.8 })
      .when(
        thirsty::Scorer,
        Steps::build()
          .label("MoveAndDrink")
          .step(move_to_water::Action { speed: 1.0 })
          .step(drink::Action { speed: 1.0 }),
      ),
  ));
}
