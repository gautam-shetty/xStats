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

    #[clap(long = "from", default_value = "false")]
    from: bool, // Not used in the current implementation yet

    #[clap(long = "to", default_value = "false")]
    to: bool, // Not used in the current implementation yet

    #[clap(long = "format", default_value = "json")]
    format: String,
}

fn cli_checks(options: &Options) {
    if options.all_commits && (options.from || options.to) {
        eprintln!("Error: --all-commits cannot be used with --from or --to");
        std::process::exit(1);
    }
}

fn main() {
    let options: Options = Options::parse();
    cli_checks(&options);

    let mut xstats = core::XStats::new(options.target, options.output, options.all_commits);
    xstats.run();

    match options.format.as_str() {
        "json" => xstats.save_data_as_json(),
        "csv" => xstats.save_data_as_csv(),
        _ => eprintln!("Unsupported format: {}", options.format),
    }

    if options.all_commits {
        xstats.print_commits();
    }
}
