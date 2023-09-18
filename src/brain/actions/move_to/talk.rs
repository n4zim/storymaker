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

use crate::{
  brain::constants::MIN_TALK_DISTANCE,
  characters::component::Character,
  time::{
    event::GameTick,
    history::{History, HistoryItemStatus},
  },
  world::{
    map::WorldMap,
    markers::TalkTarget,
    pathfinding::{distance_from_positions, path_from_to},
  },
};
use bevy::prelude::*;
use bevy_ecs_tilemap::tiles::TilePos;
use big_brain::prelude::*;

#[derive(Component, ActionBuilder, Clone, Debug)]
pub struct MoveToTalk {
  _speed: f32,
  path: Vec<TilePos>,
}

impl MoveToTalk {
  pub fn new(speed: f32) -> Self {
    Self {
      path: vec![],
      _speed: speed,
    }
  }
}

const NAME: &str = "MoveToTalk";

pub fn action(
  mut events: EventReader<GameTick>,
  world: Res<WorldMap>,
  mut query: Query<(&Actor, &mut ActionState, &mut MoveToTalk, &ActionSpan)>,
  mut characters: Query<(
    &mut Character,
    &mut TilePos,
    &mut History,
    Option<&TalkTarget>,
  )>,
  available: Query<(&Character, &TilePos), Without<TalkTarget>>,
  mut commands: Commands,
) {
  for tick in events.iter() {
    for (actor, mut state, mut action, span) in query.iter_mut() {
      let _guard = span.span().enter();

      let (mut character, mut position, mut history, target) =
        characters.get_mut(actor.0).expect("actor has no character");

      match *state {
        ActionState::Requested => {
          if target.is_some() {
            debug!(
              "[REQUEST] Waiting for someone to talk to at {:?}",
              position
            );
            *state = ActionState::Success;
            continue;
          }
          debug!("[REQUEST] Want to talk from {:?}", position);
          let mut actor_position: Option<&TilePos> = None;
          for (_, sub_position) in available.iter() {
            if position.to_owned() == *sub_position {
              continue;
            }
            if distance_from_positions(&position, sub_position)
              < MIN_TALK_DISTANCE
            {
              actor_position = Some(sub_position);
              break;
            }
          }
          if let Some(actor_position) = actor_position {
            if let Some(path) =
              path_from_to(&world, &position, &vec![actor_position.to_owned()])
            {
              action.path = path;
              commands.entity(character.entity).insert(TalkTarget);
              *state = ActionState::Executing;
              history.insert(HistoryItemStatus::Do, tick, &position, NAME);
            } else {
              debug!("[REQUEST] Can't find a path to {:?}", actor_position);
              *state = ActionState::Failure;
            }
          } else {
            debug!("[REQUEST] No one to talk to at {:?}", position);
            *state = ActionState::Failure;
          }
        }

        ActionState::Executing => {
          if action.path.is_empty() {
            debug!("[EXECUTED] Talked to {:?}", position);
            *state = ActionState::Success;
          } else {
            let destination = action.path.remove(0);

            if world.is_walkable(&destination) {
              character.set_next_posture();
              character.set_next_direction(&position, &destination);

              position.x = destination.x;
              position.y = destination.y;
            } else {
              debug!("[EXECUTING] Can't walk to {:?}", position);
              *state = ActionState::Failure;
            }
          }
        }

        ActionState::Cancelled => {
          debug!("[CANCEL] Stopped talking at {:?}", position);
          *state = ActionState::Failure;
          history.insert(HistoryItemStatus::Cancel, tick, &position, NAME);
        }
        _ => {}
      }
    }
  }
}
