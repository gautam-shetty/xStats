use clap::Parser;
use xstats::core;

#[derive(Parser)]
#[clap(version = "1.0", author = "Gautam Shetty")]
struct Opts {
    #[clap(short = 't', long = "target")]
    target: String,

    #[clap(short = 'o', long = "output")]
    output: String,
}

fn main() {
    let opts: Opts = Opts::parse();

    let mut designitex = core::XStats::new(opts.target, opts.output);
    designitex.run();
    designitex.save_data();
}
