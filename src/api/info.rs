use serde_derive::Deserialize;
use serde_derive::Serialize;
use serde_json::Value;

use crate::api::API_URL;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ResponseInfo {
    pub result: String,
    pub info: SiteInfo,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SiteInfo {
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
pub async fn request_info(sitename: &String) -> Result<ResponseInfo, Box<dyn std::error::Error>> {
    let uri = format!("{}/info?sitename={}", API_URL, sitename);
    let resp = reqwest::get(uri.as_str())
        .await?
        .json::<ResponseInfo>()
        .await?;
    Ok(resp)
}
