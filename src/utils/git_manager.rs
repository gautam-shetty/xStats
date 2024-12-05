use git2::{Commit, Error, Repository};

pub struct GitManager {
    repo: Repository,
}

impl GitManager {
    pub fn new(repo_path: &str) -> Result<Self, Error> {
        let repo = Repository::open(repo_path)?;
        Ok(GitManager { repo })
    }

    pub fn get_all_commits(&self) -> Result<Vec<Commit>, Error> {
        let mut revwalk = self.repo.revwalk()?;
        revwalk.push_head()?;
        revwalk.set_sorting(git2::Sort::TIME)?;

        let mut commits = Vec::new();
        for oid in revwalk {
            let oid = oid?;
            let commit = self.repo.find_commit(oid)?;
            commits.push(commit);
        }

        Ok(commits)
    }
}
