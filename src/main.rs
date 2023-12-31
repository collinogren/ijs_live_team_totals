#![feature(fs_try_exists)]
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

#![windows_subsystem = "windows"]

use iced::Application;
use crate::gui::TeamTotalsGui;

#[path = "html_utilities/parser.rs"]
mod parser;

#[path = "settings/settings.rs"]
mod settings;

#[path = "ui/terminal_ui.rs"]
mod terminal_ui;

#[path = "excel/xlsx_writer.rs"]
mod xlsx_writer;

#[path = "html_utilities/results_sorter.rs"]
mod results_sorter;

#[path = "ui/gui.rs"]
mod gui;

#[path = "ui/points_field.rs"]
mod points_field;

#[path = "ui/image_loader.rs"]
mod image_loader;

#[path = "ui/timer.rs"]
mod timer;

fn main() -> Result<(), iced::Error> {
    let mut settings = iced::Settings::default();
    settings.window.size = (1000, 800);
    TeamTotalsGui::run(settings)
}
