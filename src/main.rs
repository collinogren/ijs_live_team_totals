/*
Copyright (c) 2023 Collin Ogren

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
*/

//#![windows_subsystem = "windows"]

use crate::gui::TeamTotalsGui;

#[path = "io/html/parser.rs"]
mod parser;

#[path = "settings/settings.rs"]
mod settings;

#[path = "ui/terminal_ui.rs"]
mod terminal_ui;

#[path = "io/excel/xlsx_writer.rs"]
mod xlsx_writer;

#[path = "io/html/results_sorter.rs"]
mod results_sorter;

#[path = "io/html/html_writer.rs"]
mod html_writer;

#[path = "ui/gui.rs"]
mod gui;

#[path = "ui/points_field.rs"]
mod points_field;

#[path = "ui/image_loader.rs"]
mod image_loader;

#[path = "ui/timer.rs"]
mod timer;

#[path = "io/file_utils.rs"]
mod file_utils;

fn main() -> Result<(), iced::Error> {
    iced::application(TeamTotalsGui::title, TeamTotalsGui::update, TeamTotalsGui::view)
        .subscription(TeamTotalsGui::subscription)
        .theme(TeamTotalsGui::theme)
        .run_with(TeamTotalsGui::new)
}
