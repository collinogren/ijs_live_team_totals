#[derive(Debug, Clone, PartialEq)]
pub enum ScoringSystem {
    IJS,
    SixO,
}

impl ScoringSystem {
    pub fn get_name(&self) -> String {
        match self {
            ScoringSystem::IJS => {
                String::from("IJS")
            }
            ScoringSystem::SixO => {
                String::from("6.0")
            }
        }
    }
}