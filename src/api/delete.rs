use std::error::Error;

use super::credentials::Credentials;

#[tokio::main]
pub async fn api_call(_cred: Credentials, _args: Vec<String>) -> Result<(), Box<dyn Error>> {
    println!("the api module 'delete' was called!");
    todo!();
}
