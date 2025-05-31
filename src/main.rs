use clap::Parser;
use xstats::core;

#[derive(Parser)]
#[clap(version = "0.1.0", author = "Gautam Shetty")]
struct Options {
    #[clap(short = 't', long = "target")]
    target: String,

    #[clap(short = 'o', long = "output")]
    output: String,

    #[clap(short = 'a', long = "all-commits", default_value = "false")]
    all_commits: bool,

    #[clap(long = "format", default_value = "json")]
    format: String,
}

fn main() {
    let options: Options = Options::parse();

    let mut xstats = core::XStats::new(options.target, options.output);

    if options.all_commits {
        xstats.run_multi_commit();
        xstats.save_metrics_map(options.format.as_str());
    } else {
        xstats.run_default();
        xstats.save_metrics(options.format.as_str());
        xstats.save_tdg();
    }
}
