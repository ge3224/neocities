use std::error::Error;
use reqwest::header::AUTHORIZATION;
use reqwest::Response;
use serde_derive::Deserialize;
use serde_derive::Serialize;
use serde_json::Value;
use crate::api::API_URL;
use super::credentials::{Auth, Credentials};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SiteInfo {
    pub result: String,
    pub info: Info,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Info {
    pub sitename: String,
    pub views: i64,
    pub hits: i64,
    #[serde(rename = "created_at")]
    pub created_at: String,
    #[serde(rename = "last_updated")]
    pub last_updated: String,
    pub domain: Value,
    pub tags: Vec<String>,
    #[serde(rename = "latest_ipfs_hash")]
    pub latest_ipfs_hash: Value,
}

#[tokio::main]
pub async fn api_call(
    cred: Credentials,
    args: &Vec<String>,
) -> Result<SiteInfo, Box<dyn std::error::Error>> {
    let url: String;
    let mut api_key: Option<String> = None;

    // give precedence to args so a user can run `neocities info [sitename]` to lookup other
    // websites, although her or she has set environment variables.
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
