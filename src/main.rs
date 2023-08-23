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

//use map::TileKind as TK;
use bevy::utils::Duration;
use bevy::{asset::ChangeWatcher, prelude::*};
use bevy_ecs_tilemap::prelude::*;

//mod actions;
//mod actors;
//mod map;
//mod time;
//mod world;

//const INTERVAL: u64 = 1;

pub type Position = (usize, usize);

#[tokio::main]
async fn main() {
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
          .set(ImagePlugin::default_nearest())
          .set(AssetPlugin {
              watch_for_changes: ChangeWatcher::with_delay(Duration::from_millis(200)),
              ..default()
          }),
  )
  .add_plugins(TilemapPlugin)
  .add_systems(Startup, startup)
  .add_systems(Update, camera_movement)
  .run();



  /*let map = map::Map::new_with_tile_kinds(vec![
    vec![ TK::Water, TK::Water,  TK::Water,  TK::Water,  TK::Water ],
    vec![ TK::Water, TK::Firm,   TK::House,  TK::Firm,   TK::Water ],
    vec![ TK::Water, TK::House,  TK::House,  TK::House,  TK::Water ],
    vec![ TK::Water, TK::Firm,   TK::Grass,  TK::Firm,   TK::Water ],
    vec![ TK::Water, TK::House,  TK::Forest, TK::House,  TK::Water ],
    vec![ TK::Water, TK::Forest, TK::Mine,   TK::Forest, TK::Water ],
    vec![ TK::Water, TK::Water,  TK::Water,  TK::Water,  TK::Water ],
  ]);

  //map.print();

  let mut world = world::World::new(map);

  for actor in world.actors.iter_mut() {
    actor.tick();
  }

  let mut interval = interval(Duration::from_millis(INTERVAL));

  loop {
    interval.tick().await;
    world.tick();
  }*/
}

const QUADRANT_SIDE_LENGTH: u32 = 576;

fn startup(mut commands: Commands, asset_server: Res<AssetServer>) {
  commands.spawn(Camera2dBundle::default());

  let texture_handle: Handle<Image> = asset_server.load("island1/map/island1.png");

  // In total, there will be `(QUADRANT_SIDE_LENGTH * 2) * (QUADRANT_SIDE_LENGTH * 2)` tiles.
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

  fill_tilemap_rect_color(
      TileTextureIndex(5),
      TilePos { x: 0, y: 0 },
      quadrant_size,
      Color::rgba(1.0, 0.0, 0.0, 1.0),
      tilemap_id,
      &mut commands,
      &mut tile_storage,
  );

  fill_tilemap_rect_color(
      TileTextureIndex(5),
      TilePos {
          x: QUADRANT_SIDE_LENGTH,
          y: 0,
      },
      quadrant_size,
      Color::rgba(0.0, 1.0, 0.0, 1.0),
      tilemap_id,
      &mut commands,
      &mut tile_storage,
  );

  fill_tilemap_rect_color(
      TileTextureIndex(5),
      TilePos {
          x: 0,
          y: QUADRANT_SIDE_LENGTH,
      },
      quadrant_size,
      Color::rgba(0.0, 0.0, 1.0, 1.0),
      tilemap_id,
      &mut commands,
      &mut tile_storage,
  );

  fill_tilemap_rect_color(
      TileTextureIndex(5),
      TilePos {
          x: QUADRANT_SIDE_LENGTH,
          y: QUADRANT_SIDE_LENGTH,
      },
      quadrant_size,
      Color::rgba(1.0, 1.0, 0.0, 1.0),
      tilemap_id,
      &mut commands,
      &mut tile_storage,
  );

  let tile_size = TilemapTileSize { x: 16.0, y: 16.0 };
  let grid_size = tile_size.into();
  let map_type = TilemapType::default();

  commands.entity(tilemap_entity).insert(TilemapBundle {
      grid_size,
      size: map_size,
      storage: tile_storage,
      texture: TilemapTexture::Single(texture_handle),
      tile_size,
      map_type: TilemapType::Square,
      transform: get_tilemap_center_transform(&map_size, &grid_size, &map_type, 0.0),
      ..Default::default()
  });
}

pub fn camera_movement(
  time: Res<Time>,
  keyboard_input: Res<Input<KeyCode>>,
  mut query: Query<(&mut Transform, &mut OrthographicProjection), With<Camera>>,
) {
  for (mut transform, mut ortho) in query.iter_mut() {
      let mut direction = Vec3::ZERO;

      if keyboard_input.pressed(KeyCode::Left) {
          direction -= Vec3::new(1.0, 0.0, 0.0);
      }

      if keyboard_input.pressed(KeyCode::Right) {
          direction += Vec3::new(1.0, 0.0, 0.0);
      }

      if keyboard_input.pressed(KeyCode::Up) {
          direction += Vec3::new(0.0, 1.0, 0.0);
      }

      if keyboard_input.pressed(KeyCode::Down) {
          direction -= Vec3::new(0.0, 1.0, 0.0);
      }

      if keyboard_input.pressed(KeyCode::F) {
          ortho.scale += 0.1;
      }

      if keyboard_input.pressed(KeyCode::W) {
          ortho.scale -= 0.1;
      }

      if ortho.scale < 0.5 {
          ortho.scale = 0.5;
      }

      let z = transform.translation.z;
      transform.translation += time.delta_seconds() * direction * 500.;
      // Important! We need to restore the Z values when moving the camera around.
      // Bevy has a specific camera setup and this can mess with how our layers are shown.
      transform.translation.z = z;
  }
}
