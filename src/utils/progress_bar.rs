use indicatif::{MultiProgress, ProgressBar, ProgressStyle};

pub struct CustomProgressBar {
    pub mp: MultiProgress,
}

impl CustomProgressBar {
    pub fn new() -> Self {
        let multi_progress = MultiProgress::new();
        CustomProgressBar { mp: multi_progress }
    }

    pub fn generate_commits_bar(&self, length: u64) -> ProgressBar {
        let pb = self.mp.add(ProgressBar::new(length));
        pb.set_style(
            ProgressStyle::default_bar()
                .template("{bar:100} [{pos}/{len}]\n[{elapsed_precise}] Processing commit: {msg}")
                .expect("Failed to set progress bar style"),
        );
        pb
    }

    pub fn generate_files_bar(&self, length: u64) -> ProgressBar {
        let pb = self.mp.add(ProgressBar::new(length));
        pb.set_style(
            ProgressStyle::default_bar()
                .template(
                    "{bar:100} [{pos}/{len}]\n[{elapsed}] {spinner:.cyan} Processing file: {msg}",
                )
                .expect("Failed to set progress bar style"),
        );
        pb
    }
}
