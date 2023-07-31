use std::collections::{HashMap, HashSet};

use super::{match_game::Match, team::Team};

pub struct Competition {
    id: String,
    c_type: CompetitionType,
    name: String,
    amount_teams: i8,
    relegated_teams_amount: i8,
    season: String,
    rating: i8,
    competition_region_type: CompetitionRegionType,
    region: String,
    teams: Vec<Team>,
    classified: Vec<ClassifiedMetadata>,
    game_rounds: HashMap<i32, Vec<Match>>,
    standings: Vec<Standing>,
}

struct ClassifiedMetadata {
    index_rule: i8, //index of this metadata, the lower the number, the rule is precedent
    amount_teams: i8,
    competition_id: String,
    stage_next_competition: String,
    is_promoted: bool, //if the team was promoted to other competition or only advanced to a next stage in the same competition
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

struct Standing {
    position: i8,
    group: String,
    team: Team,
    pts: i16,
    goals_for: i16,
    goals_against: i16,
    goal_difference: i16,
    played: i8,
    won: i8,
    drawn: i8,
    lost: i8,
}

//TODO create a method generate_competition that it checks Competition type and metadata to know how kind of game rounds will be generated.

pub fn generate_games(teams: Vec<String>) -> HashMap<i32, Vec<Match>> {
    let mut rounds: HashMap<i32, Vec<Match>> = HashMap::new();

    let num_teams = teams.len();
    let mut played_against_team: HashMap<String, Vec<String>> = HashMap::new();

    let mut round_index: i32 = 1;
    let teams_aux = teams.clone();

    let mut round_teams_set: HashSet<String> = HashSet::new();

    loop {
        if has_all_teams_played_against_every_team(&played_against_team, &teams) {
            break;
        }

        let mut games: Vec<Match> = Vec::new();
        for team in teams.iter() {
            let mut i = 0;
            loop {
                if has_conditions_to_play_current_round(
                    &team,
                    &teams_aux,
                    i,
                    &played_against_team,
                    num_teams,
                    &round_teams_set,
                ) {
                    let game = Match {
                        team1: team.clone(),
                        team2: teams_aux[i].clone(),
                    };

                    played_against_team
                        .entry(game.team1.clone())
                        .or_insert_with(|| vec![game.team2.clone()])
                        .push(game.team2.clone());

                    played_against_team
                        .entry(game.team2.clone())
                        .or_insert_with(|| vec![game.team1.clone()])
                        .push(game.team1.clone());

                    round_teams_set.insert(team.clone());
                    round_teams_set.insert(teams_aux[i].clone());

                    break games.push(game);
                } else if i == num_teams - 1 {
                    break;
                } else {
                    i += 1;
                }
            }
        }

        rounds.insert(round_index, games);
        round_index += 1;
        round_teams_set = HashSet::new();
    }

    rounds
}

fn has_all_teams_played_against_every_team(
    played_adversaries_team: &HashMap<String, Vec<String>>,
    teams: &Vec<String>,
) -> bool {
    played_adversaries_team.len() == teams.len()
        && played_adversaries_team
            .iter()
            .all(|(_team, vec)| vec.len() == teams.len())
}

fn has_conditions_to_play_current_round(
    team: &String,
    teams_aux: &Vec<String>,
    i: usize,
    played_against_team: &HashMap<String, Vec<String>>,
    teams_amount: usize,
    round_teams_set: &HashSet<String>,
) -> bool {
    i < teams_amount - 1
        && (!round_teams_set.contains(team) && !round_teams_set.contains(&teams_aux[i]))
        && !team.eq(&*teams_aux[i])
        && played_against_team
            .get(&*team)
            .map_or(true, |played_against_teams| {
                !played_against_teams.contains(&teams_aux[i])
            })
        && played_against_team
            .get(&*teams_aux[i])
            .map_or(true, |played_against_teams| {
                !played_against_teams.contains(team)
            }) //verify if a team already played against the next team in the championship
}
