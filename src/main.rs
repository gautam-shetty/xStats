use clap::Parser;
use xstats::core;

#[derive(Parser)]
#[clap(version = "1.0", author = "Gautam Shetty")]
struct Opts {
    #[clap(short = 't', long = "target")]
    target: String,

    #[clap(short = 'o', long = "output")]
    output: String,

    #[clap(short = 'f', long = "format", default_value = "json")]
    format: String,
}

fn main() {
    let opts: Opts = Opts::parse();

    let mut xstats = core::XStats::new(opts.target, opts.output);
    xstats.run();

    match opts.format.as_str() {
        "json" => xstats.save_data_as_json(),
        "csv" => xstats.save_data_as_csv(),
        _ => eprintln!("Unsupported format: {}", opts.format),
    }
}
