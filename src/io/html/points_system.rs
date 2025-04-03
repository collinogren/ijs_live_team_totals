use crate::settings::settings::Settings;

pub struct PointsSystem {
    points_system: Vec<f64>,
}

impl PointsSystem {
    pub fn default(settings: &Settings) -> Self {
        Self {
            points_system: settings.default_points_system.clone(),
        }
    }

    pub fn new_custom(points_system: Vec<f64>) -> Self {
        Self {
            points_system,
        }
    }
}