use super::http::get_request;
use super::API_URL;
use crate::error::NeocitiesErr;
use serde_derive::Deserialize;
use serde_derive::Serialize;

/// Handles the requesting of an API key from Neocities at `/api/key`
pub struct NcKey {}

/// Contains data received from Neocities in response to a request for an API Key
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ApiKeyResponse {
    /// A status message
    pub result: String,
    /// The API key generated by the Neocities server for a specific registered user
    #[serde(rename = "api_key")]
    pub api_key: String,
}

impl NcKey {
    fn prepare_url(user: String, password: String) -> String {
        return format!("https://{}:{}@{}key", user, password, API_URL);
    }

    fn to_api_key_response(value: serde_json::Value) -> Result<ApiKeyResponse, NeocitiesErr> {
        match serde_json::from_value(value) {
            Ok(res) => Ok(res),
            Err(e) => return Err(NeocitiesErr::SerdeDeserializationError(e)),
        }
    }

    /// Prepares and sends a request for an API key to Neocities at the `api/key` endpoint. It awaits a
    /// response and returns either an ApiKey or an error.
    pub fn fetch(user: String, pass: String) -> Result<ApiKeyResponse, NeocitiesErr> {
        let url = NcKey::prepare_url(user, pass);

        match get_request(url, None) {
            Ok(res) => match NcKey::to_api_key_response(res) {
                Ok(akr) => Ok(akr),
                Err(e) => Err(NeocitiesErr::HttpRequestError(Box::new(e))),
            },
            Err(e) => Err(NeocitiesErr::HttpRequestError(e)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::NcKey;

    #[test]
    fn key_url() {
        let url = NcKey::prepare_url("foo".to_string(), "bar".to_string());
        assert_eq!(url, "https://foo:bar@neocities.org/api/key");
    }

    #[test]
    fn value_to_api_key_response() {
        let str = r#"
        {
           "result": "Success",
           "api_key": "xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx"
        }"#;

        let v: serde_json::Value = serde_json::from_str(str).unwrap();
        let akr = NcKey::to_api_key_response(v).unwrap();
        assert_eq!(akr.result, "Success");
        assert_eq!(akr.api_key, "xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx");
    }
}
