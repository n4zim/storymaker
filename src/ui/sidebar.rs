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
  brain::states::thirst::Thirst,
  characters::component::Character,
  time::history::{History, HistoryItem, HistoryItemStatus},
};
use bevy::prelude::*;
use bevy_egui::{
  egui::{self, *},
  EguiContexts,
};
use egui_extras::{Column, TableBuilder};
use std::collections::BTreeMap;

#[derive(Default, Resource)]
pub struct CurrentState {
  selected_character: Option<u32>,
}

pub fn system(
  mut contexts: EguiContexts,
  mut state: ResMut<CurrentState>,
  mut characters: Query<(&Character, &History, &mut Thirst)>,
) {
  let mut current_character: Option<&Character> = None;
  let mut current_history: Option<&History> = None;
  let mut current_thirst: Option<Mut<'_, Thirst>> = None;

  let mut characters_list = BTreeMap::<String, u32>::new();
  for (character, history, thirst) in &mut characters {
    characters_list.insert(
      format!("{} ({})", character.get_name(), character.get_gender()),
      character.id,
    );
    if let Some(selected) = state.selected_character {
      if selected == character.id {
        current_character.replace(character);
        current_history.replace(history);
        current_thirst.replace(thirst);
      }
    }
  }

  egui::SidePanel::right("sidebar")
    .default_width(300.0)
    .resizable(true)
    .show(contexts.ctx_mut(), |ui| {
      let height = ui.available_rect_before_wrap().height() / 3.0 - 10.0;

      ui.heading(RichText::new("Characters").strong().size(16.0));
      ScrollArea::vertical()
        .id_source("characters")
        .auto_shrink([false; 2])
        .max_height(height)
        .show(ui, |ui| {
          characters_ui(ui, &characters_list, &mut state);
        });

      if current_character.is_none() || current_history.is_none() {
        return;
      };

      let character_name = current_character.unwrap().get_name();

      ui.separator();

      ui.heading(
        RichText::new(format!("Actions of {}", character_name))
          .strong()
          .size(16.0),
      );
      egui::ScrollArea::horizontal()
        .id_source("actions")
        .max_height(height)
        .auto_shrink([false; 2])
        .show(ui, |ui| {
          actions_ui(ui, current_history.unwrap());
        });

      if current_thirst.is_none() {
        return;
      };

      ui.separator();

      ui.heading(
        RichText::new(format!("States of {}", character_name))
          .strong()
          .size(16.0),
      );
      ScrollArea::vertical()
        .id_source("states")
        .auto_shrink([false; 2])
        .show(ui, |ui| {
          states_ui(ui, current_thirst.unwrap());
        });
    });
}

fn characters_ui(
  ui: &mut Ui,
  characters: &BTreeMap<String, u32>,
  state: &mut CurrentState,
) {
  for (name, id) in characters.iter() {
    let selected = if let Some(current) = state.selected_character {
      current == *id
    } else {
      false
    };
    if selectable_label(ui, name.clone(), selected).clicked() {
      state.selected_character = if selected { None } else { Some(*id) };
    }
  }
}

fn actions_ui(ui: &mut Ui, history: &History) {
  TableBuilder::new(ui)
    .striped(true)
    .auto_shrink([false; 2])
    .cell_layout(egui::Layout::left_to_right(egui::Align::Center))
    .column(Column::auto())
    .column(Column::auto())
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
        ui.strong("Position");
      });
      header.col(|ui| {
        ui.strong("Type");
      });
      header.col(|ui| {
        ui.strong("Name");
      });
    })
    .body(|mut body| {
      for item in history.0.iter().rev() {
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
            ui.label(format!("{}:{}", item.position.x, item.position.y));
          });
          row.col(|ui| {
            ui.label(match item.status {
              HistoryItemStatus::Start => "Start",
              HistoryItemStatus::End => "End",
              HistoryItemStatus::Cancel => "Cancel",
            });
          });
          row.col(|ui| {
            ui.label(item.name.clone());
          });
        });
      }
    });
}

fn states_ui(ui: &mut Ui, mut current_thirst: Mut<'_, Thirst>) {
  ui.spacing_mut().slider_width = 150.0;
  ui.horizontal(|ui| {
    ui.add(
      egui::Slider::new(&mut current_thirst.current, 0.0..=100.0)
        .text(RichText::new("Thirst").strong()),
    );
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
