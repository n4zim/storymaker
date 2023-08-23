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

use crate::{Position, actions::{Action, Sleep}};

#[derive(Clone)]
pub struct Actor {
  pub name: String,
  pub house: Position,

  pub position: Position,

  action: Box<dyn Action>,
  history: Vec<Box<dyn Action>>,
}

impl Actor {
  pub fn new(name: String, house: Position) -> Actor {
    Actor {
      name,
      house,
      position: house,
      action: Box::new(Sleep {
        duration: 5,
        started: false,
      }),
      history: Vec::new(),
    }
  }

  pub fn tick(&mut self) {
    let action = self.action.tick(self.clone());
    self.history.push(self.action);
    self.action = action;
  }
}
