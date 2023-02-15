use super::{Args, Credentials};

pub struct Config {
    pub args: Result<Args, &'static str>,
    credentials: Credentials,
}

impl Config {
    pub fn build(input: &[String]) -> Result<Config, &'static str> {
        let args = Args::build(&input);

        Ok(Config {
            args,
            credentials: Credentials::new(),
        })
    }
}
