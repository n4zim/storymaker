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

use crate::time::clock::GameClock;
use bevy::prelude::*;
use bevy_egui::{
  egui::{self, Align2, FontId, RichText, Vec2},
  EguiContexts,
};

pub fn system(mut contexts: EguiContexts, clock: Res<GameClock>) {
  egui::Window::new("clock")
    .title_bar(false)
    .resizable(false)
    .collapsible(false)
    .anchor(Align2::LEFT_BOTTOM, Vec2::new(30.0, -30.0))
    .show(contexts.ctx_mut(), |ui| {
      ui.label(
        RichText::new(clock.to_string()).font(FontId::proportional(40.0)),
      );
    });
}
