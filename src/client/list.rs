use super::command::Executable;

pub const LIST: &'static str = "list";

pub struct List {
    key: String,
    usage: String,
    short: String,
    long: String,
}

impl List {
    pub fn new() -> List {
        List {
            key: String::from(LIST),
            usage: String::from(LIST),
            short: String::from("List files on Neocities"),
            long: String::from("List files in your Neocities website"),
        }
    }
}

impl Executable for List {
    fn run(&self, cred: crate::Credentials, args: Vec<String>) -> Result<(), &'static str> {
        println!("List's implementation of Executable: {:?}", args);
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
