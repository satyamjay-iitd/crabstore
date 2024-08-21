mod allocator;
mod runner;
mod store;

use std::path::PathBuf;

use clap::Parser;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct CliArgs {
    #[arg(short = 's', long)]
    socket_path: PathBuf,

    #[arg(short = 'm', long, default_value_t = 1000000)]
    sys_memory: i64,
}

fn main() {
    env_logger::init();
    let args = CliArgs::parse();

    let runner = runner::Runner::new(args.socket_path, args.sys_memory);

    runner.start();
}
