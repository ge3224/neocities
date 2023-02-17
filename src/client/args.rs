/// Args contains a command and its params
pub struct Args {
    pub command: Option<String>,
    pub params: Vec<String>,
}

impl Args {
    /// build an instance of Args by parsing command line arguments passed in as an
    /// array of Strings
    pub fn build(inputs: &[String]) -> Args {
        let mut inputs_iter = inputs.iter();

        // skip the first argument, the name of the binary
        inputs_iter.next();

        // isolate the second argument, the <command>
        let mut command: Option<String> = None;
        if let Some(s) = inputs_iter.next() {
            command = Some(s.clone());
        }

        // parse variable number of additional arguments
        let mut params = vec![];
        for param in inputs_iter {
            params.push(param.clone());
        }

        Args {
            command,
            params,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Args;

    #[test]
    fn no_args() {
        let args = Args::build(vec!["neocities".to_string()].as_ref());
        assert_eq!(args.command.is_none(), true);
        assert_eq!(args.params.len(), 0);
    }

    #[test]
    fn with_args() {
        let str = "neocities upload foo.html bar.js images/baz.png";

        let input: Vec<String> = str.split(" ").map(|x| x.to_string()).collect();
        let args = Args::build(&input);

        assert_eq!(args.command.unwrap(), "upload");
        assert_eq!(args.params.len(), 3);
        assert_eq!(args.params[0], "foo.html");
        assert_eq!(args.params[1], "bar.js");
    }
}
