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
  time::history::History, world::markers::Selected,
};
use bevy::prelude::*;
use bevy_egui::{
  egui::{self, *},
  EguiContexts,
};
use egui_extras::{Column, TableBuilder};
use std::collections::BTreeMap;

pub fn system(
  mut commands: Commands,
  mut contexts: EguiContexts,
  characters: Query<&Character>,
  mut selected: Query<(&Character, &History, &mut Thirst), With<Selected>>,
) {
  egui::SidePanel::right("sidebar")
    .default_width(300.0)
    .resizable(true)
    .show(contexts.ctx_mut(), |ui| {
      let height = ui.available_rect_before_wrap().height() / 3.0 - 10.0;

      let mut selected_entity: Option<Entity> = None;
      let selected = if selected.is_empty() {
        None
      } else {
        let selected = selected.single_mut();
        selected_entity.replace(selected.0.entity);
        Some(selected)
      };

      ui.heading(RichText::new("Characters").strong().size(16.0));
      ScrollArea::vertical()
        .id_source("characters")
        .auto_shrink([false; 2])
        .max_height(height)
        .show(ui, |ui| {
          characters_ui(&mut commands, ui, characters, selected_entity);
        });

      if selected.is_none() {
        return;
      }

      let (character, history, current_thirst) = selected.unwrap();
      let character_name = character.get_name();

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
          actions_ui(ui, history);
        });

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
          states_ui(ui, current_thirst);
        });
    });
}

fn characters_ui(
  commands: &mut Commands,
  ui: &mut Ui,
  query: Query<&Character>,
  selected: Option<Entity>,
) {
  let mut characters = BTreeMap::<String, &Character>::new();
  for character in query.iter() {
    characters.insert(
      format!(
        "{} ({})",
        character.get_name(),
        character.gender.to_string()
      ),
      &character,
    );
  }

  for (name, character) in characters.iter() {
    let is_selected = match selected {
      None => false,
      Some(selected) => selected == character.entity,
    };
    if selectable_label(ui, name.clone(), is_selected).clicked() {
      let mut entity = commands.entity(character.entity);
      if is_selected {
        entity.remove::<Selected>();
      } else {
        entity.insert(Selected);
        if selected.is_some() {
          commands.entity(selected.unwrap()).remove::<Selected>();
        }
      }
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
            ui.label(item.status.to_string());
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
