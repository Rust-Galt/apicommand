use std::fmt::Display;

use crate::error::Error;

// New type pattern to guarantee valid parameter
#[derive(Debug)]
pub struct BrandId(String);
impl BrandId {
    pub fn new(raw_brand_id: String) -> Result<Self, Error> {
        // Arbitrary validation requirement as an example
        if raw_brand_id.len() <= 64 {
            Ok(Self(raw_brand_id))
        } else {
            Err(Error::ValidateInvalidBrandId(raw_brand_id))
        }
    }
}

#[derive(Debug)]
pub struct LocationId(String);
impl LocationId {
    pub fn new(raw_location_id: String) -> Result<Self, Error> {
        // Arbitrary validation requirement as an example
        if raw_location_id.len() <= 64 {
            Ok(Self(raw_location_id))
        } else {
            Err(Error::ValidateInvalidLocationId(raw_location_id))
        }
    }
}

#[derive(Debug)]
pub struct DateTimeSpan(u64, u64);
impl DateTimeSpan {
    pub fn new(raw_from_date: String, raw_to_date: String) -> Result<Self, Error> {
        // Parse timestamp from String and "bubble" error on parse
        let from_date = str::parse::<u64>(&raw_from_date)?;
        let to_date = str::parse::<u64>(&raw_to_date)?;

        // Validate that TO timestamp is not smaller than FROM timestamp
        if to_date >= from_date {
            Ok(Self(from_date, to_date))
        } else {
            Err(Error::ValidateInvalidDateTimeSpan(from_date, to_date))
        }
    }
}
// Constructors for parameters
#[derive(Debug)]
pub struct GetParameters {
    brand_id: BrandId,
}
impl GetParameters {
    pub fn new(raw_brand_id: String) -> Result<Self, Error> {
        Ok(Self {
            brand_id: BrandId::new(raw_brand_id)?,
        })
    }
}

#[derive(Debug)]
pub struct LastRunParameters {
    brand_id: BrandId,
    location_id: LocationId,
}
impl LastRunParameters {
    pub fn new(raw_brand_id: String, raw_location_id: String) -> Result<Self, Error> {
        Ok(Self {
            brand_id: BrandId::new(raw_brand_id)?,
            location_id: LocationId::new(raw_location_id)?,
        })
    }
}

#[derive(Debug)]
pub struct RunParameters {
    brand_id: BrandId,
    location_id: LocationId,
}
impl RunParameters {
    pub fn new(raw_brand_id: String, raw_location_id: String) -> Result<Self, Error> {
        Ok(Self {
            brand_id: BrandId::new(raw_brand_id)?,
            location_id: LocationId::new(raw_location_id)?,
        })
    }
}

#[derive(Debug)]
pub struct SpecificParameters {
    brand_id: BrandId,
    location_id: LocationId,
    date_time_span: DateTimeSpan,
}
impl SpecificParameters {
    pub fn new(
        raw_brand_id: String,
        raw_location_id: String,
        raw_from_date: String,
        raw_to_date: String,
    ) -> Result<Self, Error> {
        Ok(Self {
            brand_id: BrandId::new(raw_brand_id)?,
            location_id: LocationId::new(raw_location_id)?,
            date_time_span: DateTimeSpan::new(raw_from_date, raw_to_date)?,
        })
    }
}

// Using Display to generate query string for API requests

impl Display for GetParameters {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "get/{}", self.brand_id.0)
    }
}
impl Display for LastRunParameters {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "last_run/{}/{}", self.brand_id.0, self.location_id.0)
    }
}

impl Display for RunParameters {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "run/{}/{}", self.brand_id.0, self.location_id.0)
    }
}

impl Display for SpecificParameters {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "specific/{}/{}/{}/{}",
            self.brand_id.0, self.location_id.0, self.date_time_span.0, self.date_time_span.1
        )
    }
}
