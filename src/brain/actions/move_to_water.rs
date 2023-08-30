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

use crate::markers;
use bevy::prelude::*;
use bevy_ecs_tilemap::tiles::TilePos;
use big_brain::prelude::*;

#[derive(Component, ActionBuilder, Clone, Debug)]
pub struct Action {
  speed: f32,
}

const MAX_DISTANCE: f32 = 0.1;

pub fn system(
  time: Res<Time>,
  waters: Query<&TilePos, With<markers::Water>>,
  mut positions: Query<&mut TilePos, Without<markers::Water>>,
  mut action_query: Query<(&Actor, &mut ActionState, &Action, &ActionSpan)>,
) {
  for (actor, mut action_state, move_to, span) in &mut action_query {
    let _guard = span.span().enter();
    match *action_state {
      ActionState::Requested => {
        debug!("Let's go find some water!");
        *action_state = ActionState::Executing;
      }
      ActionState::Executing => {
        let mut actor_position =
          positions.get_mut(actor.0).expect("actor has no position");
        trace!("Actor position: {:?}", actor_position.position);
        let closest_water_source =
          find_closest_water_source(&waters, &actor_position);
        let delta = closest_water_source.position - actor_position.position;
        let distance = delta.length();
        trace!("Distance: {}", distance);
        if distance > MAX_DISTANCE {
          trace!("Stepping closer.");
          let step_size = time.delta_seconds() * move_to.speed;
          let step = delta.normalize() * step_size.min(distance);
          actor_position.position += step;
        } else {
          debug!("We got there!");
          *action_state = ActionState::Success;
        }
      }
      ActionState::Cancelled => {
        *action_state = ActionState::Failure;
      }
      _ => {}
    }
  }
}
