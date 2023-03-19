use reqwest::StatusCode;

#[tokio::main]
/// Prepares and sends a GET request to the Neocities API. It awaits a respons and returns either a
/// response body or an error.
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

    match res.status() {
        StatusCode::OK => {
            let raw = res.text().await?;

            let body: serde_json::Value = serde_json::from_str(&raw)?;

            Ok(body)
        }
        StatusCode::NOT_FOUND => {
            let e: Box<dyn std::error::Error> =
                String::from(status_message(StatusCode::NOT_FOUND)).into();
            Err(e)
        }
        StatusCode::BAD_GATEWAY => {
            let e: Box<dyn std::error::Error> =
                String::from(status_message(StatusCode::BAD_GATEWAY)).into();
            Err(e)
        }
        _ => {
            // TODO handle other status codes
            let e: Box<dyn std::error::Error> =
                format!("The Neocities API could not find site '{}'.", url).into();
            Err(e)
        }
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
        _ => String::from(""),
    }
}

#[cfg(test)]
mod tests {
    use super::get_request;

    #[test]
    fn basic_request() {
        let res = get_request("https://httpbin.org/ip".to_string(), None);
        assert_eq!(res.is_ok(), true);

        if let Ok(data) = res {
            assert_eq!(data["origin"], "47.13.94.134");
        }
    }
}
