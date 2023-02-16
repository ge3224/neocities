use super::Args;
use crate::Credentials;

pub struct Config {
    pub args: Args,
    credentials: Credentials,
}

impl Config {
    pub fn build(input: &[String]) -> Result<Config, &'static str> {

        let args = Args::build(&input);

        let credentials = Credentials::new();

        Ok(Config {
            args,
            credentials,
        })
    }
}
