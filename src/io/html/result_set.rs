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


#[derive(Clone, Debug)]
pub struct ResultSet {
    pub (crate) event: Option<String>,
    pub(crate) name: Option<String>,
    pub(crate) club: Option<String>,
    pub(crate) rank: Option<u64>,
    pub(crate) participants: Option<u64>,
    pub(crate) scoring_system: ScoringSystem,
}

impl ResultSet {
    pub(crate) fn new(scoring_system: ScoringSystem) -> Self {
        Self {
            event: None,
            name: None,
            rank: None,
            participants: None,
            club: None,
            scoring_system,
        }
    }

    pub fn event(&self) -> String {
        self.event.clone().unwrap_or_else(|| String::from("Unknown Event"))
    }

    pub fn name(&self) -> String {
        self.name.clone().unwrap_or_else(|| String::from("Unknown Name"))
    }

    pub fn rank(&self) -> u64 {
        self.rank.unwrap_or(0)
    }

    pub fn participants(&self) -> u64 {
        self.participants.unwrap_or(0)
    }

    pub fn club(&self) -> String {
        self.club.clone().unwrap_or_else(|| String::from("Unknown Club"))
    }

    pub fn scoring_system(&self) -> &ScoringSystem {
        &self.scoring_system
    }
}