use git2::{BranchType, Commit, ObjectType, Repository, Tree, TreeEntry};
use std::collections::HashMap;
use std::error::Error;
use std::ffi::OsStr;
use std::str::from_utf8;

use crate::{Game, Position, Team};

pub struct ServerRepo {
    repo: Repository,
}

#[derive(Debug)]
struct PlayerData {
    name: String,
    team: Team,
    position: Position,
}

#[derive(Debug)]
struct BoardData {
    squares: Vec<Vec<Team>>,
}

impl ServerRepo {
    pub fn new(path: &OsStr) -> Result<Self, Box<dyn Error>> {
        eprintln!("Trying to open server repo at {}", path.to_string_lossy());
        let repo = Repository::open(path)?;
        Ok(ServerRepo { repo })
    }

    fn load_contents_of_file(&self, tree: &Tree, name: &str) -> Result<String, Box<dyn Error>> {
        let tree_entry = tree
            .get_name(name)
            .ok_or(format!("no file named {}", name))?;
        let blob = tree_entry.to_object(&self.repo)?.peel_to_blob()?;
        let text = from_utf8(blob.content())?.to_owned();
        Ok(text)
    }

    fn load_player_from_tree_entry(
        &self,
        tree_entry: &TreeEntry,
    ) -> Result<PlayerData, Box<dyn Error>> {
        let name = tree_entry
            .name()
            .ok_or("player tree entry does not have name")?
            .to_owned();
        let tree = tree_entry.to_object(&self.repo)?.peel_to_tree()?;
        let team_str = self.load_contents_of_file(&tree, "team")?;
        let x_str = self.load_contents_of_file(&tree, "x")?;
        let y_str = self.load_contents_of_file(&tree, "y")?;
        let team = team_str.parse()?;
        let x = x_str.parse()?;
        let y = y_str.parse()?;
        let position = Position::new(x, y);
        Ok(PlayerData {
            name,
            team,
            position,
        })
    }

    fn load_players_from_commit(&self, commit: &Commit) -> Result<Vec<PlayerData>, Box<dyn Error>> {
        let root_tree = commit.tree()?;
        let players_tree_entry = root_tree
            .get_name("players")
            .ok_or("no players directory")?;
        let players_tree = players_tree_entry.to_object(&self.repo)?.peel_to_tree()?;
        let mut players = Vec::new();
        for player_tree_entry in players_tree.iter() {
            match self.load_player_from_tree_entry(&player_tree_entry) {
                Ok(player) => players.push(player),
                Err(e) => eprintln!(
                    "Failed to load player {}: {}",
                    player_tree_entry.name().unwrap_or("[no name]"),
                    e
                ),
            }
        }
        Ok(players)
    }

    pub fn load_game(&self, history_limit: Option<u32>) -> Result<Game, Box<dyn Error>> {
        let master = self.repo.find_branch("master", BranchType::Local)?;
        let last_commit = master.into_reference().peel_to_commit()?;
        let players = self.load_players_from_commit(&last_commit)?;
        eprintln!("Players: {:#?}", players);
        let game = Game {
            players: HashMap::new(),
            timeline: Vec::new(),
        };
        Ok(game)
    }
}
