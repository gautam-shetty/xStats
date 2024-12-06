use git2::{Commit, DiffOptions, Error, Repository};

pub struct GitManager {
    repo: Repository,
}

impl GitManager {
    pub fn new(repo_path: &str) -> Result<Self, Error> {
        let repo = Repository::open(repo_path)?;
        Ok(GitManager { repo })
    }

    pub fn get_all_commits(&self) -> Result<Vec<(Commit, Vec<String>)>, Error> {
        let mut revwalk = self.repo.revwalk()?;
        revwalk.push_head()?;
        revwalk.set_sorting(git2::Sort::TIME)?;

        let mut commits = Vec::new();
        for oid in revwalk {
            let oid = oid?;
            let commit = self.repo.find_commit(oid)?;

            let tree = commit.tree()?;
            let parent = if commit.parent_count() > 0 {
                Some(commit.parent(0)?.tree()?)
            } else {
                None
            };

            let mut diff_opts = DiffOptions::new();
            let diff =
                self.repo
                    .diff_tree_to_tree(parent.as_ref(), Some(&tree), Some(&mut diff_opts))?;

            let mut files = Vec::new();
            diff.foreach(
                &mut |delta, _| {
                    if let Some(path) = delta.new_file().path() {
                        files.push(path.to_string_lossy().into_owned());
                    }
                    true
                },
                None,
                None,
                None,
            )?;

            commits.push((commit, files));
        }

        Ok(commits)
    }
}
