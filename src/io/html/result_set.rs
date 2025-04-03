use crate::io::html::points_system::PointsSystem;
use crate::io::html::scoring_system::ScoringSystem;

pub fn clean_club_names(result_sets: &mut Vec<ResultSet>) {
    for result_set in result_sets {
        let name = match &result_set.club {
            Some(name) => { name }
            None => { continue }
        };

        let mut temp = name.clone();
        for character_entities in crate::io::html::parser::HTML_CHARACTER_ENTITIES {
            temp = temp.replace(character_entities.0, character_entities.1);
        }

        result_set.club = Some(temp);
    }
}

pub struct ResultSet {
    pub(crate) rank: Option<u64>,
    pub(crate) club: Option<String>,
    pub(crate) scoring_system: ScoringSystem,
    pub(crate) points_system: PointsSystem,
}

impl ResultSet {
    pub(crate) fn new(scoring_system: ScoringSystem, points_system: PointsSystem) -> Self {
        Self {
            rank: None,
            club: None,
            scoring_system,
            points_system,
        }
    }
}