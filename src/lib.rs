//! It's rate limited and HTTPS access is a [paid feature](http://ip-api.com/docs/pro).
//! If the rate limiter catches you going over the 150 requests per minute you will be banned by IP until you [unban yourself](http://ip-api.com/docs/unban).
//! You can also view overall usage [statistics here](http://ip-api.com/docs/statistics).
//!
//! This information is likely not exact. Take this data with a grain of salt.
//!
//! Example
//!
//!```rust,ignore
//!extern crate ip_api;
//!
//!use ip_api::GeoIp;
//!
//!let fb = match GeoIp::new("www.facebook.com", false) {
//!    Err(e) => {
//!        eprintln!("{}", e);
//!        return;
//!    },
//!    Ok(geo_ip) => geo_ip
//!};


extern crate reqwest;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

use std::error::Error;
use serde_json::Value;

pub use error::IpApiError;

mod error;

/// Information about an IP address.
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct GeoIp {
    /// Get the country. (e.g. "United States")
    pub country: String,
    /// Get the country code. (e.g. "US")
    pub country_code: String,
    /// Get the region. (e.g. "CA" or "10")
    pub region: String,
    /// Get the region name. (e.g. "California")
    pub region_name: String,
    /// Get the city. (e.g. "Mountain View")
    pub city: String,
    /// Get the zip code. (e.g. "94043")
    pub zip: String,
    /// Get the location as a tuple of latitude and longitude.
    pub lat: f32,
    #[serde(rename = "lon")]
    pub lng: f32,
    /// Get the timezone. (e.g. "America/Los_Angeles")
    pub timezone: String,
    /// Get the internet service provider. (e.g. "Google")
    pub isp: String,
    /// Get the organization. (e.g. "Google")
    pub org: String,
    /// Get the [autonomous system](https://en.wikipedia.org/wiki/Autonomous_system_(Internet)) number and name. (e.g. "AS15169 Google Inc.")
    #[serde(rename = "as")]
    pub as_nn: String,
    /// Get whether the IP is a cellular connection.
    pub mobile: bool,
    /// Get whether the IP is a known proxy.
    pub proxy: bool,
}


impl GeoIp {
    pub fn new(host: Option<&str>, ssl: bool) -> Result<GeoIp, IpApiError> {
        let url = format!("http{}://ip-api.com/json/{}?fields=258047", if ssl { "s" } else { "" }, host.unwrap_or(""));

        let json: Value = reqwest::blocking::get(&url)
            .map_err(|e| IpApiError::OtherError(format!("{}", e.description())))?
            .json::<Value>()
            .map_err(|e| IpApiError::OtherError(format!("Error interpreting body as json; the body is: {}", e.description())))?;

        match json.get("status").as_ref() {
            Some(Value::String(s)) if s == "success" => {
                let res: GeoIp = serde_json::from_value(json)
                    .map_err(|e| IpApiError::OtherError(format!("Error deserialization json to GeoIp, err: {}", e.description())))?;
                Ok(res)
            }
            Some(Value::String(s)) if s == "fail" => {
                let err = match json.get("message").as_ref() {
                    Some(Value::String(s)) => {
                        match s.as_str() {
                            "private range" => IpApiError::PrivateRange,
                            "reserved range" => IpApiError::ReservedRange,
                            "invalid query" => IpApiError::InvalidQuery,
                            "quota" => IpApiError::InvalidQuery,
                            _ => unexpected_json(&json.to_string(), "unknown error message")
                        }
                    }
                    _ => unexpected_json(&json.to_string(), "unknown error message")
                };
                Err(err)
            }
            _ => Err(unexpected_json(&json.to_string(), "unknown error message")),
        }
    }
}

fn unexpected_json(body: &str, reason: &str) -> IpApiError {
    IpApiError::OtherError(format!("Unexpected response: {}; body is: {}", reason, body))
}
