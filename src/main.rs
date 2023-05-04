use clap::Parser;
use media_tools::cli::cli::Cli;

fn main() {
    let options = Cli::parse();

    match options.execute() {
        Ok(_) => {}
        Err(e) => {
            println!("Error: {:?}", e);
        }
    }
}
