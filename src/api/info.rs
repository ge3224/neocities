use std::error::Error;

use reqwest::header::AUTHORIZATION;
use reqwest::Response;
use serde_derive::Deserialize;
use serde_derive::Serialize;
use serde_json::Value;
use url::form_urlencoded::byte_serialize;

use crate::api::API_URL;

use super::Credentials;

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
        // [sitename] argument url format
        url = format!("https://{}/info?sitename={}", API_URL, args[0]);
    } else {
        // check environment variables in the following order: (1) api key, (2) username and
        // password.
        if let Some(k) = cred.get_api_key() {
            // this key is added to the request header below
            api_key = Some(k.to_string());

            // api key url format
            url = format!("https://{}/info", API_URL);
        } else {
            let user = match cred.get_username() {
                Some(u) => {
                    let user_urlencoded: String = byte_serialize(u.as_bytes()).collect();
                    user_urlencoded
                }
                None => {
                    // the client::info module already validates that `get_username` returns a
                    // Some(u), but we create an error to return as a fallback, since match
                    // expressions must be exhaustive  
                    let err: Box<dyn Error> =
                        String::from("problem accessing environment variable NEOCITIES_USER")
                            .into();
                    return Err(err);
                }
            };

            let pass = match cred.get_password() {
                Some(p) => {
                    let pass_urlencoded: String = byte_serialize(p.as_bytes()).collect();
                    pass_urlencoded
                }
                None => {
                    // the client::info module already validates that `get_password` returns a
                    // Some(p), but we create an error to return as a fallback, since match
                    // expressions must be exhaustive  
                    let err: Box<dyn Error> =
                        String::from("problem accessing environment variable NEOCITIES_PASS")
                            .into();
                    return Err(err);
                }
            };

            // user:pass url
            url = format!("https://{}:{}@{}info", user, pass, API_URL);
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
            let e: Box<dyn std::error::Error> = format!(
                "The Neocities API could not find site '{}'.",
                args[0]
            )
            .into();
            Err(e)
        }
    }
}
