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

use std::io::{ErrorKind};
use std::path::Path;
use std::time::Instant;
use iced::{Element, Sandbox, Theme};
use iced::alignment::Vertical;
use iced::widget::{Button, Checkbox, Container, horizontal_space, row, column, text_input, vertical_space, vertical_rule, text, Scrollable};
use crate::gui::TeamTotalsMessage::{HTMLRelativeDirectory, ISUCalcBaseDirectory, OutputDirectory, TXTFileName, XLSXFileName, XLSXFontSize};
use crate::parser;
use crate::parser::State;
use crate::settings::Settings;

pub struct TeamTotalsGui {
    competition: String,
    settings: Settings,
    status: String,
    theme: Theme,

    font_size: String,
}

#[derive(Debug, Clone)]
pub enum TeamTotalsMessage {
    Send,
    Input(String),
    Include60(bool),
    IncludeIJS(bool),
    Attempt60ClubCorrection(bool),
    UseEventNameForResultsPath(bool),
    GenerateXLSX(bool),
    GenerateTXT(bool),
    XLSXFontSize(String),
    ISUCalcBaseDirectory(String),
    HTMLRelativeDirectory(String),
    XLSXFileName(String),
    TXTFileName(String),
    OutputDirectory(String),
}

fn get_directory(input: String, settings: &Settings) -> Result<String, ErrorKind> {
    let result;
    if settings.use_event_name_for_results_path {
        println!("Enter the name of the competition you wish to talley");

        let path_string = format!("{}{}{}", settings.isu_calc_base_directory, input, settings.html_relative_directory).replace("\r", "").replace("\n", "");
        let path = Path::new(path_string.as_str());

        println!("{}", path_string);

        if path.is_dir() {
            result = Ok(path_string);
        } else {
            println!("Could not find a competition with name \"{}\", check the competition's name and try again.", input);
            result = Err(ErrorKind::NotFound)
        }

        result
    } else {
        let path_string = String::from(input).replace("\r", "").replace("\n", "");
        let path = Path::new(path_string.as_str());

        if path.is_dir() {
            result = Ok(path_string);
        } else {
            println!("No such file or directory: \"{}\", check the competition's path and try again.", path_string);
            result = Err(ErrorKind::NotFound);
        }

        result
    }
}

fn calculate(competition: String, settings: Settings) -> String {
    let path = match get_directory(competition, &settings) {
        Ok(path) => path,
        Err(_) => return String::from("No competition found."), // Should send signal to the user that no path is found.
    };

    let elapsed = Instant::now();

    let (output, state) = parser::parse_results(path, settings);

    let time_past = elapsed.elapsed().as_millis();

    match state {
        State::Ok => format!("{}\nCalculation finished in {} milliseconds.", output, time_past),
        State::Error => output
    }
}

impl Sandbox for TeamTotalsGui {
    type Message = TeamTotalsMessage;

    fn new() -> Self {
        let settings = Settings::read();
        TeamTotalsGui {
            competition: String::new(),
            settings: settings.clone(),
            status: String::new(),
            theme: Theme::Dark,
            font_size: settings.xlsx_font_size.to_string(),
        }
    }

    fn title(&self) -> String {
        String::from("Team Totals Calculator")
    }

    fn update(&mut self, message: Self::Message) {
        let mut settings_changed = false;
        match message {
            TeamTotalsMessage::Input(input) => self.competition = input,
            TeamTotalsMessage::Send => { self.status = calculate(self.competition.clone(), self.settings.clone()) }
            TeamTotalsMessage::Include60(include_60) => {
                self.settings.include_60 = include_60;
                settings_changed = true;
            }
            TeamTotalsMessage::IncludeIJS(include_ijs) => {
                self.settings.include_ijs = include_ijs;
                settings_changed = true;
            }
            TeamTotalsMessage::Attempt60ClubCorrection(attempt_60_club_correction) => {
                self.settings.attempt_automatic_60_club_name_recombination = attempt_60_club_correction;
                settings_changed = true;
            }
            TeamTotalsMessage::UseEventNameForResultsPath(use_event_name_for_results_path) => {
                self.settings.use_event_name_for_results_path = use_event_name_for_results_path;
                settings_changed = true;
            }
            TeamTotalsMessage::GenerateXLSX(generate_xlsx) => {
                self.settings.generate_xlsx = generate_xlsx;
                settings_changed = true;
            }
            TeamTotalsMessage::GenerateTXT(generate_txt) => {
                self.settings.generate_txt = generate_txt;
                settings_changed = true;
            }
            XLSXFontSize(font_size) => {
                match font_size.parse::<u32>() {
                    Ok(value) => {
                        self.font_size = font_size;
                        self.settings.xlsx_font_size = value;
                        settings_changed = true;
                    }
                    Err(_) => {}
                }
            }
            ISUCalcBaseDirectory(isu_calc_base_directory) => {
                self.settings.isu_calc_base_directory = isu_calc_base_directory;
                settings_changed = true;
            }
            HTMLRelativeDirectory(html_relative_directory) => {
                self.settings.html_relative_directory = html_relative_directory;
                settings_changed = true;
            }
            XLSXFileName(xlsx_file_name) => {
                self.settings.xlsx_file_name = xlsx_file_name;
                settings_changed = true;
            }
            TXTFileName(txt_file_name) => {
                self.settings.txt_file_name = txt_file_name;
                settings_changed = true;
            }
            OutputDirectory(output_directory) => {
                self.settings.output_directory = output_directory;
                settings_changed = true;
            }
        }

        if settings_changed {
            self.settings.write();
        }
    }

