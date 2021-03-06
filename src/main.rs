#[macro_use]
extern crate clap;

mod act;
mod ai;
mod arg;
mod game;
mod git;
mod show;

use act::act;
use arg::{parse_arguments, ActArgs, Command};
use game::*;
use git::ServerRepo;
use show::show;

fn main() {
    let args = parse_arguments();
    eprintln!("Running with arguments: {:?}", args);
    let server_repo =
        ServerRepo::new(&args.server_repo_path).expect("failed to initialize server repo");
    let mut game = server_repo
        .load_game(Some(12))
        .expect("failed to load game");
    game.us = Some("wmww".to_owned());
    match args.command {
        Command::Show => show(&game),
        Command::Act(act_args) => act(&game, &act_args),
    }
    eprintln!("Done");
}
