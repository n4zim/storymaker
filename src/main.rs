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
use bevy_ecs_tilemap::prelude::*;

mod camera;

const QUADRANT_SIDE_LENGTH: u32 = 10;

fn startup(mut commands: Commands, asset_server: Res<AssetServer>) {
  commands.spawn(Camera2dBundle::default());

  let texture_handle: Handle<Image> = asset_server
    .load("island1/sprites/GustavoVituri/Isometric_MedievalFantasy_Tiles.png");

  let map_size = TilemapSize {
    x: QUADRANT_SIDE_LENGTH * 2,
    y: QUADRANT_SIDE_LENGTH * 2,
  };
  let quadrant_size = TilemapSize {
    x: QUADRANT_SIDE_LENGTH,
    y: QUADRANT_SIDE_LENGTH,
  };
  let mut tile_storage = TileStorage::empty(map_size);
  let tilemap_entity = commands.spawn_empty().id();
  let tilemap_id = TilemapId(tilemap_entity);

  fill_tilemap_rect(
    TileTextureIndex(0),
    TilePos { x: 0, y: 0 },
    quadrant_size,
    tilemap_id,
    &mut commands,
    &mut tile_storage,
  );

  fill_tilemap_rect(
    TileTextureIndex(1),
    TilePos {
      x: QUADRANT_SIDE_LENGTH,
      y: 0,
    },
    quadrant_size,
    tilemap_id,
    &mut commands,
    &mut tile_storage,
  );

  fill_tilemap_rect(
    TileTextureIndex(2),
    TilePos {
      x: 0,
      y: QUADRANT_SIDE_LENGTH,
    },
    quadrant_size,
    tilemap_id,
    &mut commands,
    &mut tile_storage,
  );

  fill_tilemap_rect(
    TileTextureIndex(3),
    TilePos {
      x: QUADRANT_SIDE_LENGTH,
      y: QUADRANT_SIDE_LENGTH,
    },
    quadrant_size,
    tilemap_id,
    &mut commands,
    &mut tile_storage,
  );

  let tile_size = TilemapTileSize { x: 32.0, y: 16.0 };
  let grid_size = tile_size.into();
  let map_type = TilemapType::Isometric(IsoCoordSystem::Diamond);

  commands.entity(tilemap_entity).insert(TilemapBundle {
    grid_size,
    size: map_size,
    storage: tile_storage,
    texture: TilemapTexture::Single(texture_handle),
    tile_size,
    map_type,
    transform: get_tilemap_center_transform(
      &map_size, &grid_size, &map_type, 0.0,
    ),
    ..Default::default()
  });
}

fn main() {
  App::new()
    .add_plugins(
      DefaultPlugins
        .set(WindowPlugin {
          primary_window: Some(Window {
            title: String::from("StoryMaker"),
            ..Default::default()
          }),
          ..default()
        })
        .set(ImagePlugin::default_nearest()),
    )
    .add_plugins(TilemapPlugin)
    .add_systems(Startup, startup)
    .add_systems(Update, camera::movement)
    .run();
}
