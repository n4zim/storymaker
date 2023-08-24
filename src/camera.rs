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

use bevy::{input::Input, math::Vec3, prelude::*, render::camera::Camera};

pub fn movement(
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

    if keyboard_input.pressed(KeyCode::Home) {
      ortho.scale += 0.1;
    }

    if keyboard_input.pressed(KeyCode::End) {
      ortho.scale -= 0.1;
    }

    if ortho.scale < 0.1 {
      ortho.scale = 0.1;
    } else if ortho.scale > 10.0 {
      ortho.scale = 10.0;
    }

    let z = transform.translation.z;
    transform.translation += time.delta_seconds() * direction * 500.;
    transform.translation.z = z;
  }
}
