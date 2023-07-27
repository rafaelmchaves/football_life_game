use super::team::{Stadium, Team};

pub struct MatchCompleted {
    id: String,
    home: Team,
    away: Team,
    date: String,
    time: String,
    stadium: Stadium,
}
#[derive(Debug)]
pub struct Match {
    pub team1: String,
    pub team2: String,
}
