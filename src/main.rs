use std::io::Write;

use clap::Parser;
use log::debug;
use media_tools::cli::cli::Cli;

fn main() {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info"))
        .format(|buf, record| writeln!(buf, "{}", record.args()))
        .init();

    let options = Cli::parse();

    debug!(
        "Current working directory: {}",
        std::env::current_dir().unwrap().display()
    );

    match options.execute() {
        Ok(_) => {}
        Err(e) => {
            println!("Error: {:?}", e);
        }
    }
}
