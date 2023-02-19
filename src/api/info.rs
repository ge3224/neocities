// use super::API_URL;
use hyper::{body::HttpBody as _, Client, Uri};
use hyper_tls::HttpsConnector;
use tokio::io::{stdout, AsyncWriteExt as _};

#[tokio::main]
pub async fn request_info(
    sitename: &String,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let https = HttpsConnector::new();
    let client = Client::builder().build::<_, hyper::Body>(https);

    let p_q = format!("/api/info?sitename={}", sitename);
    let uri = Uri::builder()
        .scheme("https")
        .authority("neocities.org")
        .path_and_query(p_q.as_str())
        .build()
        .unwrap();
    
    let mut resp = client.get(uri).await?;
    
    println!("Response: {}", resp.status());
    
    while let Some(chunk) = resp.body_mut().data().await {
        stdout().write_all(&chunk?).await?;
    }

    Ok(())
}
