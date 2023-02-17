use super::Args;

type Exec = Box<dyn Fn(Args) -> Result<(), &'static str>>;

/// Command contains a function to run, a flagset, and usage instructions
pub struct Command {
    run: Exec,
    key: String,
    usage: String,
    short: String,
    long: String,
}

impl Command {
    pub fn get_name(&self) -> &String {
        &self.key
    }

    pub fn get_usage(&self) -> &String {
        &self.usage
    }

    pub fn get_short_desc(&self) -> &String {
        &self.short
    }

    pub fn get_long_desc(&self) -> &String {
        &self.long
    }

    pub fn call(&self, args: Args) -> Result<(), &'static str> {
        (self.run)(args)
    }
}

#[cfg(test)]
mod tests {
    use crate::client::Args;

    use super::Command;

    #[test]
    fn create_command() {
        let cmd = Command {
            run: Box::new(|_args| Ok(())),
            key: String::from("foo"),
            usage: String::from("bar"),
            short: String::from("baz"),
            long: String::from("foo bar baz"),
        };

        assert_eq!(cmd.get_name(), "foo");
        assert_eq!(cmd.get_usage(), "bar");
        assert_eq!(cmd.get_short_desc(), "baz");
        assert_eq!(cmd.get_long_desc(), "foo bar baz");

        let args = vec![String::from("arg")];
        assert_eq!(cmd.call(Args::build(&args)).is_ok(), true);
    }

    #[test]
    fn bad_run_fn() {
        let cmd = Command {
            run: Box::new(|_args| Err("bad run function")),
            key: String::from("foo"),
            usage: String::from("bar"),
            short: String::from("baz"),
            long: String::from("foo bar baz"),
        };

        let args = vec![String::from("arg")];
        assert_eq!(cmd.call(Args::build(&args)), Err("bad run function"));
    }
}
