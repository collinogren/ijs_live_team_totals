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

use std::cell::RefCell;
use std::io::{ErrorKind};
use std::path::{Path, PathBuf};
use std::sync::mpsc;
use std::sync::mpsc::{Receiver, Sender};
use std::{fs, thread, vec};
use iced::{Alignment, Application, Command, Element, keyboard, Subscription, Theme, widget, window};
use iced::alignment::Vertical;
use iced::widget::{Button, Checkbox, horizontal_space, row, column, text_input, vertical_space, vertical_rule, text, Scrollable, keyed_column, container, scrollable, checkbox};
use iced::widget::scrollable::{RelativeOffset};
use iced::window::icon;
use native_dialog::FileDialog;
use once_cell::sync::Lazy;
use crate::gui::TeamTotalsMessage::{HTMLRelativeDirectory, ISUCalcBaseDirectory, OutputDirectory, TXTFileName, XLSXFileName, XLSXFontSize};
use crate::image_loader::png_to_rgba;
use crate::parser;
use crate::parser::{Event, State};
use crate::settings::{appdata, Settings};
use crate::timer::{Timer};

static COMPETITION_INPUT_ID: Lazy<text_input::Id> = Lazy::new(competition_input_id);
static SCROLLABLE_ID: Lazy<scrollable::Id> = Lazy::new(scrollable::Id::unique);

fn competition_input_id() -> text_input::Id {
    text_input::Id::new(String::from("competition_input"))
}

pub struct TeamTotalsGui {
    competition: String,
    settings: Settings,
    status: String,
    theme: Theme,
    font_size: String,
    points_for_each_placement: Vec<PointsField>,
    competition_directory_receiver: RefCell<Option<Receiver<PathBuf>>>,
    competition_directory_sender: Sender<PathBuf>,

    output_directory_receiver: RefCell<Option<Receiver<PathBuf>>>,
    output_directory_sender: Sender<PathBuf>,

    event_names_receiver: RefCell<Option<Receiver<(Vec<Event>, String)>>>,
    event_names_sender: Sender<(Vec<Event>, String)>,

    results_receiver: RefCell<Option<Receiver<String>>>,
    results_sender: Sender<String>,

    events: Vec<Event>,
    event_controls: Vec<EventsControls>,
    last_checkbox: isize,
    is_shift_down: bool,

    status_timer: Option<Timer>,
}

#[derive(Debug, Clone)]
pub enum TeamTotalsMessage {
    RetrieveResults,
    CalculateResults,
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
    PointsForEachPlacement(usize, PointsForEachPlacement),
    EventInclusionChanged(usize, EventToInclude),
    EventsRetrieved(Vec<Event>, String),
    ResultsRetrieved(String),

    TabPressed {shift: bool},
    FindReceived(PathBuf),
    Find,
    FindOutputDirectoryReceived(PathBuf),
    FindOutputDirectory,
    OpenInFileViewer,

    StatusTimerDone,
    StatusTimerError,

    AddPlacement,
    RemovePlacement,
    ShiftPressed,
    ShiftReleased,
}

fn get_directory(input: String, settings: &Settings) -> Result<String, ErrorKind> {
    let result;
    if settings.use_event_name_for_results_path {

        let isu_calc_base_directory = if !settings.isu_calc_base_directory.ends_with("/") && !settings.isu_calc_base_directory.ends_with("\\") {
            format!("{}{}", settings.isu_calc_base_directory, "/")
        } else {
            settings.isu_calc_base_directory.clone()
        };

        let html_relative_directory = if !settings.html_relative_directory.starts_with("/") && !settings.html_relative_directory.starts_with("\\") {
            format!("{}{}", "/", settings.html_relative_directory)
        } else {
            settings.html_relative_directory.clone()
        };

        let path_string = format!("{}{}{}", isu_calc_base_directory, input, html_relative_directory).replace("\r", "").replace("\n", "");
        let path = Path::new(path_string.as_str());

        if path.is_dir() {
            result = Ok(path_string);
        } else {
            result = Err(ErrorKind::NotFound)
        }

        result
    } else {
        let path_string = format!("{}{}", String::from(input).replace("\r", "").replace("\n", ""), settings.html_relative_directory);
        let path = Path::new(path_string.as_str());

        if path.is_dir() {
            result = Ok(path_string);
        } else {
            result = Err(ErrorKind::NotFound);
        }

        result
    }
}

