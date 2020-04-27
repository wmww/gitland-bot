use crate::*;
use std::collections::BTreeMap;

fn show_player_stats(game: &Game) {
    let mut players_by_team = BTreeMap::new();
    for player in game.players.values() {
        if let Some(count) = players_by_team.get_mut(&player.team) {
            *count += 1;
        } else {
            players_by_team.insert(player.team, 1);
        }
    }
    for (player, count) in players_by_team {
        println!("{} {:?} players", count, player);
    }
}

fn show_map_stats(map: &Map) {
    let mut squares_by_team = BTreeMap::new();
    for square in map.squares.iter().flatten() {
        if let Some(count) = squares_by_team.get_mut(&square.controlled_by) {
            *count += 1;
        } else {
            squares_by_team.insert(square.controlled_by, 1);
        }
    }
	let total_squares = map.squares.len() * map.squares[0].len();
    for (player, count) in squares_by_team {
        println!("{} squares controlled by {:?} ({:.1}%)", count, player, (count as f32) / (total_squares as f32) * 100.0);
    }
}

pub fn show(game: &Game) {
    show_player_stats(game);
	show_map_stats(game.timeline.last().expect("no maps"));
}
