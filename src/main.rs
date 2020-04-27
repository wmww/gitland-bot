#[macro_use]
extern crate clap;

mod arg;
mod git;

use arg::parse_arguments;
use git::ServerRepo;

fn main() {
    let args = parse_arguments();
    eprintln!("Running with arguments: {:?}", args);
    let server_repo =
        ServerRepo::new(&args.server_repo_path).expect("failed to initialize server repo");
    let game = server_repo.get_game();
}
