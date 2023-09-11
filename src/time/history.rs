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

use crate::time::event::GameTick;
use bevy::prelude::*;
use bevy_ecs_tilemap::tiles::TilePos;

pub enum HistoryItemStatus {
  Do,
  Cancel,
}

impl HistoryItemStatus {
  pub fn to_string(&self) -> String {
    match self {
      HistoryItemStatus::Do => "do".to_string(),
      HistoryItemStatus::Cancel => "cancel".to_string(),
    }
  }
}

pub struct HistoryItem {
  pub status: HistoryItemStatus,
  pub tick: GameTick,
  pub position: TilePos,
  pub name: String,
}

#[derive(Component)]
pub struct History(pub Vec<HistoryItem>);

impl History {
  pub fn new() -> History {
    History(vec![])
  }

  pub fn insert(
    &mut self,
    status: HistoryItemStatus,
    tick: &GameTick,
    position: &TilePos,
    name: &str,
  ) {
    self.0.push(HistoryItem {
      status,
      tick: tick.clone(),
      position: position.clone(),
      name: name.to_string(),
    });
  }
}
