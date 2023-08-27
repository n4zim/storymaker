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
use bevy_egui::{
  egui::{self, Align2, FontId, RichText, Vec2},
  EguiContexts, EguiPlugin,
};

use crate::game::GameTime;

pub struct UIPlugin;

impl Plugin for UIPlugin {
  fn build(&self, app: &mut App) {
    app
      .init_resource::<State>()
      .add_plugins(EguiPlugin)
      .add_systems(Update, interface);
  }
}

fn interface(
  mut contexts: EguiContexts,
  mut state: ResMut<State>,
  time: Res<GameTime>,
) {
  egui::SidePanel::right("sidebar").default_width(400.0).show(
    contexts.ctx_mut(),
    |ui| {
      ui.add(egui::Slider::new(&mut state.fake, 0.0..=10.0).text("Fake"));
    },
  );

  egui::Window::new("clock")
    .title_bar(false)
    .resizable(false)
    .collapsible(false)
    .anchor(Align2::LEFT_BOTTOM, Vec2::new(30.0, -30.0))
    .show(contexts.ctx_mut(), |ui| {
      ui.label(
        RichText::new(time.to_string()).font(FontId::proportional(40.0)),
      );
    });
}

#[derive(Default, Resource)]
struct State {
  fake: f32,
}
