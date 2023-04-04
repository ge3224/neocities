use super::credentials::{Auth, Credentials};
use super::http::{get_request, HttpRequestInfo};
use crate::api::API_URL;
use crate::client::info;
use crate::error::NeocitiesErr;
use serde_derive::Deserialize;
use serde_derive::Serialize;
use serde_json::Value;

/// Handles the requesting of site information from the Neocities API at `/api/info`
pub struct NcInfo {}

/// Contains data received from Neocities in response to a request to `/api/info`
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InfoResponse {
    /// A status message
    pub result: String,
    /// Information about a Neocities website
    pub info: Info,
}

/// Information about a Neocities website
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Info {
    /// A Neocities user's sitename, aka username
    pub sitename: String,
    /// Total views
    pub views: i64,
    /// Total hits
    pub hits: i64,
    /// A timestamp for the site's creation
    #[serde(rename = "created_at")]
    pub created_at: String,
    /// A timestamp for the sites most recent update
    #[serde(rename = "last_updated")]
    pub last_updated: String,
    /// A domain for the website, if configured
    pub domain: Value,
    /// Tags a Neocities user sets about the site
    pub tags: Vec<String>,
    /// A hash associated with the InterPlanetary File System protocol
    #[serde(rename = "latest_ipfs_hash")]
    pub latest_ipfs_hash: Value,
}

impl NcInfo {
    fn request_info(args: &Vec<String>) -> Result<HttpRequestInfo, NeocitiesErr> {
        let cred = Credentials::new();

        let url: String;
        let mut api_key: Option<String> = None;

        // give precedence to args so a user can run `neocities info [sitename]` to lookup other
        // websites, although environment variables have been set
        if args.len() > 0 {
            url = format!("https://{}/info?sitename={}", API_URL, args[0]);
        } else {
            let auth = Auth::authenticate(cred, info::KEY, None);

            match auth {
                Err(e) => return Err(NeocitiesErr::HttpRequestError(e.into())),
                Ok(a) => {
                    url = a.url;
                    api_key = a.api_key;
                }
            }
        }

        let ri = HttpRequestInfo {
            uri: url,
            api_key,
            body: None,
            multipart: None,
        };

        Ok(ri)
    }

    fn to_info_response(value: serde_json::Value) -> Result<InfoResponse, NeocitiesErr> {
        match serde_json::from_value(value) {
            Ok(res) => Ok(res),
            Err(e) => Err(NeocitiesErr::SerdeDeserializationError(e)),
        }
    }

    /// Prepares and sends a request for information about a specified Neocities website. It awaits a
    /// response and returns either SiteInfo or an error.
    pub fn fetch(args: &Vec<String>) -> Result<InfoResponse, NeocitiesErr> {
        // get http path and api_key for headers
        let ri = NcInfo::request_info(args)?;
        let res = get_request(ri.uri, ri.api_key)?;
        let nci = NcInfo::to_info_response(res)?;
        Ok(nci)
    }
}

#[cfg(test)]
mod tests {
    use super::{InfoResponse, NcInfo};
    use serde_json::Value;
    #[test]
    fn site_info_request() {
        let mock_args = vec![String::from("foo")];
        let ph = NcInfo::request_info(&mock_args).unwrap();
        assert_eq!(ph.uri, "https://neocities.org/api//info?sitename=foo");
    }

    #[test]
    fn value_to_info_response() {
        let mock_str = r#"
        {
            "result": "success",
            "info": {
                "sitename": "foo",
                "views": 100,
                "hits": 1000,
                "created_at": "Tue, 12 May 2013 18:49:21 +0000",
                "last_updated":  "Tue, 12 May 2013 18:49:21 +0000", 
                "domain": null,
                "tags": [],
                "latest_ipfs_hash": null
            }
        }"#;

        let v: serde_json::Value = serde_json::from_str(mock_str).unwrap();
        let ir: InfoResponse = NcInfo::to_info_response(v).unwrap();

        assert_eq!(ir.result, "success");
        assert_eq!(ir.info.sitename, "foo");
        assert_eq!(ir.info.views, 100);
        assert_eq!(ir.info.domain, Value::Null);
    }
}
