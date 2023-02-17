use super::command::Executable;

pub const DEL: &'static str = "delete";

pub struct Delete {
    key: String,
    usage: String,
    short: String,
    long: String,
}

impl Delete {
    pub fn new() -> Delete {
        Delete {
            key: String::from(DEL),
            usage: String::from(format!("{DEL} <filename> [<another filename>]")),
            short: String::from("Delete files from Neocities"),
            long: String::from("Delete files from your Neocities website"),
        }
    }
}

impl Executable for Delete {
    fn run(&self, args: Vec<String>) -> Result<(), &'static str> {
        println!("Delete implementation of Executable: {:?}", args);
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
