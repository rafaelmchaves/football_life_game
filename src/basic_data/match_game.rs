use super::team::{Stadium, Team};

pub struct MatchInfo {
    id: String,
    home: Team,
    away: Team,
    date: String,
    time: String,
    stadium: Stadium,
}
#[derive(Debug)]
pub struct Match {
    pub home: String,
    pub away: String,
}
