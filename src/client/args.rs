/// Args contains a command and its params
pub struct Args {
    pub command: Option<String>,
    pub file_paths: Vec<String>,
}

impl Args {
    /// build an instance of Args by parsing command line arguments passed to neocities
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
            file_paths: params,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Args;

    #[test]
    fn no_args() {
        let input = vec![String::from("neocities")];
        let args = Args::build(&input);
        assert_eq!(args.command.is_none(), true);
        assert_eq!(args.file_paths.len(), 0);
    }

    #[test]
    fn one_arg() {
        let input = vec![String::from("neocities"), String::from("help")];
        let args = Args::build(&input);
        assert_eq!(args.command.unwrap(), "help");
        assert_eq!(args.file_paths.len(), 0);
    }

    #[test]
    fn two_args() {
        let input = vec![
            String::from("neocities"),
            String::from("upload"),
            String::from("foo.html"),
        ];
        let args = Args::build(&input);
        assert_eq!(args.command.unwrap(), "upload");
        assert_eq!(args.file_paths.len(), 1);
        assert_eq!(args.file_paths[0], "foo.html");
    }

    #[test]
    fn three_args() {
        let input = vec![
            String::from("neocities"),
            String::from("upload"),
            String::from("foo.html"),
            String::from("bar.js"),
        ];
        let args = Args::build(&input);
        assert_eq!(args.command.unwrap(), "upload");
        assert_eq!(args.file_paths.len(), 2);
        assert_eq!(args.file_paths[0], "foo.html");
        assert_eq!(args.file_paths[1], "bar.js");
    }
}
