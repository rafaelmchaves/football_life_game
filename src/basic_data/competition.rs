pub struct Competition {
    c_type: CompetitionType,
    name: str,
    amount_teams: i8,
    relegated_teams_amount: i8,
    season: str,
    rating: i8,
    competitionRegionType: CompetitionRegionType,
    region: str,
    teams: Vec<Team>,
    //TODO list of games
    //TODO standings of the competition
}

enum CompetitionType {
    CHAMPIONSHIP,
    CUP,
}

enum CompetitionRegionType {
    WORLD,
    CONTINENT,
    COUNTRY,
}
