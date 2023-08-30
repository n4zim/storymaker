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

use crate::{brain::states::thirst, markers};
use bevy::prelude::*;
use bevy_ecs_tilemap::tiles::TilePos;
use big_brain::prelude::*;

#[derive(Component, ActionBuilder, Clone, Debug)]
pub struct Action {
  speed: f32,
}

pub fn system(
  time: Res<Time>,
  mut thirsts: Query<(&TilePos, &mut thirst::State), Without<markers::Water>>,
  waters: Query<&TilePos, With<markers::Water>>,
  mut query: Query<(&Actor, &mut ActionState, &Action, &ActionSpan)>,
) {
  for (Actor(actor), mut state, drink, span) in &mut query {
    let _guard = span.span().enter();
    let (actor_position, mut thirst) =
      thirsts.get_mut(*actor).expect("actor has no thirst");
    match *state {
      ActionState::Requested => {
        debug!("Drinking the water.");
        *state = ActionState::Executing;
      }
      ActionState::Executing => {
        let closest_water_source =
          find_closest_water_source(&waters, actor_position);
        let distance =
          (closest_water_source.position - actor_position.position).length();
        if distance < MAX_DISTANCE {
          trace!("Drinking!");
          thirst.thirst -= drink.per_second * time.delta_seconds();
          if thirst.thirst <= 0.0 {
            thirst.thirst = 0.0;
            *state = ActionState::Success;
          }
        } else {
          debug!("We're too far away!");
          *state = ActionState::Failure;
        }
      }
      ActionState::Cancelled => {
        *state = ActionState::Failure;
      }
      _ => {}
    }
  }
}
