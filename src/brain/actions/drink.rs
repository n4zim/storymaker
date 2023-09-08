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
  brain::states::thirst,
  time::{
    event::GameTick,
    history::{History, HistoryItemStatus},
  },
  world::{map::WorldMap, markers, pathfinding::path_from_to},
};
use bevy::prelude::*;
use bevy_ecs_tilemap::tiles::{TileColor, TilePos};
use big_brain::prelude::*;

#[derive(Component, ActionBuilder, Clone, Debug)]
pub struct Drink {
  speed: f32,
}

impl Drink {
  pub fn new(speed: f32) -> Self {
    Self { speed }
  }
}

const NAME: &str = "Drink";

pub fn action(
  mut events: EventReader<GameTick>,
  world: Res<WorldMap>,
  mut query: Query<(&Actor, &mut ActionState, &Drink, &ActionSpan)>,
  mut thirsts: Query<
    (&TilePos, &mut TileColor, &mut History, &mut thirst::Thirst),
    Without<markers::Water>,
  >,
  waters: Query<&TilePos, With<markers::Water>>,
) {
  for tick in events.iter() {
    for (Actor(actor), mut state, action, span) in &mut query {
      let _guard = span.span().enter();
      let (position, mut color, mut history, mut thirst) =
        thirsts.get_mut(*actor).expect("actor has no thirst");
      //println!("Drink state: {:?} with thirst {:?}", state, thirst.current);
      match *state {
        ActionState::Requested => {
          debug!("[REQUEST] Drinking from {:?}", position);
          if let Some(path) =
            path_from_to(&world, &position, &waters.iter().cloned().collect())
          {
            if path[path.len() - 2] == *position {
              color.0 = Color::rgb(0.0, 0.0, 1.0);
              *state = ActionState::Executing;
              history.insert(HistoryItemStatus::Start, tick, position, NAME);
            } else {
              trace!("[REQUESTED] Not close enough to water");
              *state = ActionState::Failure;
            }
          } else {
            trace!("[REQUESTED] No path found to water from {:?}", position);
            *state = ActionState::Failure;
          }
        }
        ActionState::Executing => {
          thirst.current -= action.speed;
          if thirst.current <= 0.0 {
            thirst.current = 0.0;
            debug!("[EXECUTED] Drank from {:?}", position);
            color.0 = Color::rgb(1.0, 1.0, 1.0);
            *state = ActionState::Success;
            history.insert(HistoryItemStatus::End, tick, position, NAME);
          }
        }
        ActionState::Cancelled => {
          trace!("[CANCEL] Stopped drinking from {:?}", position);
          color.0 = Color::rgb(1.0, 1.0, 1.0);
          *state = ActionState::Failure;
          history.insert(HistoryItemStatus::Cancel, tick, position, NAME);
        }
        _ => {}
      }
    }
  }
}
