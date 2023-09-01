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
  input::{
    mouse::{MouseMotion, MouseWheel},
    Input,
  },
  math::Vec3,
  prelude::*,
  render::camera::Camera,
  window::PrimaryWindow,
};
use bevy_egui::EguiContext;

pub fn keyboard_movement(
  time: Res<Time>,
  keyboard_input: Res<Input<KeyCode>>,
  mut query: Query<&mut Transform, With<Camera>>,
) {
  for mut transform in query.iter_mut() {
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

    let z = transform.translation.z;
    transform.translation += time.delta_seconds() * direction * 500.;
    transform.translation.z = z;
  }
}

pub fn scroll_zoom(
  mut scroll: EventReader<MouseWheel>,
  mut query: Query<&mut OrthographicProjection, With<Camera>>,
  mut egui: Query<&mut EguiContext, With<PrimaryWindow>>,
) {
  if let Ok(mut ctx) = egui.get_single_mut() {
    if ctx.get_mut().is_pointer_over_area() {
      return;
    }
  } else {
    return;
  }
  for mut ortho in query.iter_mut() {
    for event in scroll.iter() {
      ortho.scale -= event.y / 10.0;
      if ortho.scale < 0.1 {
        ortho.scale = 0.1;
      } else if ortho.scale > 10.0 {
        ortho.scale = 10.0;
      }
    }
  }
}

pub fn middle_click_movement(
  mut mouse_motion_events: EventReader<MouseMotion>,
  input: Res<Input<MouseButton>>,
  mut query: Query<&mut Transform, With<Camera>>,
) {
  let mut delta = Vec2::ZERO;
  if input.pressed(MouseButton::Middle) || input.pressed(MouseButton::Right) {
    for event in mouse_motion_events.iter() {
      delta -= event.delta;
    }
  }
  if delta != Vec2::ZERO {
    for mut transform in query.iter_mut() {
      transform.translation.x += delta.x * transform.scale.x * 2.0;
      transform.translation.y -= delta.y * transform.scale.y * 2.0;
    }
  }
}
