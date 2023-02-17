use super::{config, Args};

type Exec = Box<dyn Fn(Args) -> Result<(), &'static str>>;

/// Command contains a function to run, a flagset, and usage instructions
struct Command {
    run: Exec,
    key: String,
    usage: String,
    short: String,
    long: String,
}

impl Command {
    pub fn new(f: Exec, key: String, usage: String, short: String, long: String) -> Command {
        Command {
            run: f,
            key,
            usage,
            short,
            long,
        }
    }

    pub fn get_key(&self) -> &String {
        &self.key
    }

    pub fn get_usage(&self) -> &String {
        &self.usage
    }

    pub fn get_short_info(&self) -> &String {
        &self.short
    }

    pub fn get_long_info(&self) -> &String {
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
    fn instantiate() {
        let input = "foo bar baz";
        let props: Vec<&str> = input.split(" ").collect();
        let args = vec![String::from("arg")];

        let cmd = Command::new(
            Box::new(|_args| Ok(())),
            String::from(props[0]),
            String::from(props[1]),
            String::from(props[2]),
            String::from(input),
        );

        assert_eq!(cmd.get_key(), "foo");
        assert_eq!(cmd.get_usage(), "bar");
        assert_eq!(cmd.get_short_info(), "baz");
        assert_eq!(cmd.get_long_info(), "foo bar baz");
        assert_eq!(cmd.call(Args::build(&args)).is_ok(), true);

        // throw error
        let cmd = Command::new(
            Box::new(|_args| Err("bad run function")),
            String::from(props[0]),
            String::from(props[1]),
            String::from(props[2]),
            String::from(input),
        );

        assert_eq!(cmd.call(Args::build(&args)), Err("bad run function"));
    }
}
