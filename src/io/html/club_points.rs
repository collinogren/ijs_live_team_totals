use std::collections::HashMap;
use iced::widget::shader::wgpu::naga::FastHashMap;

use crate::io::excel::scoring_system_reader::read_scoring_system_spreadsheet;
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

pub fn sum_results(results_sets: &Vec<ResultSet>, settings: Settings) -> FastHashMap<String, ClubPoints> {
    let spreadsheet_scoring_system = if settings.use_scoring_system_spreadsheet {
        Some(read_scoring_system_spreadsheet(settings.scoring_system_file_name).unwrap())
    } else {
        None
    };

    let mut club_points_hashmap: FastHashMap<String, ClubPoints> = FastHashMap::default();

    for results_set in results_sets {
        let club = results_set.club.clone().unwrap();
        let club_exists = club_points_hashmap.contains_key(&club);

        if !club_exists {
            club_points_hashmap.insert(club.clone(), ClubPoints::new(club));
        }
    }

    for results_set in results_sets {
        let club = results_set.club.clone().unwrap();
        let club = match club_points_hashmap.get_mut(&club) {
            Some(club) => club,
            None => continue,
        };

        if spreadsheet_scoring_system.is_some() ||
            results_set.rank.clone().unwrap() <= settings.default_points_system.len() as u64 {

            let rank = match results_set.rank.clone() {
                Some(rank) => (rank - 1) as usize,
                None => {
                    eprintln!("Failed to get rank for at event {}, skater {}",
                              results_set.event.clone().unwrap_or(String::from("Unknown")),
                              results_set.name.clone().unwrap_or(String::from("Unknown")));
                    continue
                }
            };

            match &spreadsheet_scoring_system {
                Some(scoring_system) => {
                    let participants = match results_set.participants {
                        Some(participants) => participants,
                        None => {
                            eprintln!("Failed to get number of participants at event {}",
                                      results_set.event.clone().unwrap_or(String::from("Unknown")));
                            continue
                        },
                    };

                    let scoring_system_for_n_participants = match scoring_system.get(&participants) {
                        Some(scoring_system_for_n_participants) => scoring_system_for_n_participants,
                        None => {
                            eprintln!("Failed to get scoring system column at event {}",
                                      results_set.event.clone().unwrap_or(String::from("Unknown")));
                            continue
                        },
                    };

                    match results_set.scoring_system {
                        ScoringSystem::IJS => {
                            club.points_ijs.replace(
                                club
                                    .points_ijs()
                                    .unwrap_or(0.0) + scoring_system_for_n_participants
                                    .get(rank)
                                    .unwrap_or(&0.0));
                        }

                        ScoringSystem::SixO => {
                            club.points_60.replace(
                                club
                                    .points_60()
                                    .unwrap_or(0.0) + scoring_system_for_n_participants
                                    .get(rank)
                                    .unwrap_or(&0.0));
                        }
                    };
                }
                None => {
                    match results_set.scoring_system {
                        ScoringSystem::IJS => { club.points_ijs.replace(club.points_ijs.unwrap_or(0.0) + settings.default_points_system[rank]); }
                        ScoringSystem::SixO => { club.points_60.replace(club.points_60().unwrap_or(0.0) + settings.default_points_system[rank]); }
                    };
                }
            }
        }
    }

    if settings.attempt_automatic_60_club_name_recombination {
        auto_club_combiner_hashmap(&mut club_points_hashmap);
    }

    club_points_hashmap
}

pub fn auto_club_combiner_hashmap(club_points: &mut FastHashMap<String, ClubPoints>) {
    let truncated_clubs: Vec<(String, ClubPoints)> = club_points
        .iter()
        .filter(|(key, _)| key.ends_with("..."))
        .filter_map(|(key, truncated_club_points)| {
            let truncated_key = key.strip_suffix("...")?;

            if club_points.iter().any(|(full_key, _)| full_key != key && full_key.starts_with(truncated_key)) {
                Some((truncated_key.to_string(), truncated_club_points.clone()))
            } else {
                None
            }
        })
        .collect();

    truncated_clubs.iter().for_each(|(truncated_key, truncated_club)| {
        let club_points_binding = club_points.clone();
        let club_to_combine = club_points_binding
            .iter()
            .find(|(key, _)| key.starts_with(truncated_key));

        match club_to_combine {
            Some((key, _)) => {
                match club_points.get_mut(key) {
                    Some(full_club_points) => {
                        match &mut full_club_points.points_ijs {
                            Some(ref mut points_ijs) => {
                                *points_ijs += truncated_club.points_ijs.unwrap_or(0.0);
                            }
                            None => {}
                        }

                        match &mut full_club_points.points_60 {
                            Some(ref mut points_60) => {
                                *points_60 += truncated_club.points_60.unwrap_or(0.0);
                            }
                            None => {}
                        }

                        club_points.remove(&format!("{}{}", truncated_key, "..."));
                    }
                    None => {}
                }
            }
            None => {}
        }
    });
}