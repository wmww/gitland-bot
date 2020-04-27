use git2::{BranchType, Repository};
use std::error::Error;
use std::ffi::OsStr;

pub struct ServerRepo {
    repo: Repository,
}

impl ServerRepo {
    pub fn new(path: &OsStr) -> Result<Self, Box<dyn Error>> {
        eprintln!("Trying to open server repo at {}", path.to_string_lossy());
        let repo = Repository::open(path)?;
        Ok(ServerRepo { repo })
    }

    pub fn get_game(&self) -> Result<(), Box<dyn Error>> {
        let master = self.repo.find_branch("master", BranchType::Local)?;
        let last_commit = master.into_reference().peel_to_commit()?;
        println!(
            "Last commit message: {}",
            last_commit.message().unwrap_or("[none]")
        );
        Ok(())
    }
}
