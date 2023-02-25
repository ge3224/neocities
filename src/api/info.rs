use std::error::Error;
use reqwest::header::AUTHORIZATION;
use reqwest::Response;
use serde_derive::Deserialize;
use serde_derive::Serialize;
use serde_json::Value;
use crate::api::API_URL;
use super::credentials::{Auth, Credentials};

/// Contains data received from Neocities in response to a request to `/api/info`
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SiteInfo {
    /// A status message
    pub result: String,
    /// Information about a Neocities website
    pub info: Info,
}

/// Information about a Neocities website
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Info {
    /// A Neocities user's sitename, aka username
    pub sitename: String,
    /// Total views
    pub views: i64,
    /// Total hits
    pub hits: i64,
    /// A timestamp for the site's creation
    #[serde(rename = "created_at")]
    pub created_at: String,
    /// A timestamp for the sites most recent update
    #[serde(rename = "last_updated")]
    pub last_updated: String,
    /// A domain for the website, if configured
    pub domain: Value,
    /// Tags a Neocities user sets about the site
    pub tags: Vec<String>,
    /// A hash associated with the InterPlanetary File System protocol
    #[serde(rename = "latest_ipfs_hash")]
    pub latest_ipfs_hash: Value,
}

/// Prepares and sends a request for information about a specified Neocities website. It awaits a
/// response and returns either SiteInfo or an error.
#[tokio::main]
pub async fn api_call(
    cred: Credentials,
    args: &Vec<String>,
) -> Result<SiteInfo, Box<dyn std::error::Error>> {
    let url: String;
    let mut api_key: Option<String> = None;

    // give precedence to args so a user can run `neocities info [sitename]` to lookup other
    // websites, although environment variables have been set
    if args.len() > 0 {
        url = format!("https://{}/info?sitename={}", API_URL, args[0]);
    } else {
        let auth = Auth::authenticate(cred, String::from("info"), None);

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
            let body = res.json::<SiteInfo>().await?;
            Ok(body)
        }
        _ => {
            let e: Box<dyn std::error::Error> =
                format!("The Neocities API could not find site '{}'.", args[0]).into();
            Err(e)
        }
    }
}
