use crate::metrics::{CodeMetrics, CodeMetricsMap};
use crate::parser::{TSParsers, TSTreeHistory};
use crate::utils::{
    create_multi_commit_progress_bar, create_progress_bar, save_to_csv, save_to_json, traverse_path,
};
use git2::{Delta, DiffOptions, Repository, Sort, Tree};

pub struct XStats {
    target_path: String,
    output_path: String,
    parsers: TSParsers,
    history: TSTreeHistory,
    metrics_map: CodeMetricsMap,
}

impl XStats {
    pub fn new(target_path: String, output_path: String) -> Self {
        Self {
            target_path,
            output_path,
            parsers: TSParsers::new(),
            history: TSTreeHistory::new(),
            metrics_map: CodeMetricsMap::new(),
        }
    }

    pub fn run_default(&mut self) {
        match traverse_path(&self.target_path) {
            Ok(files) => {
                if files.is_empty() {
                    println!(
                        "No files to process in the target path {}",
                        self.target_path
                    );
                } else {
                    let file_count = files.len();
                    let prog_bar = create_progress_bar(file_count as u64);

                    let mut metrics = CodeMetrics::new();

                    // Analyze each file
                    for file in &files {
                        prog_bar.set_message(format!("Processing file: {}", file));
                        self.process_file(&mut metrics, file, None);
                        prog_bar.inc(1);
                    }

                    self.metrics_map.add_default_metrics(metrics);

                    prog_bar.finish_and_clear();
                }
            }
            Err(e) => {
                println!("{}", e);
            }
        }
    }

    pub fn run_multi_commit(&mut self) {
        // Open the Git repository at target_path
        let repo = match Repository::open(&self.target_path) {
            Ok(repo) => repo,
            Err(e) => {
                println!("Failed to open repository: {}", e);
                return;
            }
        };

        // Get the HEAD commit
        let mut revwalk = match repo.revwalk() {
            Ok(walk) => walk,
            Err(e) => {
                println!("Failed to create revwalk: {}", e);
                return;
            }
        };

        revwalk.push_head().expect("Failed to push HEAD to revwalk");
        revwalk
            .set_sorting(Sort::REVERSE)
            .expect("Failed to set sorting");

        let prog_bar = create_multi_commit_progress_bar();

        // Iterate through commits
        for oid_result in revwalk {
            if let Ok(oid) = oid_result {
                if let Ok(commit) = repo.find_commit(oid) {
                    prog_bar.set_message(format!("Commit: {}", commit.id()));
                    // Get the tree for the commit
                    if let Ok(tree) = commit.tree() {
                        let parent = if commit.parent_count() > 0 {
                            Some(
                                commit
                                    .parent(0)
                                    .expect("Failed to get parent commit")
                                    .tree()
                                    .expect("Failed to get parent tree"),
                            )
                        } else {
                            None
                        };
                        let mut code_metrics = CodeMetrics::new();
                        if let Err(e) = self.process_tree(&repo, &tree, &parent, &mut code_metrics)
                        {
                            println!("Failed to process tree: {}", e);
                        }

                        self.metrics_map
                            .add_metrics(commit.id().to_string(), code_metrics);
                    }

                    prog_bar.inc(1);
                }
            }
        }

        prog_bar.finish_and_clear();
    }

