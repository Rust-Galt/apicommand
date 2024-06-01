use crate::{
    configuration::Config,
    error::Error,
    validate::{GetParameters, LastRunParameters, RunParameters, SpecificParameters},
};

use chrono::{DateTime, Utc};
use reqwest::{
    header::{HeaderMap, HeaderValue},
    StatusCode, Url,
};
use strum::Display;

#[derive(Debug)]
pub struct ApiResponse {
    pub date_time: DateTime<Utc>,
    pub request_type: RequestType,
    pub status: StatusCode,
    pub url: Url,
    pub data: String,
}

#[derive(Debug, Display)]
pub enum RequestType {
    Get(GetParameters),
    LastRun(LastRunParameters),
    Run(RunParameters),
    Specific(SpecificParameters),
}
pub async fn send_api_request(
    config: &Config,
    request_type: RequestType,
) -> Result<ApiResponse, Error> {
    // Enables to handle different request types in various ways
    let tail = match &request_type {
        RequestType::Get(p) => p.to_string(),
        RequestType::LastRun(p) => p.to_string(),
        RequestType::Run(p) => p.to_string(),
        RequestType::Specific(p) => p.to_string(),
    };
    let url = format!("{}/{}", config.get_api_root(), tail);
    let client = reqwest::Client::new();

    let mut headers = HeaderMap::new();

    if let Some(key) = config.get_api_key() {
        headers.append("X-API-Key", HeaderValue::from_str(key)?);
    }

    let response = client.get(url).headers(headers).send().await?;
    match response.status() {
        StatusCode::OK => Ok(ApiResponse {
            date_time: Utc::now(),
            request_type,
            status: response.status(),
            url: response.url().clone(),
            data: response.text().await?,
        }),
        _ => Err(Error::NetworkUnexpectedStatusCode(
            response.status().to_string(),
        )),
    }
}
