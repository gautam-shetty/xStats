use crate::metrics::CodeMetrics;
use crate::parser::TSParsers;
use crate::utils::{save_to_csv, save_to_json, traverse_path};
use indicatif::{ProgressBar, ProgressStyle};

pub struct XStats {
    target_path: String,
    output_path: String,
    parsers: TSParsers,
    metrics: CodeMetrics,
}

impl XStats {
    pub fn new(target_path: String, output_path: String) -> Self {
        let parsers = TSParsers::new();
        Self {
            target_path,
            output_path,
            parsers,
            metrics: CodeMetrics::new(),
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
