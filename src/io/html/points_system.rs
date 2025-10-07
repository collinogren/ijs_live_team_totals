use std::collections::HashMap;
use crate::io::excel::scoring_system_reader;
use crate::settings::settings::Settings;

pub struct PointsSystem {
    single_points_system: Vec<f64>,
    spreadsheet_points_system: HashMap<u64, Vec<f64>>,
}

impl PointsSystem {
    pub fn default(settings: &Settings) -> Self {
        Self {
            single_points_system: settings.default_points_system.clone(),
            spreadsheet_points_system: scoring_system_reader::deserialize(settings.scoring_system_file_name.clone()),
        }
    }
}