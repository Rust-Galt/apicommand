use reqwest::header::InvalidHeaderValue;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    // Database
    #[error("Database error")]
    DatabaseSqlx(#[from] sqlx::Error),
    #[error("Database query error")]
    DatabaseQuery(#[from] sea_query::error::Error),

    // Network
    #[error("Request error")]
    NetworkInvalidTimestamp(#[from] reqwest::Error),
    #[error("Header error")]
    NetworkInvalidHeaderValue(#[from] InvalidHeaderValue),
    #[error("Expected 200 OK status from API. But received `{0}`")]
    NetworkUnexpectedStatusCode(String),

    // Validate
    #[error("Invalid brand_id: `{0}`")]
    ValidateInvalidBrandId(String),
    #[error("Invalid location_id: `{0}`")]
    ValidateInvalidLocationId(String),
    #[error("Parse error for timestamp: `{0}`")]
    ValidateInvalidTimestamp(#[from] std::num::ParseIntError),
    #[error(r#"Invalid date_time_stamp: "to_date"=`{1}` can't be smaller than "from_date"=`{0}`"#)]
    ValidateInvalidDateTimeSpan(u64, u64),
}