    fn view(&self) -> Element<'_, Self::Message> {
        let competition_input = text_input(if self.settings.use_event_name_for_results_path {
            "Enter the name of the competition you wish to talley"
        } else {
            "Enter the absolute path of the competition you wish to talley"
        }, &self.competition).on_submit(TeamTotalsMessage::Send).on_input(TeamTotalsMessage::Input);

        let calculate_button = Button::new("Calculate Team Totals").on_press(TeamTotalsMessage::Send).width(200);

        let calculate_button_row = row![calculate_button, horizontal_space(10), text(&self.status)];
        let column1 = column![
            competition_input,
            vertical_space(10),
            calculate_button_row,
        ].padding(10).width(iced::Length::FillPortion(5));

        let include_60_checkbox = Checkbox::new("Include 6.0", self.settings.include_60, TeamTotalsMessage::Include60);
        let include_ijs_checkbox = Checkbox::new("Include IJS", self.settings.include_ijs, TeamTotalsMessage::IncludeIJS);
        let generate_xslx_checkbox = Checkbox::new("Generate .xlsx file", self.settings.generate_xlsx, TeamTotalsMessage::GenerateXLSX);
        let generate_txt_checkbox = Checkbox::new("Generate .txt file", self.settings.generate_txt, TeamTotalsMessage::GenerateTXT);
        let attempt_60_club_correction_checkbox = Checkbox::new("Attempt 6.0 Club Correction", self.settings.attempt_automatic_60_club_name_recombination, TeamTotalsMessage::Attempt60ClubCorrection);
        let use_event_name_checkbox = Checkbox::new("Use Event Name For Results Path", self.settings.use_event_name_for_results_path, TeamTotalsMessage::UseEventNameForResultsPath);

        let font_size = text_input("", &self.font_size).on_input(XLSXFontSize);
        let font_size_column = column![text("Font Size"), font_size];

        let isu_calc_base_directory = text_input("", &self.settings.isu_calc_base_directory).on_input(ISUCalcBaseDirectory);
        let isu_calc_base_directory_column = column![text("ISUCalcFS Base Directory"), vertical_space(1), isu_calc_base_directory];

        let html_relative_directory = text_input("", &self.settings.html_relative_directory).on_input(HTMLRelativeDirectory);
        let html_relative_directory_column = column![text("HTML Relative Directory"), vertical_space(1), html_relative_directory];

        let output_directory = text_input("", &self.settings.output_directory).on_input(OutputDirectory);
        let output_directory_column = column![text("Output Directory"), vertical_space(1), output_directory];

        let xlsx_file_name = text_input("", &self.settings.xlsx_file_name).on_input(XLSXFileName);
        let xlsx_file_name_column = column![text("Excel File Name"), vertical_space(1), xlsx_file_name];

        let txt_file_name = text_input("", &self.settings.txt_file_name).on_input(TXTFileName);
        let txt_file_name_column = column![text("Plain Text File Name"), vertical_space(1), txt_file_name];

        let column2 = column!
        [
            include_60_checkbox,
            vertical_space(10),
            include_ijs_checkbox,
            vertical_space(10),
            generate_xslx_checkbox,
            vertical_space(10),
            generate_txt_checkbox,
            vertical_space(10),
            attempt_60_club_correction_checkbox,
            vertical_space(10),
            use_event_name_checkbox,
            vertical_space(10),
            font_size_column,
            vertical_space(10),
            isu_calc_base_directory_column,
            vertical_space(10),
            html_relative_directory_column,
            vertical_space(10),
            output_directory_column,
            vertical_space(10),
            xlsx_file_name_column,
            vertical_space(10),
            txt_file_name_column,

        ].padding(10);

        let scroll_pane = Scrollable::new(column2).width(iced::Length::FillPortion(2));

        let row = row!
        [
            scroll_pane,
            horizontal_space(5),
            vertical_rule(1),
            horizontal_space(5),
            column1
        ].height(iced::Length::Fill);


        Container::new(row).center_x().align_y(Vertical::Top).width(iced::Length::Fill).height(iced::Length::Fill).into()
    }

    fn theme(&self) -> Theme {
        self.theme.clone()
    }
}