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
  egui::{self, *},
  EguiContexts,
};

use crate::characters::component::Character;

#[derive(Default, Resource)]
pub struct CurrentState {
  fake: f32,
}

pub fn system(
  mut contexts: EguiContexts,
  mut state: ResMut<CurrentState>,
  mut actors: Query<&mut Character>,
) {
  let actors = actors.iter_mut().collect::<Vec<_>>();
  egui::SidePanel::right("sidebar")
    .default_width(400.0)
    .resizable(true)
    .show(contexts.ctx_mut(), |ui| {
      let height = ui.available_rect_before_wrap().height() / 3.0;
      ui.label(RichText::new("Actors").size(16.0));
      ScrollArea::vertical()
        .id_source("actors")
        .auto_shrink([false; 2])
        .max_height(height)
        .show_rows(
          ui,
          ui.text_style_height(&TextStyle::Body),
          actors.len(),
          |ui, row_range| {
            for row in row_range {
              let actor = actors.get(row).unwrap();
              ui.label(actor.get_name());
            }
          },
        );

      ui.separator();

      ui.vertical(|ui| {
        ui.label(
          RichText::new(
            "States
        ",
          )
          .size(16.0),
        );
        ui.add(egui::Slider::new(&mut state.fake, 0.0..=10.0).text("Thirst"));
        ui.add(egui::Slider::new(&mut state.fake, 0.0..=10.0).text("Hunger"));
      });
    });
}
