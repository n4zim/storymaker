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

use bevy::{
  prelude::{Handle, Plugin, Vec2},
  render::render_resource::Texture,
};

pub struct MapPlugin;

impl Plugin for MapPlugin {
  fn build(&self, app: &mut bevy::prelude::App) {}
}

struct Map {
  size: Vec2,
  grid_size: Vec2,
  tile_size: Vec2,
  texture: Handle<Texture>,
  tiles: Vec<Tile>,
}
