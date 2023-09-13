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
  brain::{states::sociability, THRESHOLD},
  characters::component::Character,
  world::pathfinding::distance_from_positions,
};
use bevy::prelude::*;
use bevy_ecs_tilemap::tiles::TilePos;
use big_brain::prelude::*;

const MIN_DISTANCE: u32 = 2;

#[derive(Component, ScorerBuilder, Clone, Debug)]
pub struct Discussion;

pub fn scorer_system(
  states: Query<&sociability::Sociability>,
  mut query: Query<(&Actor, &mut Score), With<Discussion>>,
  characters: Query<(&Character, &TilePos)>,
) {
  let positions = characters.iter().map(|(_, pos)| pos).collect::<Vec<_>>();
  for (Actor(actor), mut score) in &mut query {
    if let Ok(sociability) = states.get(*actor) {
      let sociability = sociability.current / 100.;
      if sociability < THRESHOLD {
        score.set(sociability);
      } else {
        let position = characters.get(*actor).unwrap().1;
        let mut actors = 0;
        for target in positions.iter() {
          if position == *target {
            continue;
          }
          if distance_from_positions(position, target) < MIN_DISTANCE {
            actors += 1;
          }
        }
        if actors > 0 {
          score.set(sociability);
        } else {
          score.set(0.);
        }
      }
    }
  }
}
