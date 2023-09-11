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

use super::component::{Character, CharacterGender};
use bevy::prelude::*;
use std::collections::HashMap;

#[derive(PartialEq)]
pub enum RelationshipKind {
  Enemy,
  Acquaintance,
  Friend,
  Lover,
  Spouse,
}

type Relationship = (RelationshipKind, i8);

#[derive(Component)]
pub struct Relationships {
  map: HashMap<Entity, Relationship>,
  interest: CharacterGender,
}

impl Relationships {
  pub fn new(interest: CharacterGender) -> Relationships {
    Relationships {
      map: HashMap::new(),
      interest,
    }
  }

  pub fn add(&mut self, entity: Entity) {
    self.map.insert(entity, (RelationshipKind::Acquaintance, 0));
  }

  pub fn knows(&self, entity: Entity) -> Option<&Relationship> {
    self.map.get(&entity)
  }

  pub fn count(&self, kind: RelationshipKind) -> usize {
    self.map.values().filter(|&v| v.0 == kind).count()
  }

  pub fn upgrade(&mut self, character: &Character, steps: i8) {
    let spouces = self.count(RelationshipKind::Spouse);
    if let Some((kind, strength)) = self.map.get_mut(&character.entity) {
      match kind {
        RelationshipKind::Enemy => {
          let value = *strength - steps;
          if value < 0 {
            *kind = RelationshipKind::Acquaintance;
            *strength = -value;
          } else {
            *strength = value;
          }
        }
        RelationshipKind::Acquaintance => {
          let value = *strength + steps;
          if value > 100 {
            *kind = RelationshipKind::Friend;
            *strength = 100 - value;
          } else {
            *strength = value;
          }
        }
        RelationshipKind::Friend => {
          let value = *strength + steps;
          if value >= 100 {
            if self.interest == character.gender {
              *kind = RelationshipKind::Lover;
              *strength = 100 - value;
            } else {
              *strength = 100;
            }
          } else {
            *strength = value;
          }
        }
        RelationshipKind::Lover => {
          let value = *strength + steps;
          if value > 100 {
            if spouces == 0 {
              *kind = RelationshipKind::Spouse;
              *strength = 100 - value;
            } else {
              *strength = 100;
            }
          } else {
            *strength = value;
          }
        }
        RelationshipKind::Spouse => {
          let value = *strength + steps;
          if value > 100 {
            *strength = 100;
          } else {
            *strength = value;
          }
        }
      }
    }
  }

  pub fn downgrade(&mut self, character: &Character, steps: i8) {
    if let Some((kind, strength)) = self.map.get_mut(&character.entity) {
      match kind {
        RelationshipKind::Enemy => {
          let value = *strength + steps;
          if value > 100 {
            *strength = 100;
          } else {
            *strength = value;
          }
        }
        RelationshipKind::Acquaintance => {
          let value = *strength - steps;
          if value < 0 {
            *kind = RelationshipKind::Enemy;
            *strength = -value;
          } else {
            *strength = value;
          }
        }
        RelationshipKind::Friend => {
          let value = *strength - steps;
          if value < 0 {
            *kind = RelationshipKind::Acquaintance;
            *strength = -value;
          } else {
            *strength = value;
          }
        }
        RelationshipKind::Lover | RelationshipKind::Spouse => {
          let value = *strength - steps;
          if value < 0 {
            *kind = RelationshipKind::Friend;
            *strength = -value;
          } else {
            *strength = value;
          }
        }
      }
    }
  }
}
