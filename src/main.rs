use crate::basic_data::{
    competition::generate_games,
    team::{Stadium, Team},
};

mod basic_data;

fn main() {
    println!("Hello, world!");

    // let teams_list = vec![
    //     "Cruzeiro".to_string(),
    //     "São Paulo".to_string(),
    //     "Palmeiras".to_string(),
    //     "Flamengo".to_string(),
    //     "Coritiba".to_string(),
    //     "Santos".to_string(),
    //     "Corinthians".to_string(),
    //     "Botafogo".to_string(),
    //     "Fluminense".to_string(),
    //     "Atlético MG".to_string(),
    //     "Athletico PR".to_string(),
    //     "Grêmio".to_string(),
    //     "Internacional".to_string(),
    //     "Fortaleza".to_string(),
    //     "Bahia".to_string(),
    //     "Cuiabá".to_string(),
    //     "Vasco".to_string(),
    //     "Goiás".to_string(),
    //     "RedBull".to_string(),
    //     "América".to_string(),
    // ];

    let teams_list = vec![
        "Team A".to_string(),
        "Team B".to_string(),
        "Team C".to_string(),
        "Team D".to_string(),
        "Team E".to_string(),
        "Team F".to_string(),
    ];

    let matches_list = generate_games(teams_list);
    for (round_num, round_matches) in matches_list.iter().enumerate() {
        println!("Round {}: {:?}", round_num + 1, round_matches);
    }

    // let matches_list = generate_competion_games(teams_list);
    // for (round_num, round_matches) in matches_list.iter().enumerate() {
    //     println!("Round {}: ", round_num + 1);
    //     for game in round_matches {
    //         println!("{} vs {}", game.home, game.away);
    //     }
    // }
}
