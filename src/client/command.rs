pub type Exec = Box<dyn Fn(Vec<String>) -> Result<(), &'static str>>;

/// Command contains a function to run, a flagset, and usage instructions
pub struct Command {
    run: Exec,
    key: String,
    usage: String,
    short_desc: String,
    long_desc: String,
}

impl Command {
    pub fn new(
        run_fn: Exec,
        key: String,
        usage: String,
        short_desc: String,
        long_desc: String,
    ) -> Command {
        Command {
            run: run_fn,
            key,
            usage,
            short_desc,
            long_desc,
        }
    }

    pub fn get_name(&self) -> &String {
        &self.key
    }

    pub fn get_usage(&self) -> &String {
        &self.usage
    }

    pub fn get_short_desc(&self) -> &String {
        &self.short_desc
    }

    pub fn get_long_desc(&self) -> &String {
        &self.long_desc
    }

    pub fn call(&self, args: Vec<String>) -> Result<(), &'static str> {
        (self.run)(args)
    }
}

#[cfg(test)]
mod tests {
    use super::Command;

    #[test]
    fn create_command() {
        let cmd = Command {
            run: Box::new(|_args| Ok(())),
            key: String::from("foo"),
            usage: String::from("bar"),
            short_desc: String::from("baz"),
            long_desc: String::from("foo bar baz"),
        };

        assert_eq!(cmd.get_name(), "foo");
        assert_eq!(cmd.get_usage(), "bar");
        assert_eq!(cmd.get_short_desc(), "baz");
        assert_eq!(cmd.get_long_desc(), "foo bar baz");

        assert_eq!(cmd.call(vec![String::from("arg")]).is_ok(), true);
    }

    #[test]
    fn bad_run_fn() {
        let cmd = Command {
            run: Box::new(|_args| Err("bad run function")),
            key: String::from("foo"),
            usage: String::from("bar"),
            short_desc: String::from("baz"),
            long_desc: String::from("foo bar baz"),
        };

        assert_eq!(cmd.call(vec![String::from("arg")]), Err("bad run function"));
    }
}
