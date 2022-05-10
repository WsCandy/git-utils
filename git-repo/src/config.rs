use std::env;

pub struct Config {
    pub remote: String,
}

impl Config {
    pub fn new(mut args: env::Args) -> Result<Config, &'static str> {
        args.next();

        let remote = match args.next() {
            Some(v) => v,
            None => String::from("origin"),
        };

        Ok(Config { remote })
    }
}
