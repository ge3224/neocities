use std::error::Error;

use reqwest::header::AUTHORIZATION;
use reqwest::Client;
use reqwest::Response;
use serde_derive::Deserialize;
use serde_derive::Serialize;

use super::credentials::Credentials;
use crate::api::credentials::Auth;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeleteResponse {
    pub result: String,
    #[serde(rename = "error_type")]
    pub error_type: Option<String>,
    pub message: String,
}

#[tokio::main]
pub async fn api_call(
    cred: Credentials,
    args: Vec<String>,
) -> Result<DeleteResponse, Box<dyn Error>> {
    let url: String;
    let api_key: Option<String>;

    let auth = Auth::authenticate(cred, String::from("delete"), None);

    match auth {
        Ok(a) => {
            url = a.url;
            api_key = a.api_key;
        }
        Err(e) => return Err(format!("problem authenticating credentials: {e}").into()),
    }

    let mut files = String::from("");
    for arg in args.iter() {
        if files.len() > 0 {
            files.push_str("&");
        }
        files.push_str("filenames[]=");
        files.push_str(arg);
    }
    println!("files = {}", files);

    let req = Client::new();
    let res: Response;

    if let Some(k) = api_key {
        res = req
            .post(&url)
            .header(AUTHORIZATION, format!("Bearer {}", k))
            .body(files)
            .send()
            .await?;
    } else {
        res = req.post(&url).body(files).send().await?;
    }

    match res.status() {
        reqwest::StatusCode::OK => {
            let body = res.json::<DeleteResponse>().await?;
            Ok(body)
        }
        _ => return Err(String::from("error deleting file").into()),
    }
}