fn retrieve_events(competition: String, settings: Settings) -> (Vec<Event>, String) {
    let path = match get_directory(competition, &settings) {
        Ok(path) => path,
        Err(_) => return (vec![], String::from("No competition found.")), // Should send signal to the user that no path is found.
    };

    let (events, output, state) = parser::retrieve_events(path);

    match state {
        State::Ok => (events, output),
        State::Error => (events, output)
    }
}

fn calculate(events: Vec<Event>, settings: &Settings) -> String {
    let result = parser::parse_results(events, settings);
    String::from(result.0)
}

impl TeamTotalsGui {
    fn start_status_timer(&mut self) {
        match &mut self.status_timer {
            None => {
                self.status_timer = Some(Timer::new(5000, 1));
            }
            Some(timer) => {
                timer.stop();
                self.status_timer = Some(Timer::new(5000, 1));
            }
        }


        match &mut self.status_timer {
            None => {}
            Some(ref mut v) => {
                v.start();
            }
        }
    }

    fn apply_points_for_each_placement(&mut self, value: String, index: usize) {
        match str::parse::<f64>(value.as_str()) {
            Ok(_) => {
                self.points_for_each_placement[index] = PointsField::new(index, value);
            }
            Err(_) => {
                if value == "" {
                    self.points_for_each_placement[index] = PointsField::new(index, value);
                } else {
                    return;
                }
            }
        };

        self.synchronize_settings_with_gui();
    }

    fn synchronize_settings_with_gui(&mut self) {
        self.settings.points_for_each_placement.clear();

        for points in &self.points_for_each_placement {
            self.settings.points_for_each_placement.push(match str::parse::<f64>(points.value.as_str()) {
                Ok(value) => value,
                Err(_) => 0.0,
            });
        }
    }

    fn synchronize_gui_with_settings(&mut self) {
        self.points_for_each_placement.clear();

        for (i, points) in self.settings.points_for_each_placement.iter().enumerate() {
            self.points_for_each_placement.push(PointsField::new(i, format!("{}", points)));
        }
    }
}

impl Application for TeamTotalsGui {
    type Executor = iced::executor::Default;
    type Message = TeamTotalsMessage;
    type Theme = Theme;
    type Flags = ();

    fn new(_flags: Self::Flags) -> (Self, Command<Self::Message>) {
        let settings = Settings::read();

        let (path_sender, path_receiver) = mpsc::channel::<PathBuf>();
        let (output_path_sender, output_path_receiver) = mpsc::channel::<PathBuf>();
        let (event_names_sender, event_names_receiver) = mpsc::channel::<(Vec<Event>, String)>();
        let (results_sender, results_receiver) = mpsc::channel::<String>();

        let mut gui = TeamTotalsGui {
            competition: String::new(),
            settings: settings.clone(),
            status: String::new(),
            theme: Theme::Dark,
            font_size: settings.xlsx_font_size.to_string(),
            points_for_each_placement: vec![],
            competition_directory_sender: path_sender,
            output_directory_sender: output_path_sender,
            output_directory_receiver: RefCell::new(Some(output_path_receiver)),
            competition_directory_receiver: RefCell::new(Some(path_receiver)),
            event_names_receiver: RefCell::new(Some(event_names_receiver)),
            event_names_sender,
            results_sender,
            results_receiver: RefCell::new(Some(results_receiver)),
            events: vec![],
            event_controls: vec![],
            last_checkbox: -1,
            is_shift_down: false,
            status_timer: None,
        };

        let mut commands = vec![];

        let icon_bytes = include_bytes!("icon.png");
        let binding = appdata("/assets");
        let binding = binding.0.as_str();
        let icon_path = Path::new(binding);
        if !icon_path.exists() {
            fs::create_dir_all(icon_path).unwrap();
        }

        let binding = appdata("/assets/icon.png");
        let binding = binding.0.as_str();
        let icon_path = Path::new(binding);

        if !icon_path.exists() {
            fs::write(icon_path, icon_bytes).unwrap();
        }

        let (raw_icon, width, height) = png_to_rgba(icon_path.to_str().unwrap());
        let icon = window::change_icon(icon::from_rgba(raw_icon, width, height).unwrap());

        commands.push(icon);

        commands.push(text_input::focus(COMPETITION_INPUT_ID.clone()));

        gui.synchronize_gui_with_settings();

        (gui, Command::batch(commands))
    }

