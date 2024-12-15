pub use git2::{Delta, DiffOptions, Repository, Revwalk, Sort, Tree};
use std::process;

pub fn open_repo(path: &str) -> Repository {
    let repo = match Repository::open(path) {
        Ok(repo) => repo,
        Err(e) => {
            println!("Failed to open repository: {}", e);
            process::exit(1);
        }
    };

    repo
}

pub fn generate_revwalk(repo: &Repository) -> Revwalk {
    let mut revwalk = match repo.revwalk() {
        Ok(walk) => walk,
        Err(e) => {
            println!("Failed to create revwalk: {}", e);
            process::exit(1);
        }
    };

    revwalk.push_head().expect("Failed to push HEAD to revwalk");
    revwalk
        .set_sorting(Sort::REVERSE)
        .expect("Failed to set sorting");

    revwalk
}
