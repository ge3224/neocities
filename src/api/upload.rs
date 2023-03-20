use super::credentials::{Auth, Credentials};
use super::http::{post_request_multipart, HttpRequestInfo};
use serde_derive::Deserialize;
use serde_derive::Serialize;
use std::error::Error;

/// Handles the request to upload file(s) to a Neocities website using the
/// following endpoint: `/api/upload`
pub struct NcUpload {}

/// Contains data from Neocities in response to a request at `/api/upload`
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UploadResponse {
    /// A status message
    pub result: String,
    /// An error message
    pub error_type: Option<String>,
    /// An explanation of the upload operation that has occurred
    pub message: String,
}

impl NcUpload {
    fn request_info(args: Vec<String>) -> Result<HttpRequestInfo, Box<dyn std::error::Error>> {
        let cred = Credentials::new();
        let uri: String;
        let api_key: Option<String>;

        let auth = Auth::authenticate(cred, String::from("upload"), None);

        match auth {
            Ok(a) => {
                uri = a.url;
                api_key = a.api_key;
            }
            Err(e) => {
                let err: Box<dyn Error> = format!("problem authenticating credentials: {e}").into();
                return Err(err);
            }
        }

        let hri = HttpRequestInfo {
            uri,
            api_key,
            body: None,
            multipart: Some(args),
        };

        Ok(hri)
    }

    fn to_upload_response(
        value: serde_json::Value,
    ) -> Result<UploadResponse, Box<dyn std::error::Error>> {
        let attempt = serde_json::from_value(value);
        match attempt {
            Ok(res) => Ok(res),
            _ => {
                let e: Box<dyn std::error::Error> = String::from("a problem occurred while converting the deserialized json to the DeleteResponse type").into();
                return Err(e);
            }
        }
    }

    /// Prepares and sends a request containing a multipart form file upload. It awaits a response and
    /// returns either a UploadResponse or an error.
    pub fn fetch(args: Vec<String>) -> Result<UploadResponse, Box<dyn Error>> {
        // get http path and api_key for headers
        let req_info = match NcUpload::request_info(args) {
            Ok(v) => v,
            Err(e) => return Err(e),
        };

        match post_request_multipart(req_info.uri, req_info.api_key, req_info.multipart) {
            Ok(res) => match NcUpload::to_upload_response(res) {
                Ok(ir) => Ok(ir),
                Err(e) => Err(e),
            },
            Err(e) => Err(e),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::api::{credentials::ENV_KEY, upload::NcUpload};
    use std::env;

    #[test]
    fn request_info_format() {
        let preserve_key = env::var(ENV_KEY);
        env::set_var(ENV_KEY, "foo");

        let mock_args = vec![String::from("foo")];
        let hri = NcUpload::request_info(mock_args).unwrap();

        assert_eq!(hri.api_key.unwrap(), "foo");
        assert_eq!(hri.uri, "https://neocities.org/api/upload");
        assert_eq!(hri.multipart.is_some(), true);
        assert_eq!(hri.multipart.unwrap()[0], "foo");

        // reset environment var
        match preserve_key {
            Ok(v) => env::set_var(ENV_KEY, v),
            _ => env::remove_var(ENV_KEY),
        }
    }
}
