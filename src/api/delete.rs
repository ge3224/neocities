use super::credentials::Credentials;
use super::http::post_request_body;
use super::http::HttpRequestInfo;
use crate::api::credentials::Auth;
use serde_derive::Deserialize;
use serde_derive::Serialize;
use std::error::Error;

/// Handles the request to delete file(s) from a Neocities website using the
/// following endpoint: `/api/delete`
pub struct NcDelete {}

/// Contains data received from Neocities in response to a request to `/api/delete`
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeleteResponse {
    /// A status message
    pub result: String,
    #[serde(rename = "error_type")]
    /// An error message
    pub error_type: Option<String>,
    /// An explanation of the the delete operation that has occurred
    pub message: String,
}

impl NcDelete {
    fn request_info(args: Vec<String>) -> Result<HttpRequestInfo, Box<dyn std::error::Error>> {
        let url: String;
        let api_key: Option<String>;
        let cred = Credentials::new();
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
        let pk = HttpRequestInfo {
            uri: url,
            api_key,
            body: Some(files),
            multipart: None,
        };
        Ok(pk)
    }

    fn to_delete_response(
        value: serde_json::Value,
    ) -> Result<DeleteResponse, Box<dyn std::error::Error>> {
        let attempt = serde_json::from_value(value);
        match attempt {
            Ok(res) => Ok(res),
            _ => {
                let e: Box<dyn std::error::Error> = String::from("a problem occurred while converting the deserialized json to the DeleteResponse type").into();
                return Err(e);
            }
        }
    }

    /// Prepares and sends a request for specified files to be deleted from a Neocities user's website.
    /// It awaits a response and returns either a DeleteResponse or an error.
    pub fn fetch(args: Vec<String>) -> Result<DeleteResponse, Box<dyn Error>> {
        // get http path and api_key for headers
        let req_info = match NcDelete::request_info(args) {
            Ok(v) => v,
            Err(e) => return Err(e),
        };

        match post_request_body(req_info.uri, req_info.api_key, req_info.body) {
            Ok(res) => match NcDelete::to_delete_response(res) {
                Ok(ir) => Ok(ir),
                Err(e) => Err(e),
            },
            Err(e) => Err(e),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::DeleteResponse;
    use crate::api::{credentials::ENV_KEY, delete::NcDelete};
    use std::env;

    #[test]
    fn delete_request_path() {
        let preserve_key = env::var(ENV_KEY);
        env::set_var(ENV_KEY, "foo");

        let mock_args = vec![String::from("foo")];
        let pk = NcDelete::request_info(mock_args).unwrap();

        assert_eq!(pk.api_key.unwrap(), "foo");
        assert_eq!(pk.uri, "https://neocities.org/api/delete");
        assert_eq!(pk.body.unwrap(), "filenames[]=foo");

        // reset environment var
        match preserve_key {
            Ok(v) => env::set_var(ENV_KEY, v),
            _ => env::remove_var(ENV_KEY),
        }
    }

    #[test]
    fn convert_value_to_delete_response() {
        let mock_str_1 = r#"
        {
          "result": "success",
          "message": "file(s) have been deleted"
        }"#;

        let v: serde_json::Value = serde_json::from_str(mock_str_1).unwrap();
        let dr: DeleteResponse = NcDelete::to_delete_response(v).unwrap();

        assert_eq!(dr.result, "success");
        assert_eq!(dr.message, "file(s) have been deleted");

        let mock_str_2 = r#"
         {
           "result": "error",
           "error_type": "missing_files",
           "message": "foo.html was not found on your site, canceled deleting"
         }"#;

        let v: serde_json::Value = serde_json::from_str(mock_str_2).unwrap();
        let dr: DeleteResponse = NcDelete::to_delete_response(v).unwrap();

        assert_eq!(dr.result, "error");
        assert_eq!(dr.error_type.unwrap(), "missing_files");
        assert_eq!(
            dr.message,
            "foo.html was not found on your site, canceled deleting"
        );
    }
}
