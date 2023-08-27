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

use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::TilemapTexture;

pub struct ActorsPlugin;

impl Plugin for ActorsPlugin {
  fn build(&self, app: &mut App) {
    app.add_systems(Startup, init);
  }
}

fn init(asset_server: Res<AssetServer>, mut commands: Commands) {
  let texture = TilemapTexture::Single(
    asset_server
      .load("sprites/AlexDreamer/Small-8-Direction-Characters_by_AxulArt.png"),
  );
}

#[derive(Component)]
pub struct Actor {
  name: String,
  position: Vec2,
}
