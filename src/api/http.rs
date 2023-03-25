use std::path::PathBuf;

use reqwest::{header::AUTHORIZATION, multipart, Body, Client, Response, StatusCode};
use tokio::fs::File;
use tokio_util::codec::{BytesCodec, FramedRead};

/// Contains specific data for forming http requests to interact with the Neocities API.
pub struct HttpRequestInfo {
    /// The path in an http request-line
    pub uri: String,
    /// An optional Neocities API Key, which will be added to an http request's header
    pub api_key: Option<String>,
    /// An optional http request body, used on POST requests
    pub body: Option<String>,
    /// Indicates whether a request should include multipart/form-data
    pub multipart: Option<Vec<String>>,
}

/// Prepares and sends a GET request to the Neocities API. It awaits a response and returns either a
/// response body or an error.
#[tokio::main]
pub async fn get_request(
    url: String,
    api_key: Option<String>,
) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
    let req = reqwest::Client::new();
    let res: reqwest::Response;
    if let Some(k) = api_key {
        res = req
            .get(url.as_str())
            .header(reqwest::header::AUTHORIZATION, format!("Bearer {}", k))
            .send()
            .await?;
    } else {
        res = req.get(url.as_str()).send().await?;
    }

    let status = res.status();
    if let reqwest::StatusCode::OK = status {
        let raw = res.text().await?;
        let body: serde_json::Value = serde_json::from_str(&raw)?;
        Ok(body)
    } else {
        Err(status_message(status).into())
    }
}

/// Prepares and sends a POST request to the Neocities API containing multipart/form-data.
#[tokio::main]
pub async fn post_request_multipart(
    uri: String,
    api_key: Option<String>,
    multipart: Option<Vec<String>>,
) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
    let client = Client::new();
    let mut form = multipart::Form::new();

    if let Some(a) = multipart {
        for arg in a.iter() {
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
    } else {
        return Err(format!("no filepaths were given").into());
    }

    let res: Response;
    if let Some(k) = api_key {
        res = client
            .post(&uri)
            .header(AUTHORIZATION, format!("Bearer {}", k))
            .multipart(form)
            .send()
            .await?;
    } else {
        res = client.post(&uri).multipart(form).send().await?;
    }

    let status = res.status();
    if let reqwest::StatusCode::OK = status {
        let raw = res.text().await?;
        let body: serde_json::Value = serde_json::from_str(&raw)?;
        Ok(body)
    } else {
        Err(status_message(status).into())
    }
}

/// Prepares and sends a POST request with a body to the Neocities API.
#[tokio::main]
pub async fn post_request_body(
    uri: String,
    api_key: Option<String>,
    body: Option<String>,
) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
    let files = match body {
        Some(f) => f,
        None => {
            let e: Box<dyn std::error::Error> =
                String::from("not files were given for this request").into();
            return Err(e);
        }
    };

    let req = reqwest::Client::new();
    let res: reqwest::Response;
    if let Some(k) = api_key {
        res = req
            .post(&uri)
            .header(reqwest::header::AUTHORIZATION, format!("Bearer {}", k))
            .body(files)
            .send()
            .await?;
    } else {
        res = req.post(&uri).body(files).send().await?;
    }

    let status = res.status();
    if let reqwest::StatusCode::OK = status {
        let raw = res.text().await?;
        let body: serde_json::Value = serde_json::from_str(&raw)?;
        Ok(body)
    } else {
        Err(status_message(status).into())
    }
}

fn status_message(code: StatusCode) -> String {
    match code {
        StatusCode::BAD_REQUEST => {
            let msg = "400 Bad Request - The server cannot or will not process the request due to something that is perceived to be a client error.";
            return String::from(msg);
        }
        StatusCode::NOT_FOUND => {
            let msg = "404 Not Found - The server cannot find the requested resource. The URL is not recognized.";
            return String::from(msg);
        }
        StatusCode::INTERNAL_SERVER_ERROR => {
            let msg = "500 Internal Server Error - The server has encountered a situation it does not know how to handle.";
            return String::from(msg);
        }
        StatusCode::BAD_GATEWAY => {
            let msg = "502 Bad Gateway - This error response means that the server, while working as a gateway to get a response needed to handle the request, got an invalid response.";
            return String::from(msg);
        }
        _ => {
            if let Some(reason) = code.canonical_reason() {
                String::from(reason)
            } else {
                String::from(code.to_string())
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{get_request, post_request_body, post_request_multipart};

    #[test]
    fn basic_get_request() {
        let res = get_request("https://httpbin.org/ip".to_string(), None);
        assert_eq!(res.is_ok(), true);
        assert_ne!(res.unwrap()["origin"], "");
    }

    #[test]
    fn basic_post_request_body() {
        let res = post_request_body(
            "https://httpbin.org/post".to_string(),
            None,
            Some("filenames[]=img2.jpg".to_string()),
        );
        assert_eq!(res.is_ok(), true);
        assert_eq!(res.unwrap()["data"], "filenames[]=img2.jpg");
    }

    #[test]
    fn basic_post_request_multipart() {
        let res = post_request_multipart(
            "https://httpbin.org/post".to_string(),
            None,
            Some(vec!["./tests/fixtures/foo.html".to_string()]),
        );
        assert_eq!(res.is_ok(), true);
    }
}
