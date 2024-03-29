use crate::error::NeocitiesErr;
use reqwest::{header::AUTHORIZATION, multipart, Body, Client, StatusCode};
use std::path::PathBuf;
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
    uri: String,
    api_key: Option<String>,
) -> Result<serde_json::Value, NeocitiesErr> {
    let req = reqwest::Client::new();

    let res: reqwest::Response = match api_key {
        Some(k) => {
            let attempt = req
                .get(uri.as_str())
                .header(AUTHORIZATION, format!("Bearer {}", k))
                .send()
                .await;
            match attempt {
                Ok(r) => r,
                Err(e) => return Err(NeocitiesErr::HttpRequestError(e.into())),
            }
        }
        None => {
            let attempt = req.get(uri.as_str()).send().await;
            match attempt {
                Ok(r) => r,
                Err(e) => return Err(NeocitiesErr::HttpRequestError(e.into())),
            }
        }
    };

    if let reqwest::StatusCode::OK = res.status() {
        let attempt = res.text().await;
        match attempt {
            Ok(t) => {
                match serde_json::from_str(&t) {
                    Ok(b) => return Ok(b),
                    Err(e) => return Err(NeocitiesErr::SerdeDeserializationError(e)),
                };
            }
            Err(e) => return Err(NeocitiesErr::HttpRequestError(e.into())),
        }
    }

    return Err(NeocitiesErr::HttpRequestError(
        status_message(res.status()).into(),
    ));
}

/// Prepares and sends a POST request to the Neocities API containing multipart/form-data.
#[tokio::main]
pub async fn post_request_multipart(
    uri: String,
    api_key: Option<String>,
    multipart: Option<Vec<String>>,
) -> Result<serde_json::Value, NeocitiesErr> {
    let client = Client::new();
    let mut form = multipart::Form::new();

    if let Some(a) = multipart {
        for arg in a.iter() {
            let path = PathBuf::from(&arg);

            let filepath: String;
            if let Some(p) = path.to_str() {
                filepath = p.to_string();
            } else {
                return Err(NeocitiesErr::HttpRequestError(
                    format!("problem with file/path: {arg}").into(),
                ));
            }

            let file = File::open(path).await?;
            let stream = FramedRead::new(file, BytesCodec::new());
            let file_body = Body::wrap_stream(stream);

            let some_file = multipart::Part::stream(file_body).file_name(filepath.clone());
            form = form.part(filepath, some_file);
        }
    } else {
        return Err(NeocitiesErr::HttpRequestError(
            format!("no filepaths were given").into(),
        ));
    }

    let res: reqwest::Response = match api_key {
        Some(k) => {
            let attempt = client
                .post(&uri)
                .header(AUTHORIZATION, format!("Bearer {}", k))
                .multipart(form)
                .send()
                .await;
            match attempt {
                Ok(r) => r,
                Err(e) => return Err(NeocitiesErr::HttpRequestError(e.into())),
            }
        }
        None => {
            let attempt = client.post(&uri).multipart(form).send().await;
            match attempt {
                Ok(r) => r,
                Err(e) => return Err(NeocitiesErr::HttpRequestError(e.into())),
            }
        }
    };

    if let reqwest::StatusCode::OK = res.status() {
        let attempt = res.text().await;
        match attempt {
            Ok(t) => {
                match serde_json::from_str(&t) {
                    Ok(b) => return Ok(b),
                    Err(e) => return Err(NeocitiesErr::SerdeDeserializationError(e)),
                };
            }
            Err(e) => return Err(NeocitiesErr::HttpRequestError(e.into())),
        }
    }

    return Err(NeocitiesErr::HttpRequestError(
        status_message(res.status()).into(),
    ));
}

/// Prepares and sends a POST request with a body to the Neocities API.
#[tokio::main]
pub async fn post_request_body(
    uri: String,
    api_key: Option<String>,
    body: Option<String>,
) -> Result<serde_json::Value, NeocitiesErr> {
    let files = match body {
        Some(f) => f,
        None => {
            let e: Box<dyn std::error::Error> =
                String::from("not files were given for this request").into();
            return Err(NeocitiesErr::HttpRequestError(e.into()));
        }
    };

    let req = reqwest::Client::new();

    let res: reqwest::Response = match api_key {
        Some(k) => {
            let attempt = req
                .post(&uri)
                .header(AUTHORIZATION, format!("Bearer {}", k))
                .body(files)
                .send()
                .await;
            match attempt {
                Ok(r) => r,
                Err(e) => return Err(NeocitiesErr::HttpRequestError(e.into())),
            }
        }
        None => {
            let attempt = req.post(&uri).body(files).send().await;
            match attempt {
                Ok(r) => r,
                Err(e) => return Err(NeocitiesErr::HttpRequestError(e.into())),
            }
        }
    };

    if let reqwest::StatusCode::OK = res.status() {
        let attempt = res.text().await;
        match attempt {
            Ok(t) => {
                match serde_json::from_str(&t) {
                    Ok(b) => return Ok(b),
                    Err(e) => return Err(NeocitiesErr::SerdeDeserializationError(e)),
                };
            }
            Err(e) => return Err(NeocitiesErr::HttpRequestError(e.into())),
        }
    }

    return Err(NeocitiesErr::HttpRequestError(
        status_message(res.status()).into(),
    ));
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
