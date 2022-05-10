use std::error::Error;
use std::{env, process};

use git2::Repository;

mod config;
mod remotes;

fn main() {
    let args = env::args();
    let config = config::Config::new(args).unwrap_or_else(|e| {
        eprintln!("Config Error: {}", e);
        process::exit(1)
    });

    if let Err(e) = run(config) {
        eprintln!("Application Error: {}", e);
        process::exit(1)
    }
}

pub fn run(config: config::Config) -> Result<(), Box<dyn Error>> {
    let repository = Repository::open(".")?;
    let remotes = remotes::get_remotes(&repository)?;
    let remote = remotes.get(&config.remote);

    match remote {
        Some(value) => remotes::open_remote(value),
        None => {
            let message = format!("unable to find remote location '{}'", config.remote);
            Err(Box::from(message))
        }
    }
}
