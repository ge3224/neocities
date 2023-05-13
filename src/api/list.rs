use super::credentials::{Auth, Credentials, QueryString};
use super::http::{get_request, HttpRequestInfo};
use crate::client::list;
use crate::error::NeocitiesErr;
use chrono::{DateTime, FixedOffset, Utc};
use serde_derive::{Deserialize, Serialize};

/// Handles the requesting of a list of a Neocities website's directory contents using the
/// following Neocities API endpoint `/api/list`
pub struct NcList {}

/// Contains data received from Neocities in response to a request to `/api/list`.
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ListResponse {
    /// A status message
    pub result: String,
    /// An array of file data
    pub files: Vec<File>,
}

/// Contains file data found for a specific path on a Neocities user's website
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct File {
    /// The location of the file on the server
    pub path: String,
    /// A boolean indicating whether the file is a directory or not
    #[serde(rename = "is_directory")]
    pub is_directory: bool,
    /// The byte size of the file
    pub size: Option<i64>,
    /// A timestamp for the file's most recent modification
    #[serde(rename = "updated_at")]
    pub updated_at: String,
    /// A checksum for the file
    #[serde(rename = "sha1_hash")]
    pub sha1_hash: Option<String>,
}

impl File {
    /// parses the updated_at field and returns a chrono::DateTime object is no error occurs.
    pub fn parse_timestamp(&self) -> Result<DateTime<FixedOffset>, NeocitiesErr> {
        match DateTime::parse_from_rfc2822(self.updated_at.as_str()) {
            Ok(d) => Ok(d),
            Err(e) => Err(NeocitiesErr::ParseDateError(e)),
        }
    }
}

impl NcList {
    fn request_info(file_path: Option<String>) -> Result<HttpRequestInfo, NeocitiesErr> {
        let cred = Credentials::new();

        let mut query_string: Option<QueryString> = None;
        let url: String;
        let api_key: Option<String>;

        if let Some(p) = file_path {
            query_string = Some(QueryString {
                key: String::from("path"),
                value: format!("{}", p),
            });
        }

        let auth = Auth::authenticate(cred, list::KEY, query_string);

        match auth {
            Err(e) => return Err(NeocitiesErr::HttpRequestError(e.into())),
            Ok(a) => {
                url = a.url;
                api_key = a.api_key;
            }
        }

        let pk = HttpRequestInfo {
            uri: url,
            api_key,
            body: None,
            multipart: None,
        };
        Ok(pk)
    }

    fn to_list_response(value: serde_json::Value) -> Result<ListResponse, NeocitiesErr> {
        match serde_json::from_value(value) {
            Ok(res) => Ok(res),
            Err(e) => return Err(NeocitiesErr::SerdeDeserializationError(e)),
        }
    }

    /// Prepares and sends a request to the `api/list` endpoint of the Neocities API. It awaits a
    /// response and returns either a FileList or an error.
    pub fn fetch(path: Option<String>) -> Result<ListResponse, NeocitiesErr> {
        // get http path and api_key for headers
        let pk = NcList::request_info(path)?;
        let res = get_request(pk.uri, pk.api_key)?;
        let ir = NcList::to_list_response(res)?;
        Ok(ir)
    }
}

#[cfg(test)]
mod tests {
    use super::{ListResponse, NcList};
    use crate::api::credentials::ENV_KEY;
    use chrono::{DateTime, FixedOffset, TimeZone, Utc};
    use std::env;

    #[test]
    fn list_request_path() {
        let preserve_key = env::var(ENV_KEY);

        env::set_var(ENV_KEY, "foo");

        let mock_args = String::from("bar");
        let pk = NcList::request_info(Some(mock_args)).unwrap();
        assert_eq!(pk.api_key.unwrap(), "foo");
        assert_eq!(pk.uri, "https://neocities.org/api/list?path=bar");

        // reset environment var
        match preserve_key {
            Ok(v) => env::set_var(ENV_KEY, v),
            _ => env::remove_var(ENV_KEY),
        }
    }

    #[test]
    fn value_to_list_response() {
        let mock_str = r#"
        {
          "result": "success",
          "files": [
            {
              "path": "index.html",
              "is_directory": false,
              "size": 1023,
              "updated_at": "Sat, 13 Feb 2016 03:04:00 -0000",
              "sha1_hash": "c8aac06f343c962a24a7eb111aad739ff48b7fb1"
            },
            {
              "path": "not_found.html",
              "is_directory": false,
              "size": 271,
              "updated_at": "Sat, 13 Feb 2016 03:04:00 -0000",
              "sha1_hash": "cfdf0bda2557c322be78302da23c32fec72ffc0b"
            },
            {
              "path": "images",
              "is_directory": true,
              "updated_at": "Sat, 13 Feb 2016 03:04:00 -0000"
            },
            {
              "path": "images/cat.png",
              "is_directory": false,
              "size": 16793,
              "updated_at": "Sat, 13 Feb 2016 03:04:00 -0000",
              "sha1_hash": "41fe08fc0dd44e79f799d03ece903e62be25dc7d"
            }
          ]
        }"#;

        let v: serde_json::Value = serde_json::from_str(mock_str).unwrap();
        let ls_res: ListResponse = NcList::to_list_response(v).unwrap();

        assert_eq!(ls_res.result, "success");
        assert_eq!(ls_res.files.len(), 4);
        assert_eq!(ls_res.files[0].path, "index.html");
        assert_eq!(
            ls_res.files[1].sha1_hash.as_ref().unwrap(),
            "cfdf0bda2557c322be78302da23c32fec72ffc0b"
        );

        let dt = Utc.with_ymd_and_hms(2016, 02, 13, 03, 4, 00).unwrap();
        let fixed_dt = dt.with_timezone(&FixedOffset::west_opt(0).unwrap());
        assert_eq!(ls_res.files[0].parse_timestamp().unwrap(), fixed_dt.clone());
    }
}
