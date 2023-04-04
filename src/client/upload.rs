use super::command::Executable;
use crate::{
    api::{
        credentials::{Credentials, ENV_VAR_MSG},
        upload::{NcUpload, UploadResponse},
    },
    error::NeocitiesErr,
};

/// The string literal a user must type to run functionality in this module
pub const KEY: &'static str = "upload";

/// Uploads files to a Neocities user's site. The Neocities API allows a user to upload as many
/// files as desired, as long as the entire request stays within the disk space limit.
pub struct Upload {
    usage: String,
    short: String,
    long: String,
}

impl Upload {
    /// A constructor that returns an instance of `Upload`.
    pub fn new() -> Upload {
        Upload {
            usage: String::from(format!(
                "\x1b[1;32m{}\x1b[0m <filename> [<another filename>]",
                KEY
            )),
            short: String::from(DESC_SHORT),
            long: String::from(DESC),
        }
    }

    fn write(&self, msg: &str, mut writer: impl std::io::Write) -> Result<(), NeocitiesErr> {
        let output = format!("{}", msg);
        writer.write_all(output.as_bytes())?;
        Ok(())
    }

    fn parse_response(
        &self,
        res: UploadResponse,
        mut writer: impl std::io::Write,
    ) -> Result<(), NeocitiesErr> {
        let output = format!(
            "\x1b[93mStatus\x1b[0m: {} - {}\n",
            &res.result, &res.message
        );
        writer.write_all(output.as_bytes())?;
        Ok(())
    }
}

impl Executable for Upload {
    fn run(&self, args: Vec<String>) -> Result<(), NeocitiesErr> {
        let mut stdout = std::io::stdout();

        if args.len() < 1 {
            let output = format!("{}\nusage: {}\n", self.get_long_desc(), self.get_usage());
            self.write(output.as_str(), &mut stdout)?;
            return Ok(());
        }

        if Credentials::have_env_vars() != true {
            self.write(ENV_VAR_MSG, &mut stdout)?;
            return Ok(());
        }

        let data = NcUpload::fetch(args)?;
        self.parse_response(data, &mut stdout)?;

        Ok(())
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

const DESC_SHORT: &'static str = "Upload files to Neocities";

const DESC: &'static str = "Upload files to your Neocities website";

#[cfg(test)]
mod tests {
    use super::{Upload, DESC, DESC_SHORT, KEY};
    use crate::{api::upload::UploadResponse, client::command::Executable, error::NeocitiesErr};

    #[test]
    fn get_usage_method() {
        let up = Upload::new();
        assert_eq!(up.get_usage().contains(KEY), true);
    }

    #[test]
    fn get_long_desc_method() {
        let u = Upload::new();
        assert_eq!(u.get_long_desc(), DESC);
    }

    #[test]
    fn get_short_desc_method() {
        let u = Upload::new();
        assert_eq!(u.get_short_desc(), DESC_SHORT);
    }

    #[test]
    fn write_method() -> Result<(), NeocitiesErr> {
        let u = Upload::new();
        let mut output = Vec::new();
        u.write("foo", &mut output)?;

        let s = String::from_utf8(output)?;

        assert_eq!(s.contains("foo"), true);

        Ok(())
    }

    #[test]
    fn parse_response_method() -> Result<(), NeocitiesErr> {
        let mock_res = UploadResponse {
            result: String::from("foo"),
            error_type: None,
            message: String::from("bar"),
        };

        let u = Upload::new();
        let mut output = Vec::new();
        u.parse_response(mock_res, &mut output)?;

        let s = String::from_utf8(output)?;

        assert_eq!(s.contains("foo"), true);
        assert_eq!(s.contains("bar"), true);

        Ok(())
    }
}
