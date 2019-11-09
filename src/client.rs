use hyper::header;
use reqwest::Response;
use opennode::withdrawal::Withdrawal;
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use serde_json;

pub struct Client {
    pub clt: reqwest::Client,
    pub host: String,
    pub api_key: String,
}

impl Client {
    /// Creates a new client posted to a custom host.
    pub fn new(host: &str, apikey: &str) -> Client {
        Client {
            clt: reqwest::Client::new(),
            api_key: apikey.to_string(),
            host: host.to_string(),
        }
    }
}

pub enum Error {
    Http(reqwest::Error),
    Opennode(opennode::error::RequestError)
}

#[derive(Debug, Deserialize)]
pub struct Data<T> {
    pub data: T,
}

pub async fn post<P, T>(client: &Client, path: &str, payload: Option<P>) -> Result<T, Error>
where
    P: Serialize,
    T: DeserializeOwned,
{

    let mut body: Vec<u8> = Vec::new();
    let mut content_type = "".to_string();
    if let Some(p) = payload {
        body = serde_json::to_vec(&p).unwrap();
        content_type = "application/json".to_string();
    }

    let res = client.clt.post(path)
        .header(header::CONTENT_TYPE, content_type)
        .body(body)
        .send()
        .await
        .map_err(|e| Error::Http(e))?;

    if res.status().is_success() {
        let d: Data<T> = res.json().await.map_err(|e| Error::Http(e))?;
        return Ok(d.data);
    }

    let e: opennode::error::RequestError = res.json().await.map_err(|e|{Error::Http(e)})?;
    Err(Error::Opennode(e))
}

// async fn create_withdrawal(client: &Client, invoice: &str) -> Result<Withdrawal, GenericError> {
//     let req = request::builder().method(Method)

// }
