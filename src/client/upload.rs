use super::command::Executable;

pub const UP: &'static str = "upload";

pub struct Upload {
    key: String,
    usage: String,
    short: String,
    long: String,
}

impl Upload {
    pub fn new() -> Upload {
        Upload {
            key: String::from(UP),
            usage: String::from(format!("{UP} <filename> [<another filename>]")),
            short: String::from("Upload files to Neocities"),
            long: String::from("Upload files to your Neocities website"),
        }
    }
}

impl Executable for Upload {
    fn run(&self, args: Vec<String>) -> Result<(), &'static str> {
        println!("Implementation of Executable for Upload");
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
