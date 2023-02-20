use std::error::Error;

use serde_derive::Deserialize;
use serde_derive::Serialize;
use serde_json::Value;
use url::form_urlencoded::byte_serialize;

use crate::api::API_URL;
use crate::client::help;

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

    if args.len() > 0 {
        url = format!("https://{}/info?sitename={}", API_URL, args[0]);
    } else {
        if let Some(_k) = cred.get_api_key() {
            todo!()
        } else {
            let user = match cred.get_username() {
                Some(u) => {
                    let user_urlencoded: String = byte_serialize(u.as_bytes()).collect();
                    user_urlencoded
                }
                None => {
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
                    let err: Box<dyn Error> =
                        String::from("problem accessing environment variable NEOCITIES_PASS")
                            .into();
                    return Err(err);
                }
            };

            url = format!("https://{}:{}@{}info", user, pass, API_URL);
        }
    }

    let res = reqwest::get(url.as_str()).await?;

    match res.status() {
        reqwest::StatusCode::OK => {
            let body = res.json::<SiteInfo>().await?;
            Ok(body)
        }
        _ => {
            let e: Box<dyn std::error::Error> = format!(
                "The Neocities API could not find site '{}'. Please try a different sitename.",
                args[0]
            )
            .into();
            Err(e)
        }
    }
}
