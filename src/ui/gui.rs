use std::io::{ErrorKind};
use std::path::Path;
use std::time::Instant;
use iced::{Element, Sandbox, Theme};
use iced::widget::{Button, Checkbox, Container, horizontal_rule, horizontal_space, row, column, text_input, vertical_space, vertical_rule};
use crate::parser;
use crate::settings::Settings;

pub struct TeamTotalsGui {
    competition: String,
    settings: Settings,
    theme: Theme,
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

fn calculate(competition: String, settings: Settings) {
    let path = match get_directory(competition, &settings) {
        Ok(path) => path,
        Err(_) => return, // Should send signal to the user that no path is found.
    };

    let elapsed = Instant::now();

    parser::parse_results(path, settings);

    println!("Calculation finished in {} milliseconds", elapsed.elapsed().as_millis());
}

impl Sandbox for TeamTotalsGui {
    type Message = TeamTotalsMessage;

    fn new() -> Self {
        let settings = Settings::read();
        TeamTotalsGui {
            competition: String::new(),
            settings,
            theme: Theme::Dark,
        }
    }

    fn title(&self) -> String {
        String::from("Team Totals Calculator")
    }

    fn update(&mut self, message: Self::Message) {
        let mut settings_changed = false;
        match message {
            TeamTotalsMessage::Input(input) => self.competition = input,
            TeamTotalsMessage::Send => { calculate(self.competition.clone(), self.settings.clone()) }
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


        //let column1 = Column::new().push(competition_input).push(vertical_space(10)).push(calculate_button).padding(10);
        let column1 = column![
            competition_input,
            vertical_space(10),
            calculate_button
        ].padding(10);

        let include_60_checkbox = Checkbox::new("Include 6.0", self.settings.include_60, TeamTotalsMessage::Include60);
        let include_ijs_checkbox = Checkbox::new("Include IJS", self.settings.include_ijs, TeamTotalsMessage::IncludeIJS);
        let generate_xslx_checkbox = Checkbox::new("Generate .xlsx file", self.settings.generate_xlsx, TeamTotalsMessage::GenerateXLSX);
        let generate_txt_checkbox = Checkbox::new("Generate .txt file", self.settings.generate_txt, TeamTotalsMessage::GenerateTXT);
        let attempt_60_club_correction_checkbox = Checkbox::new("Attempt 6.0 Club Correction", self.settings.attempt_automatic_60_club_name_recombination, TeamTotalsMessage::Attempt60ClubCorrection);
        let use_event_name_checkbox = Checkbox::new("Use Event Name For Results Path", self.settings.use_event_name_for_results_path, TeamTotalsMessage::UseEventNameForResultsPath);

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
            horizontal_rule(1),
        ].padding(10).width(250);

        let row = row!
        [
            column2,
            horizontal_space(5),
            vertical_rule(1),
            horizontal_space(5),
            column1
        ];

        Container::new(row).center_x().center_y().width(iced::Length::Fill).height(iced::Length::Fill).into()
    }

    fn theme(&self) -> Theme {
        self.theme.clone()
    }
}