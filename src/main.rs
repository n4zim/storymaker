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

use bevy::{log::LogPlugin, prelude::*};
use bevy_ecs_tilemap::{prelude::TilemapRenderSettings, TilemapPlugin};

mod brain;
mod camera;
mod characters;
mod game;
mod markers;
mod pathfinding;
mod ui;
mod world;

fn main() {
  App::new()
    .add_plugins((
      DefaultPlugins
        .set(WindowPlugin {
          primary_window: Some(Window {
            title: String::from("StoryMaker"),
            ..Default::default()
          }),
          ..default()
        })
        .set(ImagePlugin::default_nearest())
        .set(LogPlugin {
          filter: "wgpu=error,naga=warn".to_string(),
          ..default()
        }),
      TilemapPlugin,
      game::GamePlugin,
      brain::BrainPlugin,
      camera::CameraPlugin,
      ui::UIPlugin,
    ))
    .insert_resource(TilemapRenderSettings {
      render_chunk_size: UVec2::new(3, 1),
      y_sort: true,
    })
    .run();
}
