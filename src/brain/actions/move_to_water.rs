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

use crate::{game::GameTick, markers, pathfinding::paths, world::WorldMap};
use bevy::prelude::*;
use bevy_ecs_tilemap::tiles::TilePos;
use big_brain::prelude::*;

#[derive(Component, ActionBuilder, Clone, Debug)]
pub struct MoveToWater {
  speed: f32,
  path: Vec<TilePos>,
}

impl MoveToWater {
  pub fn new(speed: f32) -> Self {
    Self {
      speed,
      path: Vec::new(),
    }
  }
}

pub fn action(
  mut events: EventReader<GameTick>,
  world: Res<WorldMap>,
  mut query: Query<(&Actor, &mut ActionState, &mut MoveToWater, &ActionSpan)>,
  mut positions: Query<&mut TilePos, Without<markers::Water>>,
  waters: Query<&TilePos, With<markers::Water>>,
) {
  for _clock in events.iter() {
    for (actor, mut state, mut action, span) in &mut query {
      let _guard = span.span().enter();
      let mut position =
        positions.get_mut(actor.0).expect("actor has no position");
      match *state {
        ActionState::Requested => {
          debug!("[REQUEST] Moving to water from {:?}", position);
          if let Some(path) =
            paths(&world, &position, &waters.iter().cloned().collect())
          {
            action.path = path.iter().take(path.len() - 1).cloned().collect();
            *state = ActionState::Executing;
          } else {
            trace!("[REQUESTED] No path found to water from {:?}", position);
            *state = ActionState::Failure;
          }
        }
        ActionState::Executing => {
          if action.path.is_empty() {
            debug!("[EXECUTED] Arrived at water to drink at {:?}", position);
            *state = ActionState::Success;
          } else {
            let destination = action.path.remove(0);
            position.x = destination.x;
            position.y = destination.y;
          }
        }
        ActionState::Cancelled => {
          trace!("[CANCEL] Stopped moving to water from {:?}", position);
          *state = ActionState::Failure;
        }
        _ => {}
      }
    }
  }
}
