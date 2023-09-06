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

use self::map::WorldMap;
use crate::characters::spawner::CharactersSpawner;
use bevy::prelude::*;
use bevy_ecs_tilemap::{prelude::TilemapRenderSettings, TilemapPlugin};
use bevy_turborand::prelude::*;

pub mod map;
pub mod markers;
pub mod pathfinding;

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
  fn build(&self, app: &mut App) {
    app
      .add_plugins((TilemapPlugin, RngPlugin::default()))
      .insert_resource(TilemapRenderSettings {
        render_chunk_size: UVec2::new(3, 1),
        y_sort: true,
      })
      .add_systems(Startup, render);
  }
}

fn render(
  mut commands: Commands,
  asset_server: Res<AssetServer>,
  mut global_rng: ResMut<GlobalRng>,
) {
  commands.spawn(Camera2dBundle::default());

  let world = WorldMap::new(&asset_server, "island1");

  let mut spawner = CharactersSpawner::new(
    world.size,
    world.grid,
    &mut commands,
    &asset_server,
  );

  world.render(&mut commands, &mut spawner, &mut global_rng);

  commands.insert_resource(world);
  commands.insert_resource(spawner);
}
