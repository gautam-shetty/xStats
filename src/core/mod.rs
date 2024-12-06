use crate::metrics::CodeMetrics;
use crate::parser::TSParsers;
use crate::utils::git_manager::GitManager;
use crate::utils::{save_to_csv, save_to_json, traverse_path};
use indicatif::{ProgressBar, ProgressStyle};

pub struct XStats {
    target_path: String,
    output_path: String,
    git_manager: Option<GitManager>,
    parsers: TSParsers,
    metrics: CodeMetrics,
}

impl XStats {
    pub fn new(target_path: String, output_path: String, multi_commit: bool) -> Self {
        let parsers = TSParsers::new();

        let git_manager = if multi_commit {
            Some(GitManager::new(&target_path).expect("Could not find git information."))
        } else {
            None
        };

        Self {
            target_path,
            output_path,
            git_manager,
            parsers,
            metrics: CodeMetrics::new(),
        }
    }

    pub fn get_git_manager(&self) -> &GitManager {
        self.git_manager
            .as_ref()
            .expect("GitManager was not instantiated for the target repository.")
    }

    pub fn print_commits(&self) {
        let git_manager = self.get_git_manager();
        match git_manager.get_all_commits() {
            Ok(repo_commits) => {
                for (commit, modified_files) in repo_commits {
                    let commit_id = commit.id().to_string();
                    println!("Commit ID: {}", commit_id);
                    for file in modified_files {
                        println!("\tFile: {}", file);
                    }
                    println!();
                }
            }
            Err(e) => {
                println!("Failed to get commits: {}", e);
            }
        }
    }

    pub fn run(&mut self) {
        match traverse_path(&self.target_path) {
            Ok(files) => {
                if files.is_empty() {
                    println!(
                        "No files to process in the target path {}",
                        self.target_path
                    );
                } else {
                    let file_count = files.len();
                    let prog_bar = ProgressBar::new(file_count as u64);
                    prog_bar.set_style(
                        ProgressStyle::default_bar()
                            .template(
                                "[{elapsed}] {bar:100} [{pos}/{len}]\n{spinner:.green} Processing file: {msg}",
                            )
                            .expect("Failed to set progress bar style"),
                    );

                    // Analyze each file
                    for file in &files {
                        prog_bar.set_message(file.to_string());
                        self.process_file(file);
                        prog_bar.inc(1);
                    }

                    prog_bar.finish_and_clear();
                }
            }
            Err(e) => {
                println!("{}", e);
            }
        }
    }

    fn process_file(&mut self, file: &str) {
        if let Some((language, tree, source_code)) = self.parsers.generate_tree(&file) {
            self.metrics.generate_root_metrics(
                &self.parsers,
                &source_code,
                &language.to_string(),
                &file.to_string(),
                &tree,
            );
        }
    }

    pub fn save_data_as_csv(&self) {
        let output_file = format!("{}/metrics.csv", self.output_path);
        let data = self.get_metrics();
        if save_to_csv(&output_file, data).is_ok() {
            println!("Code metrics saved at {}", output_file);
        } else {
            println!("Failed to save metrics to CSV");
        }
    }

    pub fn save_data_as_json(&self) {
        let output_file = format!("{}/metrics.json", self.output_path);
        let data: Vec<Vec<String>> = self.get_metrics();
        if save_to_json(&output_file, data).is_ok() {
            println!("Code metrics saved at {}", output_file);
        } else {
            println!("Failed to save metrics to JSON");
        }
    }

    pub fn get_metrics(&self) -> Vec<Vec<String>> {
        let mut data = Vec::new();
        data.push(vec![
            "language".to_string(),
            "file_path".to_string(),
            "start_row".to_string(),
            "start_col".to_string(),
            "end_row".to_string(),
            "end_col".to_string(),
            "node_name".to_string(),
            "node_type".to_string(),
            "is_broken".to_string(),
            "aloc".to_string(),
            "eloc".to_string(),
            "cloc".to_string(),
            "dcloc".to_string(),
            "noi".to_string(),
            "noc".to_string(),
            "nom".to_string(),
            "cc".to_string(),
            "pc".to_string(),
        ]); // Add header row
        for metric in &self.metrics.metrics {
            data.push(vec![
                metric.language.clone(),
                metric.file_path.clone(),
                metric.start_row.to_string(),
                metric.start_col.to_string(),
                metric.end_row.to_string(),
                metric.end_col.to_string(),
                metric.node_name.clone(),
                metric.node_type.clone(),
                metric.is_broken.to_string(),
                metric.aloc.to_string(),
                metric.eloc.to_string(),
                metric.cloc.to_string(),
                metric.dcloc.to_string(),
                metric.noi.to_string(),
                metric.noc.to_string(),
                metric.nom.to_string(),
                metric.cc.to_string(),
                metric.pc.to_string(),
            ]);
        }

        data
    }
}
