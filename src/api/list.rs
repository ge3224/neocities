use super::credentials::{Auth, Credentials, QueryString};
use reqwest::{header::AUTHORIZATION, Response};
use serde_derive::Deserialize;
use serde_derive::Serialize;
use std::error::Error;

/// Contains data from the response body of the Neocities' `/api/list` endpoint.
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FileList {
    /// A status message
    pub result: String,
    /// An array of file data
    pub files: Vec<File>,
}

/// Contains file data found for a specific path in a Neocities user's website
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
    /// A datestamp of the file's most recent modification
    #[serde(rename = "updated_at")]
    pub updated_at: String,
    /// A checksum for the file
    #[serde(rename = "sha1_hash")]
    pub sha1_hash: Option<String>,
}

/// Prepares and sends a list request to the Neocities API. It awaits a response and returns
/// a Result of a FileList or an error.
#[tokio::main]
pub async fn api_call(cred: Credentials, path: Option<String>) -> Result<FileList, Box<dyn Error>> {
    let mut query_string: Option<QueryString> = None;

    if let Some(p) = path {
        query_string = Some(QueryString {
            key: String::from("path"),
            value: format!("{}", p),
        });
    }

    let auth = Auth::authenticate(cred, String::from("list"), query_string);

    let url: String;
    let api_key: Option<String>;

    match auth {
        Err(e) => {
            let err: Box<dyn Error> = format!("problem authenticating credentials: {e}").into();
            return Err(err);
        }
        Ok(a) => {
            url = a.url;
            api_key = a.api_key;
        }
    }

    let req = reqwest::Client::new();
    let res: Response;

    if let Some(k) = api_key {
        res = req
            .get(url.as_str())
            .header(AUTHORIZATION, format!("Bearer {}", k))
            .send()
            .await?;
    } else {
        res = req.get(url.as_str()).send().await?;
    }

    match res.status() {
        reqwest::StatusCode::OK => {
            let body = res.json::<FileList>().await?;
            Ok(body)
        }
        _ => {
            let e: Box<dyn std::error::Error> =
                String::from("Bad request to the Neocities API.").into();
            Err(e)
        }
    }
}