    // Process each file in a tree
    fn process_tree(
        &mut self,
        repo: &Repository,
        tree: &Tree,
        parent: &Option<Tree>,
        code_metrics: &mut CodeMetrics,
    ) -> Result<(), git2::Error> {
        let mut diff_opts = DiffOptions::new();
        let diff = repo
            .diff_tree_to_tree(parent.as_ref(), Some(tree), Some(&mut diff_opts))
            .expect("Failed to get diff");

        let stats = diff.stats().expect("Failed to get diff stats");
        let files_changed = stats.files_changed();

        diff.foreach(
            &mut |delta, _| {
                match delta.status() {
                    Delta::Added => {
                        if let Some(path) = delta.new_file().path() {
                            // Retrieve the file content for added or modified files
                            if let Ok(blob) = repo.find_blob(delta.new_file().id()) {
                                if let Ok(content) = std::str::from_utf8(blob.content()) {
                                    // Pass the file content to `process_file`
                                    self.process_file(
                                        code_metrics,
                                        path.to_string_lossy().as_ref(),
                                        Some(content.to_string()),
                                    );
                                } else {
                                    println!(
                                        "Failed to read content as UTF-8 for file: {}",
                                        path.to_string_lossy()
                                    );
                                }
                            } else {
                                println!(
                                    "Failed to find blob for file: {}",
                                    path.to_string_lossy()
                                );
                            }
                        }
                    }
                    Delta::Modified => {
                        if let Some(path) = delta.new_file().path() {
                            // Retrieve the file content for added or modified files
                            if let Ok(blob) = repo.find_blob(delta.new_file().id()) {
                                if let Ok(content) = std::str::from_utf8(blob.content()) {
                                    // Pass the file content to `process_file`
                                    self.process_file(
                                        code_metrics,
                                        path.to_string_lossy().as_ref(),
                                        Some(content.to_string()),
                                    );
                                } else {
                                    println!(
                                        "Failed to read content as UTF-8 for file: {}",
                                        path.to_string_lossy()
                                    );
                                }
                            } else {
                                println!(
                                    "Failed to find blob for file: {}",
                                    path.to_string_lossy()
                                );
                            }
                        }
                    }
                    Delta::Deleted => {
                        if let Some(path) = delta.old_file().path() {
                            self.history.delete_tree(&path.to_string_lossy());
                        }
                    }
                    _ => {}
                }
                true
            },
            None,
            None,
            None,
        )?;
        Ok(())
    }

    fn process_file(
        &mut self,
        code_metrics: &mut CodeMetrics,
        file: &str,
        content: Option<String>,
    ) {
        let result = self.parsers.generate_tree(&mut self.history, file, content);
        if let Some((language, tree, source_code)) = result {
            code_metrics.generate_root_metrics(
                &self.parsers,
                &source_code,
                &language.to_string(),
                &file.to_string(),
                &tree,
            );
            self.history.insert_tree(&file, tree);
        }
    }

    pub fn save_metrics_multi_commit(&self, format: &str) {
        let metrics_dir = format!("{}/metrics", self.output_path);
        std::fs::create_dir_all(&metrics_dir).expect("Failed to create metrics directory");

        for (key, _) in self.metrics_map.iter() {
            match format {
                "csv" => self.save_data_as_csv(Some(key)),
                "json" => self.save_data_as_json(Some(key)),
                _ => println!("Unsupported format: {}", format),
            }
        }
    }

    pub fn save_data_as_csv(&self, metric_key: Option<&str>) {
        let output_file = if let Some(key) = metric_key {
            format!("{}/metrics/{}.csv", self.output_path, key)
        } else {
            format!("{}/metrics.csv", self.output_path)
        };
        let data = self.get_metrics_data(metric_key);
        if save_to_csv(&output_file, data).is_ok() {
            println!("Code metrics saved at {}", output_file);
        } else {
            println!("Failed to save metrics to CSV");
        }
    }

    pub fn save_data_as_json(&self, metric_key: Option<&str>) {
        let output_file = if let Some(key) = metric_key {
            format!("{}//metrics/{}.json", self.output_path, key)
        } else {
            format!("{}/metrics.json", self.output_path)
        };
        let data: Vec<Vec<String>> = self.get_metrics_data(metric_key);
        if save_to_json(&output_file, data).is_ok() {
            println!("Code metrics saved at {}", output_file);
        } else {
            println!("Failed to save metrics to JSON");
        }
    }

    pub fn get_metrics_data(&self, name: Option<&str>) -> Vec<Vec<String>> {
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

        let metrics = if let Some(name) = name {
            self.metrics_map.get_metrics(&name.to_string())
        } else {
            self.metrics_map.get_default_metrics()
        };

        if let Some(metrics) = metrics {
            for metric in &metrics.metrics {
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
        }

        data
    }

    fn print_ts_history_info(&self) {
        let trees = &self.history.get_trees();
        let num_trees = self.history.num_trees();

        let history_size = std::mem::size_of_val(&trees);
        let entries_size: usize = trees
            .iter()
            .map(|(k, v)| std::mem::size_of_val(k) + std::mem::size_of_val(v))
            .sum();
        let total_size = history_size + entries_size;
        println!("Number of trees in TSHistory: {}", num_trees);
        println!("Size of the HashMap TSHistory: {} bytes", total_size);
    }
}
