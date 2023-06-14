use std::fs;
use std::path::Path;
use serde_derive::{Deserialize, Serialize};

#[derive(Clone)]
#[derive(Serialize, Deserialize)]
pub struct Settings {
    pub(crate) points_for_each_placement: Vec<f64>,
    pub(crate) include_60: bool,
    pub(crate) include_ijs: bool,
    pub(crate) attempt_automatic_60_club_name_recombination_inop: bool,
    pub(crate) use_event_name_for_results_path: bool, //If this is set to true, then the program will find the results based on event name rather than absolute path.
    pub(crate) isu_calc_base_directory: String,
    pub(crate) html_relative_directory: String,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            points_for_each_placement: vec![3.0, 2.0, 1.0],
            include_60: true,
            include_ijs: true,
            attempt_automatic_60_club_name_recombination_inop: false,
            use_event_name_for_results_path: true,
            isu_calc_base_directory: String::from("C:/ISUCalcFS/"),
            html_relative_directory: String::from("/IJScompanion_html_winnercomm"),
        }
    }
}

impl Settings {
    #[allow(unused)]
    pub fn new(points_for_each_placement: Vec<f64>,
               include_60: bool,
               include_ijs: bool,
               participant_quantity_exclusion_point: u64,
               attempt_automatic_60_club_name_recombination_inop: bool,
               use_event_name_for_results_path: bool,
               isu_calc_base_directory: String,
               html_relative_directory: String,
    ) -> Self {
        Settings {
            points_for_each_placement,
            include_60,
            include_ijs,
            attempt_automatic_60_club_name_recombination_inop,
            use_event_name_for_results_path,
            isu_calc_base_directory,
            html_relative_directory,
        }
    }

    pub fn read_settings() -> Self {
        let settings_path = Path::new("./settings/settings.toml");
        if !settings_path.exists() {
            match fs::create_dir("./settings") {
                Ok(_) => {}
                Err(err) => eprintln!("Failed to create settings directory: {}", err),
            };
            let toml = match toml::to_string(&Settings::default()) {
                Ok(v) => v,
                Err(err) => {
                    eprintln!("Failed to serialize default settings.toml file: {}", err);
                    format!("Failed to serialize default settings.toml file: {}", err)
                }
            };
            match fs::write(settings_path, toml) {
                Ok(_) => {}
                Err(err) => eprintln!("Failed to write to settings.toml file: {}", err),
            }
        }
        let contents = match fs::read_to_string("./settings/settings.toml") {
            Ok(v) => v,
            Err(err) => {
                eprintln!("Failed to read settings file: {}\nUsing default values.", err);
                return Settings::default();
            }
        };

        let settings: Settings = match toml::from_str(&contents) {
            Ok(s) => s,
            Err(err) => {
                eprintln!("Failed to read settings file: {}\nUsing default values.", err);
                Settings::default()
            }
        };

        settings
    }
}