    fn title(&self) -> String {
        String::from("Team Totals Calculator")
    }

    fn update(&mut self, message: Self::Message) -> Command<TeamTotalsMessage> {
        let mut commands = vec![];
        let mut settings_changed = false;

        let command = match message {
            TeamTotalsMessage::Input(input) => {
                self.competition = input;
                Command::none()
            },
            TeamTotalsMessage::RetrieveResults => {
                let sender = self.event_names_sender.clone();

                let competition = self.competition.clone();
                let settings = self.settings.clone();

                thread::spawn(move || {
                    let ret = retrieve_events(competition, settings);
                    sender.send(ret).unwrap();
                });

                Command::none()
            }
            TeamTotalsMessage::EventsRetrieved(events, status) => {
                self.events = events;
                self.event_controls = self.events.iter().enumerate().map(|(i, event)| {
                    EventsControls::new(i, event.clone())
                }).collect::<Vec<EventsControls>>();
                self.status = status;
                //self.start_status_timer();

                Command::none()
            }
            TeamTotalsMessage::Include60(include_60) => {
                self.settings.include_60 = include_60;
                settings_changed = true;
                Command::none()
            }
            TeamTotalsMessage::IncludeIJS(include_ijs) => {
                self.settings.include_ijs = include_ijs;
                settings_changed = true;
                Command::none()
            }
            TeamTotalsMessage::Attempt60ClubCorrection(attempt_60_club_correction) => {
                self.settings.attempt_automatic_60_club_name_recombination = attempt_60_club_correction;
                settings_changed = true;
                Command::none()
            }
            TeamTotalsMessage::UseEventNameForResultsPath(use_event_name_for_results_path) => {
                self.settings.use_event_name_for_results_path = use_event_name_for_results_path;
                settings_changed = true;
                Command::none()
            }
            TeamTotalsMessage::GenerateXLSX(generate_xlsx) => {
                self.settings.generate_xlsx = generate_xlsx;
                settings_changed = true;
                Command::none()
            }
            TeamTotalsMessage::GenerateTXT(generate_txt) => {
                self.settings.generate_txt = generate_txt;
                settings_changed = true;
                Command::none()
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

                Command::none()
            }
            ISUCalcBaseDirectory(isu_calc_base_directory) => {
                self.settings.isu_calc_base_directory = isu_calc_base_directory;
                settings_changed = true;
                Command::none()
            }
            HTMLRelativeDirectory(html_relative_directory) => {
                self.settings.html_relative_directory = html_relative_directory;
                settings_changed = true;
                Command::none()
            }
            XLSXFileName(xlsx_file_name) => {
                self.settings.xlsx_file_name = xlsx_file_name;
                settings_changed = true;
                Command::none()
            }
            TXTFileName(txt_file_name) => {
                self.settings.txt_file_name = txt_file_name;
                settings_changed = true;
                Command::none()
            }
            OutputDirectory(output_directory) => {
                self.settings.output_directory = output_directory;
                settings_changed = true;
                Command::none()
            }
            TeamTotalsMessage::AddPlacement => {
                self.points_for_each_placement.push(PointsField::new(self.points_for_each_placement.len(), String::from("0")));
                self.synchronize_settings_with_gui();
                settings_changed = true;
                scrollable::snap_to(SCROLLABLE_ID.clone(), RelativeOffset::END)
            }
            TeamTotalsMessage::RemovePlacement => {
                self.points_for_each_placement.remove(self.points_for_each_placement.len() - 1);
                self.synchronize_settings_with_gui();
                settings_changed = true;
                Command::none()
            }
            TeamTotalsMessage::PointsForEachPlacement(i, value) => {
                match value {
                    PointsForEachPlacement::Edited(value) => {
                        self.apply_points_for_each_placement(value, i);
                        settings_changed = true;
                    }
                }

                Command::none()
            }
            TeamTotalsMessage::TabPressed { shift } => {
                if shift {
                    widget::focus_previous()
                } else {
                    widget::focus_next()
                }
            }
            TeamTotalsMessage::Find => {
                let sender = self.competition_directory_sender.clone();

                let base_directory = self.settings.isu_calc_base_directory.clone();

                thread::spawn(move || {
                    let directory = FileDialog::new()
                        .set_location(base_directory.as_str())
                        .show_open_single_dir()
                        .unwrap().unwrap();

                        sender.send(directory).unwrap();
                });

                Command::none()
            }
            TeamTotalsMessage::FindReceived(directory) => {
                if self.settings.use_event_name_for_results_path {
                    self.competition = directory.file_name().unwrap().to_str().unwrap_or("").to_string();
                } else {
                    self.competition = directory.to_str().unwrap_or("").to_string();
                }

                Command::none()
            }

            TeamTotalsMessage::EventInclusionChanged(i, event) => {
                match event {
                    EventToInclude::Edited(b) => {
                        if self.is_shift_down && self.last_checkbox > -1 {
                            for x in if self.last_checkbox < (i + 1) as isize {
                                self.last_checkbox..(i + 1) as isize
                            } else {
                                i as isize.. self.last_checkbox + 1
                            } {
                                let x = x as usize;
                                self.event_controls[x].event.active = b;
                                self.events[x] = self.event_controls[x].event.clone();
                            }
                        } else {
                            self.event_controls[i].event.active = b;
                            self.events[i] = self.event_controls[i].event.clone();
                        }

                        self.last_checkbox = i as isize;
                    }
                }

                Command::none()
            }
            TeamTotalsMessage::CalculateResults => {
                let sender = self.results_sender.clone();

                let events = self.events.clone();
                let settings = self.settings.clone();

                thread::spawn(move || {
                    sender.send(calculate(events, &settings)).unwrap();
                });

                Command::none()
            }

            TeamTotalsMessage::ResultsRetrieved(status) => {
                self.status = status;
                //self.start_status_timer();

                Command::none()
            }

            TeamTotalsMessage::ShiftPressed => {
                self.is_shift_down = true;

                Command::none()
            }
            TeamTotalsMessage::ShiftReleased => {
                self.is_shift_down = false;

                Command::none()
            }
            TeamTotalsMessage::FindOutputDirectory => {
                let sender = self.output_directory_sender.clone();

                let output_directory = self.settings.output_directory.clone();

                thread::spawn(move || {
                    let directory = FileDialog::new()
                        .set_location(output_directory.as_str())
                        .show_open_single_dir()
                        .unwrap().unwrap();

                    sender.send(directory).unwrap();
                });

                Command::none()
            }
            TeamTotalsMessage::FindOutputDirectoryReceived(directory) => {
                self.settings.output_directory = directory.to_str().unwrap_or("").to_string().replace("\\", "/");
                settings_changed = true;

                Command::none()
            }
            TeamTotalsMessage::OpenInFileViewer => {
                println!("{}", &self.settings.output_directory);
                let directory = self.settings.output_directory.clone();
                thread::spawn(move || {
                    match open::that(&directory) {
                        Ok(_) => {}
                        Err(_) => {}
                    }
                });

                Command::none()
            }
            TeamTotalsMessage::StatusTimerDone => {
                self.status = String::new();
                self.status_timer = None;

                Command::none()
            }
            TeamTotalsMessage::StatusTimerError => {
                Command::none()
            }
        };

        if settings_changed {
            self.settings.write();
        }

        commands.push(command);

        Command::batch(commands)
    }

