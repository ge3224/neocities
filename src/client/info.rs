use super::command::Executable;

pub const INFO: &'static str = "info";

pub struct Info {
    key: String,
    usage: String,
    short: String,
    long: String,
}

impl Info {
    pub fn new() -> Info {
        Info {
            key: String::from("info"),
            usage: String::from("info [sitename]"),
            short: String::from("Info about Neocities websites"),
            long: String::from("Info about your Neocities website, or somebody else's"),
        }
    }
}

impl Executable for Info {
    fn run(&self, args: Vec<String>) -> Result<(), &'static str> {
        println!("Info' implementation of Executable: {:?}", args);
        Ok(())
    }

    fn get_key(&self) -> &str {
        self.key.as_str()
    }

    fn get_usage(&self) -> &str {
        self.usage.as_str()
    }

    fn get_short_desc(&self) -> &str {
        self.short.as_str()
    }

    fn get_long_desc(&self) -> &str {
        self.long.as_str()
    }
}