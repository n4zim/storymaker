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

use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct WorldConfig {
  pub size_x: u32,
  pub size_y: u32,
  pub grid_x: f32,
  pub grid_y: f32,
  pub tile_sets: HashMap<String, WorldConfigTileSet>,
  pub layers: Vec<WorldConfigLayer>,
}

#[derive(Serialize, Deserialize)]
pub struct WorldConfigTileSet {
  pub source: String,
  pub size_x: f32,
  pub size_y: f32,
}

#[derive(Serialize, Deserialize)]
pub struct WorldConfigLayer {
  pub name: String,
  pub tiles: Vec<Vec<u32>>,
}
