use std::env;

use crate::Credentials;

pub struct Config {
    pub cmd: String,
    pub file_path: String,
    credentials: Credentials,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let cmd = args[1].clone();
        let file_path = args[2].clone();
        
        Ok(Config {
            cmd,
            file_path,
            credentials: Credentials::new(),
        })
    }
}
