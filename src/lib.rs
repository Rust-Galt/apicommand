pub mod configuration;
pub mod error;
pub mod validate;

mod database;
mod network;

use configuration::Config;
use error::Error;
use network::{ApiResponse, RequestType};
use validate::{GetParameters, LastRunParameters, RunParameters, SpecificParameters};

/// get function returns X from API providing brand_id
pub async fn get(config: &Config, raw_brand_id: String) -> Result<ApiResponse, Error> {
    // Validate parameters
    let parameters = GetParameters::new(raw_brand_id)?;
    // Send API requests
    let api_response = network::send_api_request(config, RequestType::Get(parameters)).await?;
    // Save result into database
    database::write(config, &api_response).await?;
    Ok(api_response)
}

/// last_run function returns X from API providing brand_id and location_id
pub async fn last_run(
    config: &Config,
    raw_brand_id: String,
    raw_location_id: String,
) -> Result<ApiResponse, Error> {
    // Validate parameters
    let parameters = LastRunParameters::new(raw_brand_id, raw_location_id)?;
    // Send API requests
    let api_response = network::send_api_request(config, RequestType::LastRun(parameters)).await?;
    // Save result into database
    database::write(config, &api_response).await?;
    Ok(api_response)
}

/// run function returns X from API providing brand_id and location_id
pub async fn run(
    config: &Config,
    raw_brand_id: String,
    raw_location_id: String,
) -> Result<ApiResponse, Error> {
    // Validate parameters
    let parameters = RunParameters::new(raw_brand_id, raw_location_id)?;
    // Send API requests
    let api_response = network::send_api_request(config, RequestType::Run(parameters)).await?;
    // Save result into database
    database::write(config, &api_response).await?;
    Ok(api_response)
}

/// specific function returns X from API providing brand_id, location_id and to/from dates
pub async fn specific(
    config: &Config,
    raw_brand_id: String,
    raw_location_id: String,
    raw_from_date: String,
    raw_to_date: String,
) -> Result<ApiResponse, Error> {
    // Validate parameters
    let parameters =
        SpecificParameters::new(raw_brand_id, raw_location_id, raw_from_date, raw_to_date)?;
    // Send API requests
    let api_response = network::send_api_request(config, RequestType::Specific(parameters)).await?;
    // Save result into database
    database::write(config, &api_response).await?;
    Ok(api_response)
}

#[tokio::test]
async fn get_test() {
    let config = Config::builder()
        .api_key(Some("API-TEST-KEY".to_string()))
        .api_root("https://httpbin.org/anything".to_string())
        .db_path(std::path::PathBuf::from("test.sqlite3"))
        .build();
    let raw_brand_id = "test_brand_id";
    let response = get(&config, raw_brand_id.to_string()).await.unwrap();

    assert!(response.url.as_str() == format!("{}/get/{}", &config.get_api_root(), raw_brand_id));
    println!("{}", response.url.as_str());
}

#[tokio::test]
async fn last_run_test() {
    let config = Config::builder()
        .api_key(Some("API-TEST-KEY".to_string()))
        .api_root("https://httpbin.org/anything".to_string())
        .db_path(std::path::PathBuf::from("test.sqlite3"))
        .build();
    let raw_brand_id = "test_brand_id";
    let raw_location_id = "test_location_id";
    let response = last_run(
        &config,
        raw_brand_id.to_string(),
        raw_location_id.to_string(),
    )
    .await
    .unwrap();

    assert!(
        response.url.as_str()
            == format!(
                "{}/last_run/{}/{}",
                &config.get_api_root(),
                raw_brand_id,
                raw_location_id
            )
    );
    println!("{}", response.url.as_str());
}

#[tokio::test]
async fn run_test() {
    let config = Config::builder()
        .api_key(Some("API-TEST-KEY".to_string()))
        .api_root("https://httpbin.org/anything".to_string())
        .db_path(std::path::PathBuf::from("test.sqlite3"))
        .build();
    let raw_brand_id = "test_brand_id";
    let raw_location_id = "test_location_id";
    let response = run(
        &config,
        raw_brand_id.to_string(),
        raw_location_id.to_string(),
    )
    .await
    .unwrap();

    assert!(
        response.url.as_str()
            == format!(
                "{}/run/{}/{}",
                &config.get_api_root(),
                raw_brand_id,
                raw_location_id
            )
    );
    println!("{}", response.url.as_str());
}

#[tokio::test]
async fn specific_test() {
    let config = Config::builder()
        .api_key(Some("API-TEST-KEY".to_string()))
        .api_root("https://httpbin.org/anything".to_string())
        .db_path(std::path::PathBuf::from("test.sqlite3"))
        .build();
    let raw_brand_id = "test_brand_id";
    let raw_location_id = "test_location_id";
    let raw_from_date = "100010001000";
    let raw_to_date = "100010001001";
    let response = specific(
        &config,
        raw_brand_id.to_string(),
        raw_location_id.to_string(),
        raw_from_date.to_string(),
        raw_to_date.to_string(),
    )
    .await
    .unwrap();

    assert!(
        response.url.as_str()
            == format!(
                "{}/specific/{}/{}/{}/{}",
                &config.get_api_root(),
                raw_brand_id,
                raw_location_id,
                raw_from_date,
                raw_to_date,
            )
    );
    println!("{}", response.url.as_str());
}