    fn view(&self) -> Element<TeamTotalsMessage> {
        let competition_input = text_input(if self.settings.use_event_name_for_results_path {
            "Enter the name of the competition you wish to talley"
        } else {
            "Enter the absolute path of the competition you wish to talley"
        }, &self.competition).on_submit(TeamTotalsMessage::RetrieveResults).on_input(TeamTotalsMessage::Input).id(COMPETITION_INPUT_ID.clone());

        let find_button = Button::new("...").on_press(TeamTotalsMessage::Find).width(20);

        let competition_input_row = row!(competition_input, find_button).width(iced::Length::Fill);

        let retrieve_data_button = Button::new("Retrieve Data").on_press(TeamTotalsMessage::RetrieveResults).width(140);

        let mut can_calculate = false;

        for event in &self.events {
            if event.active {
                can_calculate = true;
                break;
            }
        }

        let calculate_button = if can_calculate {
            Button::new("Tabulate Results").on_press(TeamTotalsMessage::CalculateResults).width(140)
        } else {
            Button::new("Tabulate Results").width(140)
        };

        let calculate_button_row = row![retrieve_data_button, horizontal_space(10), calculate_button, horizontal_space(10), text(&self.status)].align_items(Alignment::Center);

        let loaded_events_column: Element<_> =
            keyed_column(
                self.event_controls
                    .iter()
                    .enumerate()
                    .map(|(i, event)| {
                        (
                            event.index,
                            event.view().map(move |message| {
                                TeamTotalsMessage::EventInclusionChanged(i, message)
                            }),
                        )
                    }),
            )
                .spacing(10)
                .into();

        let loaded_events_scrollable = scrollable(loaded_events_column).width(iced::Length::Fill);

        let open_output_directory_button = Button::new("Open Output Directory in File Viewer...").on_press(TeamTotalsMessage::OpenInFileViewer).width(290);
        let open_output_directory_row = row!(open_output_directory_button);

        let column1 = column![
            competition_input_row,
            vertical_space(10),
            calculate_button_row,
            vertical_space(10),
            open_output_directory_row,
            vertical_space(10),
            iced::widget::horizontal_rule(1),
            vertical_space(10),
            loaded_events_scrollable
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
        let output_directory_button = Button::new("...").on_press(TeamTotalsMessage::FindOutputDirectory).width(20);
        let output_directory_column = column![text("Output Directory"), vertical_space(1), row![output_directory, output_directory_button], vertical_space(5)];

        let xlsx_file_name = text_input("", &self.settings.xlsx_file_name).on_input(XLSXFileName);
        let xlsx_file_name_column = column![text("Excel File Name"), vertical_space(1), xlsx_file_name];

        let txt_file_name = text_input("", &self.settings.txt_file_name).on_input(TXTFileName);
        let txt_file_name_column = column![text("Plain Text File Name"), vertical_space(1), txt_file_name];

        let mut column2: widget::Column<'_, TeamTotalsMessage> = column![
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
            vertical_space(10),
        ].padding(10);

        column2 = column2.push(text("Points For Each Placement"));

        let points_for_each_placement: Element<_> =
            keyed_column(
                self.points_for_each_placement
                    .iter()
                    .enumerate()
                    .map(|(i, placements)| {
                        (
                            placements.index,
                            placements.view(i).map(move |message| {
                                TeamTotalsMessage::PointsForEachPlacement(i, message)
                            }),
                        )
                    }),
            )
                .spacing(10)
                .into();

        column2 = column2.push(points_for_each_placement).push(vertical_space(10));

        let remove_placement = Button::new("Remove").on_press(TeamTotalsMessage::RemovePlacement).width(iced::Length::Fill);
        let add_placement = Button::new("Add").on_press(TeamTotalsMessage::AddPlacement).width(iced::Length::Fill);
        let add_remove_placement_row = row![horizontal_space(25), remove_placement, horizontal_space(10), add_placement];

        column2 = column2.push(add_remove_placement_row);

        let scroll_pane = Scrollable::new(column2).width(iced::Length::FillPortion(3)).id(SCROLLABLE_ID.clone());

        let row = row![
            scroll_pane,
            horizontal_space(5),
            vertical_rule(1),
            column1
        ].height(iced::Length::Fill);

        container(row).center_x().align_y(Vertical::Top).width(iced::Length::Fill).height(iced::Length::Fill).into()
    }

