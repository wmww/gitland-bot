#[macro_use]
extern crate clap;

mod ai;
mod arg;
mod game;
mod git;

use arg::parse_arguments;
use game::{Direction, Game, Map, Player, Position, Square, Team};
use git::ServerRepo;

fn main() {
    let args = parse_arguments();
    eprintln!("Running with arguments: {:?}", args);
    let server_repo =
        ServerRepo::new(&args.server_repo_path).expect("failed to initialize server repo");
    let game = server_repo
        .load_game(Some(12))
        .expect("failed to load game");
    //eprintln!("Game: {:?}", game);
    let action = ai::run(&game);
    eprintln!("AI thinks we should move {:?}", action);
}
