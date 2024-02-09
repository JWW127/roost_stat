use errors::StatsError;
use src_stats::get_summary_src_stats;
use std::path::PathBuf;
use structopt::StructOpt;
mod errors;
mod src_stats;

#[derive(Debug, StructOpt)]
#[structopt(name = "roost_stat", about = "Generate Statistics on Rust Projects")]
struct Opt {
    #[structopt(name = "source directory", parse(from_os_str))]
    in_dir: PathBuf,
    #[structopt(name = "mode", short)]
    mode: String,
}

fn main() -> Result<(), StatsError> {
    let opt = Opt::from_args(); //builds struct Opt form command line args
    let mode = &opt.mode[..];
    match mode {
        "src" => {
            let stats = get_summary_src_stats(&opt.in_dir)?;
            println!("Summary stats: {:?}", stats);
        }
        _ => println!("Sorry, no stats found"),
    }
    Ok(())
}