    fn theme(&self) -> Theme {
        self.theme.clone()
    }

    fn subscription(&self) -> Subscription<TeamTotalsMessage> {
        let mut subscriptions = vec![];
        let tab = keyboard::on_key_press(|key_code, modifiers| {
            match (key_code, modifiers) {
                (keyboard::KeyCode::LShift, _) => Some(TeamTotalsMessage::ShiftPressed),
                (keyboard::KeyCode::RShift, _) => Some(TeamTotalsMessage::ShiftPressed),
                (keyboard::KeyCode::Tab, _) => Some(TeamTotalsMessage::TabPressed {
                    shift: modifiers.shift(),
                }),
                _ => None,
            }
        });

        subscriptions.push(tab);

        let shift_down = keyboard::on_key_press(|key_code, modifiers| {
            match (key_code, modifiers) {
                (keyboard::KeyCode::LShift, _) => Some(TeamTotalsMessage::ShiftPressed),
                (keyboard::KeyCode::RShift, _) => Some(TeamTotalsMessage::ShiftPressed),
                _ => None,
            }
        });

        let shift_up = keyboard::on_key_release(|key_code, modifiers| {
            match (key_code, modifiers) {
                (keyboard::KeyCode::LShift, _) => Some(TeamTotalsMessage::ShiftReleased),
                (keyboard::KeyCode::RShift, _) => Some(TeamTotalsMessage::ShiftReleased),
                _ => None,
            }
        });

        subscriptions.push(shift_down);
        subscriptions.push(shift_up);

        let competition_directory = iced::subscription::unfold(
            "competition_name_subscription",
            self.competition_directory_receiver.take(),
            move |mut receiver| async move {
                let directory = match receiver.as_mut().unwrap().recv() {
                    Ok(directory) => directory,
                    Err(_) => PathBuf::new(),
                };
                (TeamTotalsMessage::FindReceived(directory), receiver)
            },
        );
        subscriptions.push(competition_directory);

        let output_directory = iced::subscription::unfold(
            "output_directory_subscription",
            self.output_directory_receiver.take(),
            move |mut receiver| async move {
                let directory = match receiver.as_mut().unwrap().recv() {
                    Ok(directory) => directory,
                    Err(_) => PathBuf::new(),
                };
                (TeamTotalsMessage::FindOutputDirectoryReceived(directory), receiver)
            },
        );
        subscriptions.push(output_directory);

        let retrieve_events = iced::subscription::unfold(
            "retrieve_events",
            self.event_names_receiver.take(),
            move |mut receiver| async move {
                let (events, status) = match receiver.as_mut().unwrap().recv() {
                    Ok(v) => v,
                    Err(_) => (vec![], String::from("Failed to retrieve events.")),
                };
                (TeamTotalsMessage::EventsRetrieved(events, status), receiver)
            },
        );
        subscriptions.push(retrieve_events);

        let retrieve_results = iced::subscription::unfold(
            "retrieve_results",
            self.results_receiver.take(),
            move |mut receiver| async move {
                let status = match receiver.as_mut().unwrap().recv() {
                    Ok(v) => v,
                    Err(_) => String::from("Failed to retrieve status."),
                };
                (TeamTotalsMessage::ResultsRetrieved(status), receiver)
            },
        );
        subscriptions.push(retrieve_results);

        if self.status_timer.is_some() {
            let status_timer = iced::subscription::unfold(
                "status_timer",
                match &self.status_timer {
                    None => {panic!()}
                    Some(v) => {v}
                }.receiver.take(),
                move |mut receiver| async move {
                    match receiver.as_mut().unwrap().recv() {
                        Ok(_) => {
                            (TeamTotalsMessage::StatusTimerDone, receiver)
                        }
                        Err(err) => {
                            eprintln!("{}", err);
                            (TeamTotalsMessage::StatusTimerError, receiver)
                        }
                    }
                },
            );
            subscriptions.push(status_timer);
        }

        Subscription::batch(subscriptions)
    }
}

