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
  characters::Character, game::GameTick, pathfinding::paths, world::WorldMap,
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
  mut query: Query<(
    &mut Character,
    &mut TilePos,
    &mut TileColor,
    &mut ActionState,
    &mut Wander,
    &ActionSpan,
  )>,
) {
  for _clock in events.iter() {
    for (mut character, mut position, mut color, mut state, mut action, span) in
      query.iter_mut()
    {
      let _guard = span.span().enter();
      match *state {
        ActionState::Requested => {
          debug!("[REQUEST] Wander from {:?}", position);
          let mut rng: rand::rngs::ThreadRng = rand::thread_rng();
          loop {
            if let Some(path) = paths(
              &world,
              &position,
              &vec![TilePos {
                x: rng.gen_range(0..world.size.x),
                y: rng.gen_range(0..world.size.y),
              }],
            ) {
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
            character.set_next_posture();

            let destination = action.path.remove(0);

            if world.is_walkable(&destination) {
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

/*pub fn directions_system(mut query: Query<(&mut Character, &mut TilePos, &)>) {
  for (mut actor, position) in query.iter_mut() {
    if !actor.path.is_empty() {
      let destination = actor.path[0];
      actor.direction = if position.x < destination.x {
        if position.y < destination.y {
          CharacterDirection::BottomRight
        } else if position.y > destination.y {
          CharacterDirection::Bottom
        } else {
          CharacterDirection::Right
        }
      } else if position.x > destination.x {
        if position.y < destination.y {
          CharacterDirection::Top
        } else if position.y > destination.y {
          CharacterDirection::TopLeft
        } else {
          CharacterDirection::Left
        }
      } else if position.y < destination.y {
        CharacterDirection::TopRight
      } else {
        CharacterDirection::BottomLeft
      };
    }
  }
}

pub fn texture_system(mut query: Query<(&mut Character, &mut TileTextureIndex)>) {
  for (actor, mut texture_index) in query.iter_mut() {
    texture_index
      .set(Box::new(actor.get_texture_index()))
      .unwrap()
  }
}*/
