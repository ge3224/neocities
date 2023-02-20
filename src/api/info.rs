use serde_derive::Deserialize;
use serde_derive::Serialize;
use serde_json::Value;

use crate::api::API_URL;

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
pub async fn api_call(sitename: &String) -> Result<SiteInfo, Box<dyn std::error::Error>> {
    let url = format!("https://{}/info?sitename={}", API_URL, sitename);

    let res = reqwest::get(url.as_str()).await?;

    match res.status() {
        reqwest::StatusCode::OK => {
            let body = res.json::<SiteInfo>().await?;
            Ok(body)
        }
        _ => {
            let e: Box<dyn std::error::Error> = format!(
                "The Neocities API could not find site '{}'. Please try a different sitename.",
                sitename
            )
            .into();
            Err(e)
        }
    }
}
