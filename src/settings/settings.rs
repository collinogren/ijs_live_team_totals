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

use std::fs;
use std::path::Path;
use directories::{ProjectDirs, UserDirs};
use serde_derive::{Deserialize, Serialize};

const SETTINGS_FILE: &'static str = "/settings.toml";

// Return appdata directory.
pub fn appdata(relative: &str) -> (String, String) {
    match ProjectDirs::from("", "",  "Auto Team Totals") {
        Some(v) => (format!("{}{}", v.config_dir().to_string_lossy().replace("\\", "/"), relative), v.config_dir().to_string_lossy().replace("\\", "/")),
        None => (String::new(), String::new()),
    }
}

// Return documents directory.
pub fn documents() -> String {
    match UserDirs::new() {
        None => {String::new()}
        Some(user_dirs) => {
            match user_dirs.document_dir() {
                None => {String::new()}
                Some(v) => {format!("{}{}", v.to_string_lossy().replace("\\", "/"), "/Auto Team Totals")}
            }
        }
    }
}

// Serializable struct to store all permanent program settings.
#[derive(Clone)]
#[derive(Serialize, Deserialize)]
pub struct Settings {
    pub(crate) default_points_system: Vec<f64>,
    pub(crate) include_60: bool,
    pub(crate) include_ijs: bool,
    pub(crate) generate_xlsx: bool,
    pub(crate) generate_html: bool,
    pub(crate) attempt_automatic_60_club_name_recombination: bool,
    pub(crate) use_event_name_for_results_path: bool, //If this is set to true, then the program will find the results based on event name rather than absolute path.
    pub(crate) isu_calc_base_directory: String,
    pub(crate) html_relative_directory: String,
    pub(crate) output_directory: String,
    pub(crate) xlsx_file_name: String,
    pub(crate) html_file_name: String,
    pub(crate) xlsx_header_cell_values: Vec<String>,
    pub(crate) xlsx_column_widths: Vec<i32>,
    pub(crate) xlsx_font_size: u32,
}

// Default settings
impl Default for Settings {
    fn default() -> Self {
        Self {
            default_points_system: vec![3.0, 2.0, 1.0],
            include_60: true,
            include_ijs: true,
            generate_xlsx: true,
            generate_html: true,
            attempt_automatic_60_club_name_recombination: true,
            use_event_name_for_results_path: true,
            isu_calc_base_directory: String::from("C:/ISUCalcFS/"),
            html_relative_directory: String::from("/IJScompanion_html_winnercomm"),
            output_directory: documents(),
            xlsx_file_name: String::from("team_totals.xlsx"),
            html_file_name: String::from("team_totals.html"),
            xlsx_header_cell_values: vec![String::from("Rank"), String::from("Club"), String::from("IJS"), String::from("6.0"), String::from("Total")],
            xlsx_column_widths: vec![15, 100, 11, 11, 15],
            xlsx_font_size: 32,
        }
    }
}
 impl Settings {
     #[allow(unused)]
    pub fn new(
        points_for_each_placement: Vec<f64>,
        include_60: bool,
        include_ijs: bool,
        generate_xlsx: bool,
        generate_html: bool,
        participant_quantity_exclusion_point: u64,
        attempt_automatic_60_club_name_recombination: bool,
        use_event_name_for_results_path: bool,
        isu_calc_base_directory: String,
        html_relative_directory: String,
        output_directory: String,
        xlsx_file_name: String,
        html_file_name: String,
        xlsx_header_cell_values: Vec<String>,
        xlsx_column_widths: Vec<i32>,
        xlsx_font_size: u32,
    ) -> Self {
        Settings {
            default_points_system: points_for_each_placement,
            include_60,
            include_ijs,
            generate_xlsx,
            generate_html,
            attempt_automatic_60_club_name_recombination,
            use_event_name_for_results_path,
            isu_calc_base_directory,
            html_relative_directory,
            output_directory,
            xlsx_file_name,
            html_file_name,
            xlsx_header_cell_values,
            xlsx_column_widths,
            xlsx_font_size,
        }
    }

     // Read all settings from file. If no file exists, the file will be created and the default
     // settings will be used and written to the new file.
    pub fn read() -> Self {
        let (settings_file, settings_dir) = appdata(SETTINGS_FILE);
        if !Path::new(settings_file.as_str()).exists() {
            match fs::create_dir_all(settings_dir.clone()) {
                Ok(_) => {}
                Err(err) => eprintln!("Failed to create settings directory at {}: {}", settings_dir, err),
            };
            let toml = toml::to_string(&Settings::default()).unwrap_or_else(|err| {
                eprintln!("Failed to serialize default settings.toml file: {}", err);
                format!("Failed to serialize default settings.toml file: {}", err)
            });
            match fs::write(settings_file.clone(), toml) {
                Ok(_) => {}
                Err(err) => eprintln!("Failed to write to settings.toml file: {}", err),
            }
        }
        let contents = match fs::read_to_string(settings_file.clone()) {
            Ok(v) => v,
            Err(err) => {
                eprintln!("Failed to read settings file: {}\nUsing default values.", err);
                return Settings::default();
            }
        };

        let settings: Settings = toml::from_str(&contents).unwrap_or_else(|err| {
            eprintln!("Failed to read settings file: {}\nUsing default values.", err);
            Settings::default()
        });

        settings
    }

     // Write the TOML file from the serializable struct.
    pub fn write(&self) {
        let (settings_file, _settings_dir) = appdata(SETTINGS_FILE);
        let toml = match toml::to_string(self) {
            Ok(toml) => { toml }
            Err(err) => {
                eprintln!("Failed to serialize settings: {}", err);
                return
            }
        };

        match fs::write(settings_file, toml) {
            Ok(_) => {
                println!("Successfully wrote to settings.toml")
            }
            Err(err) => {
                eprintln!("Failed to write to settings.toml: {}", err)
            }
        }
    }

    pub fn xlsx_path(&self) -> String {
        self.output_directory.clone() + "/" + self.xlsx_file_name.as_str()
    }
}
