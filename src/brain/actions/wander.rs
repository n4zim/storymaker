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
  time::event::GameTick,
  world::{pathfinding::path_from_to, ressource::WorldMap},
};
use bevy::prelude::*;
use bevy_ecs_tilemap::tiles::{TileColor, TilePos};
use big_brain::prelude::*;
use rand::Rng;

#[derive(Component, ActionBuilder, Clone, Debug)]
pub struct Wander {
  path: Vec<TilePos>,
}

impl Wander {
  pub fn new() -> Self {
    Self { path: vec![] }
  }
}

pub fn action(
  mut events: EventReader<GameTick>,
  world: Res<WorldMap>,
  mut query: Query<(&Actor, &mut ActionState, &mut Wander, &ActionSpan)>,
  mut characters: Query<(&mut Character, &mut TilePos, &mut TileColor)>,
) {
  for _clock in events.iter() {
    for (actor, mut state, mut action, span) in query.iter_mut() {
      let _guard = span.span().enter();

      let (mut character, mut position, mut color) =
        characters.get_mut(actor.0).expect("actor has no character");

      match *state {
        ActionState::Requested => {
          debug!("[REQUEST] Wander from {:?}", position);
          let mut rng: rand::rngs::ThreadRng = rand::thread_rng();
          loop {
            let destination = TilePos {
              x: rng.gen_range(0..world.size.x),
              y: rng.gen_range(0..world.size.y),
            };
            if !world.is_walkable(&destination) {
              continue;
            }
            if let Some(path) =
              path_from_to(&world, &position, &vec![destination])
            {
              action.path = path;
              *state = ActionState::Executing;
              break;
            }
          }
        }

        ActionState::Executing => {
          if action.path.is_empty() {
            trace!("[EXECUTED] Wandered to {:?}", position);
            *state = ActionState::Success;
          } else {
            let destination = action.path.remove(0);

            if world.is_walkable(&destination) {
              character.set_next_posture();
              character.set_next_direction(&position, &destination);

              position.x = destination.x;
              position.y = destination.y;

              color.0 = Color::Rgba {
                red: 1.0,
                green: 1.0,
                blue: 1.0,
                alpha: 1.0,
              };
            } else {
              color.0 = Color::RED;
              trace!("[EXECUTING] Can't walk to {:?}", position);
              *state = ActionState::Failure;
            }
          }
        }

        ActionState::Cancelled => {
          trace!("[CANCEL] Stopped wandering at {:?}", position);
          *state = ActionState::Failure;
        }
        _ => {}
      }
    }
  }
}
