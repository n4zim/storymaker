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
  characters::component::Character,
  time::{
    event::GameTick,
    history::{History, HistoryItemStatus},
  },
  world::{map::WorldMap, markers::TalkTarget, pathfinding::path_from_to},
};
use bevy::prelude::*;
use bevy_ecs_tilemap::tiles::{TileColor, TilePos};
use bevy_turborand::prelude::*;
use big_brain::prelude::*;

#[derive(Component, ActionBuilder, Clone, Debug)]
pub struct Wait {
  max: f32,
}

impl Wait {
  pub fn new(max: f32) -> Self {
    Self { max }
  }
}

const NAME: &str = "Wait";

pub fn action(
  mut events: EventReader<GameTick>,
  mut query: Query<(&Actor, &mut ActionState, &mut Wait, &ActionSpan)>,
  mut characters: Query<(&Character, &TilePos, &mut History)>,
) {
  for tick in events.iter() {
    for (actor, mut state, mut _action, span) in query.iter_mut() {
      let _guard = span.span().enter();

      let (_, position, mut history) =
        characters.get_mut(actor.0).expect("actor has no character");

      match *state {
        ActionState::Requested => {
          debug!("[REQUEST] Waiting from {:?}", position);
          *state = ActionState::Executing;
          history.insert(HistoryItemStatus::Do, tick, &position, NAME);
        }

        ActionState::Cancelled => {
          trace!("[CANCEL] Stopped waiting at {:?}", position);
          *state = ActionState::Failure;
          history.insert(HistoryItemStatus::Cancel, tick, &position, NAME);
        }

        _ => {}
      }
    }
  }
}
