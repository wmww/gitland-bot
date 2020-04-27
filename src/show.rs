use crate::*;
use std::collections::HashMap;

fn show_stats(game: &Game) {
    let mut players_by_team = HashMap::new();
    for player in game.players.values() {
        if let Some(count) = players_by_team.get_mut(&player.team) {
            *count += 1;
        } else {
            players_by_team.insert(player.team, 1);
        }
    }
    for (player, count) in players_by_team {
        println!("{:?} players: {}", player, count);
    }
}

pub fn show(game: &Game) {
    show_stats(game);
}
