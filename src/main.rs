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

fn startup(mut commands: Commands, asset_server: Res<AssetServer>) {
  commands.spawn(Camera2dBundle::default());

  let texture_handle: Handle<Image> = asset_server
    .load("island1/sprites/GustavoVituri/Isometric_MedievalFantasy_Tiles.png");

  let map_size = TilemapSize { x: 10, y: 10 };
  let mut tile_storage = TileStorage::empty(map_size);
  let tilemap_entity = commands.spawn_empty().id();
  let tilemap_id = TilemapId(tilemap_entity);

  commands.entity(tilemap_id.0).with_children(|parent| {
    for y in 0..map_size.y {
      for x in 0..map_size.x {
        let tile_pos = TilePos { x, y };
        println!("tile_pos: {:?}", tile_pos);
        let tile_entity = parent
          .spawn(TileBundle {
            position: tile_pos,
            tilemap_id,
            texture_index: TileTextureIndex(0),
            ..Default::default()
          })
          .id();
        tile_storage.set(&tile_pos, tile_entity);
      }
    }
  });

  let tile_size = TilemapTileSize { x: 16.0, y: 17.0 };
  let grid_size = TilemapTileSize { x: 16.0, y: -8.5 }.into();
  let map_type = TilemapType::Isometric(IsoCoordSystem::Staggered);

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
