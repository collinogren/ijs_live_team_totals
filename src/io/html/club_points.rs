use crate::io::excel::scoring_system_reader::deserialize;
use crate::io::html::result_set::ResultSet;
use crate::io::html::scoring_system::ScoringSystem;
use crate::settings::settings::Settings;

#[derive(Clone, Debug)]
pub struct ClubPoints {
    club: String,
    points_ijs: Option<f64>,
    points_60: Option<f64>,
}

impl ClubPoints {
    pub fn new(club: String) -> Self {
        Self {
            club,
            points_ijs: Some(0.0),
            points_60: Some(0.0),
        }
    }

    pub fn club(&self) -> &String {
        &self.club
    }

    pub fn points_ijs(&self) -> Option<f64> {
        self.points_ijs
    }

    pub fn points_60(&self) -> Option<f64> {
        self.points_60
    }

    pub fn set_club(&mut self, club: String) {
        self.club = club;
    }

    pub fn set_points_ijs(&mut self, points_ijs: f64) {
        self.points_ijs.replace(points_ijs);
    }

    pub fn set_points_60(&mut self, points_60: f64) {
        self.points_60.replace(points_60);
    }

    pub fn set_points_ijs_none(&mut self) {
        self.points_ijs = None;
    }

    pub fn set_points_60_none(&mut self) {
        self.points_60 = None;
    }

    pub fn calc_total(&self) -> f64 {
        self.points_ijs.unwrap_or(0.0) + self.points_60.unwrap_or(0.0)
    }
}

pub fn sum_results(results_sets: Vec<ResultSet>, settings: Settings) -> Vec<ClubPoints> {
    let spreadsheet_scoring_system = if settings.use_scoring_system_spreadsheet {
        Some(deserialize(settings.scoring_system_file_name))
    } else {
        None
    };

    let mut club_points_vec: Vec<ClubPoints> = vec![];

    for results_set in &results_sets {
        let club = results_set.club.clone().unwrap();
        let mut club_exists = false;
        for club_points in &club_points_vec {
            if club_points.club.eq(&results_set.club.clone().unwrap()) {
                club_exists = true;
                break;
            }
        }

        if !club_exists {
            club_points_vec.push(ClubPoints::new(club));
        }
    }

    for results_set in &results_sets {
        for club in &mut club_points_vec {
            if results_set.club.clone().unwrap().eq(&club.club) {
                if results_set.rank.clone().unwrap() <= settings.default_points_system.len() as u64 {
                    match &spreadsheet_scoring_system {
                        Some(scoring_system) => {
                            match results_set.scoring_system {
                                ScoringSystem::IJS => { club.points_ijs.replace(club.points_ijs.unwrap_or(0.0) + scoring_system.get(&results_set.participants.unwrap()).unwrap().get((results_set.rank.clone().unwrap() - 1) as usize).unwrap()); }
                                ScoringSystem::SixO => { club.points_60.replace(club.points_60().unwrap_or(0.0) + scoring_system.get(&results_set.participants.unwrap()).unwrap().get((results_set.rank.clone().unwrap() - 1) as usize).unwrap()); }
                            };
                        },
                        None => {
                            match results_set.scoring_system {
                                ScoringSystem::IJS => { club.points_ijs.replace(club.points_ijs.unwrap_or(0.0) + settings.default_points_system[(results_set.rank.clone().unwrap() - 1) as usize]); }
                                ScoringSystem::SixO => { club.points_60.replace(club.points_60().unwrap_or(0.0) + settings.default_points_system[(results_set.rank.clone().unwrap() - 1) as usize]); }
                            };
                        }
                    }
                }

                continue;
            }
        }
    }

    club_points_vec
}

#[allow(unused)]
pub fn auto_club_combiner(mut club_points: Vec<ClubPoints>) -> Vec<ClubPoints> {
    let mut broken_60_club_names = vec![];
    let mut indices_to_remove = vec![];

    let mut i = 0;
    for mut club in club_points.clone() {
        if club.club.ends_with("...") { // 6.0 results html files used a clipped club name if they are longer than 21 characters with "..." at the end.
            let corrected_club_name = club.club.split("...").nth(0);
            club.club = corrected_club_name.unwrap().to_string();
            broken_60_club_names.push(club);
            indices_to_remove.push(i);
        }

        i += 1;
    }

    indices_to_remove.reverse();

    let mut ret_club_points = vec![];

    'outer: for (i, club) in club_points.into_iter().enumerate() {
        for r in &indices_to_remove {
            if i == r.clone() {
                continue 'outer;
            }
        }

        ret_club_points.push(club.clone());
    }

    let mut unicorn_clubs = vec![];

    'outer: for broken_club in &broken_60_club_names {
        for mut club in &mut ret_club_points {
            let mut limited_club = club.club.clone();
            if club.club.len() > 21 {
                let mut limited_club = limited_club.as_bytes();
                let mut limited_club = limited_club[0..21].to_vec();
                let mut limited_club = String::from_utf8(limited_club).unwrap();
            }

            if limited_club.starts_with(&broken_club.club.clone()) {
                club.points_ijs.replace(club.points_ijs.unwrap_or(0.0) + broken_club.points_ijs.unwrap_or(0.0));
                club.points_60.replace(club.points_60.unwrap_or(0.0) + broken_club.points_60.unwrap_or(0.0));
                continue 'outer;
            }
        }

        let mut broken_club_copy = broken_club.clone();
        broken_club_copy.club = broken_club_copy.club + "...";
        unicorn_clubs.push(broken_club_copy);
    }

    ret_club_points.extend(unicorn_clubs);
    ret_club_points
}