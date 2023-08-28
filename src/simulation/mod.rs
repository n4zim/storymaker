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

use self::actors::spawner::ActorsSpawner;
use self::world::World;
use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;

mod actors;
mod world;

pub struct SimulationPlugin;

impl Plugin for SimulationPlugin {
  fn build(&self, app: &mut App) {
    app
      .add_plugins(TilemapPlugin)
      .insert_resource(TilemapRenderSettings {
        render_chunk_size: UVec2::new(3, 1),
        y_sort: true,
      })
      .add_systems(Startup, init);
  }
}

fn init(mut commands: Commands, asset_server: Res<AssetServer>) {
  let world = World::new(&asset_server, "island1");

  let mut spawner: ActorsSpawner =
    ActorsSpawner::new(world.size, world.grid, &mut commands, &asset_server);

  world.render(&mut commands, &mut spawner);

  commands.insert_resource(world);
  commands.insert_resource(spawner);
}
