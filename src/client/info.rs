use std::io::Write;

use super::command::Executable;
use crate::{
    api::{
        credentials::{Credentials, ENV_VAR_MSG},
        info::{InfoResponse, NcInfo},
    },
    error::NeocitiesErr,
};

/// The string literal a user must type to run functionality in this module
pub const KEY: &'static str = "info";

/// Retreives public information about any Neocities user's web site. Site authorization is not
/// needed if the user provides a sitename argument. Note that the sitename is the same as a
/// username.
pub struct Info {
    usage: String,
    short: String,
    long: String,
}

impl Info {
    /// A constructor that returns an instance of `Info`
    pub fn new() -> Info {
        Info {
            usage: String::from(format!("\x1b[1;32m{KEY}\x1b[0m [sitename]")),
            short: String::from(DESC_SHORT),
            long: String::from(DESC),
        }
    }

    fn write(
        &self,
        key: &str,
        value: &str,
        mut writer: impl std::io::Write,
    ) -> Result<(), NeocitiesErr> {
        let output = format!("\x1b[1;92m{0: <20}\x1b[0m {1:}\n", key, value);
        writer.write_all(output.as_bytes())?;
        Ok(())
    }

    fn parse_response(
        &self,
        ir: InfoResponse,
        mut writer: impl std::io::Write,
    ) -> Result<(), NeocitiesErr> {
        self.write("sitename", &ir.info.sitename, &mut writer)?;

        self.write("views", &ir.info.views.to_string(), &mut writer)?;

        self.write("hits", &ir.info.hits.to_string(), &mut writer)?;

        self.write("created_at", &ir.info.created_at, &mut writer)?;

        self.write("last_updated", &ir.info.last_updated, &mut writer)?;

        let domain_value: &str;

        if let serde_json::Value::String(v) = &ir.info.domain {
            domain_value = v.as_str();
        } else {
            domain_value = "null";
        }

        self.write("domain", domain_value, &mut writer)?;

        self.write("tags", format!("{:?}", &ir.info.tags).as_str(), &mut writer)?;

        let hash_value: &str;
        if let serde_json::Value::String(v) = &ir.info.latest_ipfs_hash {
            hash_value = v.as_str();
        } else {
            hash_value = "null";
        }

        self.write("latest_ipfs_hash", &hash_value, &mut writer)?;
        Ok(())
    }
}

const DESC: &'static str = "Info about your Neocities website, or somebody else's";

const DESC_SHORT: &'static str = "Info about Neocities websites";

impl Executable for Info {
    fn run(&self, args: Vec<String>) -> Result<(), NeocitiesErr> {
        let mut stdout = std::io::stdout();

        if args.len() < 1 {
            if Credentials::have_env_vars() != true {
                stdout.write_all(ENV_VAR_MSG.as_bytes())?;
                return Ok(());
            }
        }

        let data = NcInfo::fetch(&args)?;
        self.parse_response(data, stdout)?;
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

#[cfg(test)]
mod tests {
    use super::{Info, DESC, DESC_SHORT, KEY};
    use crate::{api::info, client::command::Executable, error::NeocitiesErr};

    #[test]
    fn get_desc_method() {
        let i = Info::new();
        assert_eq!(i.get_long_desc(), DESC);
    }

    #[test]
    fn get_short_desc_method() {
        let i = Info::new();
        assert_eq!(i.get_short_desc(), DESC_SHORT);
    }

    #[test]
    fn get_usage_method() {
        let i = Info::new();
        assert_eq!(i.get_usage().contains(KEY), true);
    }

    #[test]
    fn parse_response_data() -> Result<(), NeocitiesErr> {
        let date = "Tue, 04 April 2023 18:49:21 +0000";
        let sitename = "foo";
        let mock_info = info::Info {
            sitename: String::from(sitename),
            views: 100,
            hits: 1000,
            created_at: String::from(date),
            last_updated: String::from(date),
            domain: serde_json::Value::Null,
            tags: Vec::new(),
            latest_ipfs_hash: serde_json::Value::Null,
        };

        let mock = info::InfoResponse {
            result: String::from("success"),
            info: mock_info,
        };
        let i = Info::new();
        let mut output = Vec::new();
        i.parse_response(mock, &mut output)?;

        let s = String::from_utf8(output)?;
        assert_eq!(s.contains(sitename), true);
        assert_eq!(s.contains("100"), true);
        assert_eq!(s.contains(date), true);

        Ok(())
    }
}
