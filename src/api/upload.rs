use super::credentials::{Auth, Credentials};
use reqwest::{header::AUTHORIZATION, multipart, Body, Client, Response};
use serde_derive::Deserialize;
use serde_derive::Serialize;
use std::{error::Error, path::PathBuf};
use tokio::fs::File;
use tokio_util::codec::{BytesCodec, FramedRead};

/// Contains data from Neocities in response to a request at `/api/upload`
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UploadRequest {
    /// A status message
    pub result: String,
    /// An explanation of the upload operation that has occurred
    pub message: String,
}

impl UploadRequest {
    /// Prepares and sends a request containing a multipart form file upload. It awaits a response and
    /// returns either a UploadResponse or an error.
    #[tokio::main]
    pub async fn fetch(
        cred: Credentials,
        args: Vec<String>,
    ) -> Result<UploadRequest, Box<dyn Error>> {
        let url: String;
        let api_key: Option<String>;

        let auth = Auth::authenticate(cred, String::from("upload"), None);

        match auth {
            Ok(a) => {
                url = a.url;
                api_key = a.api_key;
            }
            Err(e) => {
                let err: Box<dyn Error> = format!("problem authenticating credentials: {e}").into();
                return Err(err);
            }
        }

        let client = Client::new();
        let mut form = multipart::Form::new();

        for arg in args.iter() {
            let path = PathBuf::from(&arg);

            let filepath: String;
            if let Some(p) = path.to_str() {
                filepath = p.to_string();
            } else {
                return Err(format!("problem with file/path: {arg}").into());
            }

            let file = File::open(path).await?;
            let stream = FramedRead::new(file, BytesCodec::new());
            let file_body = Body::wrap_stream(stream);

            let some_file = multipart::Part::stream(file_body).file_name(filepath.clone());
            form = form.part(filepath, some_file);
        }

        let res: Response;
        if let Some(k) = api_key {
            res = client
                .post(&url)
                .header(AUTHORIZATION, format!("Bearer {}", k))
                .multipart(form)
                .send()
                .await?;
        } else {
            res = client.post(&url).multipart(form).send().await?;
        }

        match res.status() {
            reqwest::StatusCode::OK => {
                let body = res.json::<UploadRequest>().await?;
                Ok(body)
            }
            _ => todo!(),
        }
    }
}
