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

use crate::{
  brain::states::thirst::Thirst, characters::component::Character,
  time::history::History,
};
use bevy::prelude::*;
use bevy_egui::{
  egui::{self, *},
  EguiContexts,
};
use egui_extras::{Column, TableBuilder};
use std::collections::{BTreeMap, HashMap};

#[derive(Default, Resource)]
pub struct CurrentState {
  selected_character: Option<u32>,
}

pub fn system(
  mut contexts: EguiContexts,
  mut state: ResMut<CurrentState>,
  mut characters: Query<(&Character, &History, &mut Thirst)>,
) {
  let mut characters_names = BTreeMap::<String, &Character>::new();
  let mut characters_ids =
    HashMap::<u32, (&Character, &History, &mut Thirst)>::new();

  for (character, history, mut thirst) in &mut characters {
    characters_names.insert(character.get_name(), character);
    characters_ids.insert(character.id, (character, history, &mut thirst));
  }

  egui::SidePanel::right("sidebar")
    .default_width(400.0)
    .resizable(true)
    .show(contexts.ctx_mut(), |ui| {
      let height = ui.available_rect_before_wrap().height() / 3.0 - 10.0;

      ui.label(RichText::new("Characters").strong().size(16.0));
      ScrollArea::vertical()
        .id_source("characters")
        .auto_shrink([false; 2])
        .max_height(height)
        .show(ui, |ui| {
          for (name, character) in characters_names.iter() {
            let selected = if let Some(current) = state.selected_character {
              current == character.id
            } else {
              false
            };

            let label = selectable_label(
              ui,
              format!("{} ({})", name, character.get_gender()),
              selected,
            );

            if label.clicked() {
              state.selected_character =
                if selected { None } else { Some(character.id) };
            }
          }
        });

      let Some(selected) = state.selected_character else {
        return;
      };

      let (character, history, mut thirst) =
        characters_ids.get_mut(&selected).unwrap();
      let character_name = character.get_name();

      ui.separator();

      ui.label(
        RichText::new(format!("Actions of {}", character_name))
          .strong()
          .size(16.0),
      );
      egui::ScrollArea::horizontal()
        .id_source("actions")
        .max_height(height)
        .auto_shrink([false; 2])
        .show(ui, |ui| {
          TableBuilder::new(ui)
            .striped(true)
            .auto_shrink([false; 2])
            .cell_layout(egui::Layout::left_to_right(egui::Align::Center))
            .column(Column::auto())
            .column(Column::auto())
            .column(Column::remainder())
            .min_scrolled_height(0.0)
            .header(20.0, |mut header| {
              header.col(|ui| {
                ui.strong("Day");
              });
              header.col(|ui| {
                ui.strong("Time");
              });
              header.col(|ui| {
                ui.strong("Description");
              });
            })
            .body(|mut body| {
              for item in history.0.iter() {
                body.row(10.0, |mut row| {
                  row.col(|ui| {
                    ui.label(item.tick.day.to_string());
                  });
                  row.col(|ui| {
                    ui.label(format!(
                      "{:02}:{:02}:{:02}",
                      item.tick.hour, item.tick.minute, item.tick.second,
                    ));
                  });
                  row.col(|ui| {
                    ui.label(item.description.clone());
                  });
                });
              }
            });
        });

      ui.separator();

      ui.label(
        RichText::new(format!("States of {}", character_name))
          .strong()
          .size(16.0),
      );
      ScrollArea::vertical()
        .id_source("states")
        .auto_shrink([false; 2])
        .show(ui, |ui| {
          ui.add(
            egui::Slider::new(&mut thirst.current, 0.0..=10.0).text("Thirst"),
          );
        });
    });
}

fn selectable_label(ui: &mut Ui, text: String, selected: bool) -> Response {
  let text = if selected {
    RichText::new(text)
      .strong()
      .background_color(Color32::from_rgb(128, 128, 128))
  } else {
    RichText::new(text)
  };
  Label::new(text).sense(Sense::click()).ui(ui)
}
