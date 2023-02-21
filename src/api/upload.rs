use reqwest::{header::AUTHORIZATION, multipart, Body, Client, Response};
use std::error::Error;
use tokio::fs::File;
use tokio_util::codec::{BytesCodec, FramedRead};

use super::credentials::{Auth, Credentials};

#[tokio::main]
pub async fn api_call(cred: Credentials, args: Vec<String>) -> Result<(), Box<dyn Error>> {
    println!("starting api call");
    if args.len() < 1 {
        let err: Box<dyn Error> = String::from("no arguments given").into();
        return Err(err);
    }

    // let filename = args[0].as_str();
    let url: String;
    let api_key: Option<String>;

    println!("starting authentication");
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
    let file = File::open(&args[0]).await?;

    let stream = FramedRead::new(file, BytesCodec::new());
    let file_body = Body::wrap_stream(stream);

    let some_file = multipart::Part::stream(file_body)
        .file_name("test.txt")
        .mime_str("text/plain")?;

    let form = multipart::Form::new().part("file", some_file);

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
            let body = res.text().await?;
            println!("res body = {}", body);
            Ok(())
        }
        _ => {
            let err: Box<dyn Error> = String::from("bad request").into();
            Err(err)
        }
    }
}
