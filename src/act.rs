use super::*;
use std::fs::{remove_file, OpenOptions};
use std::io::Write;

pub fn act(game: &Game, args: &ActArgs) {
    let action = ai::run(&game);
    eprintln!("AI thinks we should move {:?}", action);
    let action_str = action.to_string();
    let mut act_path = args.client_repo_path.clone();
    act_path.push("/act");
    eprintln!("Writing {:?} to {:?}", action_str, act_path);
    remove_file(&act_path).expect("failed to remove old file");
    let mut act_file = OpenOptions::new()
        .write(true)
        .create(true)
        .open(&act_path)
        .expect("failed to open act file");
    act_file
        .write_all(action_str.as_bytes())
        .expect("failed to write to file");
}