#[derive(Debug, Clone)]
pub enum EventToInclude {
    Edited(bool),
}

#[derive(Debug, Clone)]
pub struct EventsControls {
    pub(crate) index: usize,
    pub(crate) event: Event,
}

impl EventsControls {
    pub fn new(index: usize, event: Event) -> Self {
        EventsControls {
            index,
            event,
        }
    }

    pub fn view(&self) -> Element<EventToInclude> {
        let checkbox = checkbox("", self.event.active, EventToInclude::Edited);

        row![checkbox, text(&self.event.event_name).vertical_alignment(Vertical::Center).height(30)].align_items(Alignment::Center).into()
    }
}

#[derive(Debug, Clone)]
pub enum PointsForEachPlacement {
    Edited(String),
}

#[derive(Debug, Clone)]
pub struct PointsField {
    pub(crate) index: usize,
    pub(crate) value: String,
}

impl PointsField {
    pub fn new(index: usize, value: String) -> Self {
        PointsField {
            index,
            value,
        }
    }

    fn text_input_id(i: usize) -> text_input::Id {
        text_input::Id::new(format!("{i}"))
    }

    pub fn view(&self, index: usize) -> Element<PointsForEachPlacement> {
        let points_field = text_input(
            format!("Points for position {}", index + 1).as_str(),
            &self.value,
        ).id(Self::text_input_id(index)).on_input(PointsForEachPlacement::Edited);

       row![text(if index < 9 {format!("  {}: ", index + 1)} else {format!("{}: ", index + 1)}).vertical_alignment(Vertical::Center).height(30), points_field].into()
    }
}