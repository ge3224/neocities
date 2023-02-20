use super::API_URL;
use serde_derive::Deserialize;
use serde_derive::Serialize;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ApiKey {
    pub result: String,
    #[serde(rename = "api_key")]
    pub api_key: String,
}

#[tokio::main]
pub async fn api_call(user: String, pass: String) -> Result<ApiKey, Box<dyn std::error::Error>> {
    let url = format!("https://{}:{}@{}key", user, pass, API_URL);

    let res = reqwest::get(url.as_str()).await?;

    match res.status() {
        reqwest::StatusCode::OK => {
            let body = res.json::<ApiKey>().await?;
            Ok(body)
        }
        _ => {
            let e: Box<dyn std::error::Error> = String::from("The Neocities API would not accept our request; check that you're environment variables are set correctly. (See neocities help).").into();
            Err(e)
        }
    }
}
