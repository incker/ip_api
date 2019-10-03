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
//!
//!println!("{}", fb.country().unwrap());
//!```

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
    country: String,
    country_code: String,
    region: String,
    region_name: String,
    city: String,
    zip: String,
    lat: f32,
    lon: f32,
    timezone: String,
    isp: String,
    org: String,
    #[serde(rename = "as")]
    as_nn: String,
    mobile: bool,
    proxy: bool,
}

impl GeoIp {
    /// Get information on an IP address or domain name.
    /// If no host is provided then it will return information on your current IP.
    pub fn new(host: Option<&str>, https: bool) -> Result<GeoIp, IpApiError> {
        let url = format!(
            "{}://ip-api.com/json/{}?fields=258047",
            if https { "https" } else { "http" },
            host.unwrap_or("")
        );

        let json: Value = reqwest::get(&url)
            .map_err(|e| IpApiError::OtherError(format!("{}", e.description())))?
            .json()
            .map_err(|e| IpApiError::OtherError(format!("Error interpreting body as json; the body is: {}", e.description())))?;


        match json.get("status") {
            Some(&Value::String(ref s)) => {
                match s.as_ref() {
                    "success" => {
                        serde_json::from_value(json)
                            .map_err(|e| IpApiError::OtherError(format!("Error deserialization json to GeoIp, err: {}", e.description())))
                    },
                    "fail" => {
                        match json.get("message") {
                            Some(&Value::String(ref s)) => {
                                match s.as_ref() {
                                    "private range" => Err(IpApiError::PrivateRange),
                                    "reserved range" => Err(IpApiError::ReservedRange),
                                    "invalid query" => Err(IpApiError::InvalidQuery),
                                    "quota" => Err(IpApiError::Quota),
                                    _ => Err(unexpected_json(&json.to_string(), "unknown error message"))
                                }
                            }
                            _ => Err(unexpected_json(&json.to_string(), "unexpected message type"))
                        }
                    }
                    _ => Err(unexpected_json(&json.to_string(), "invalid status value"))
                }
            }
            _ => Err(unexpected_json(&json.to_string(), "invalid status"))
        }

    }

    /// Get the country. (e.g. "United States")
    pub fn country(&self) -> Option<String> {
        as_option(&self.country)
    }

    /// Get the country code. (e.g. "US")
    pub fn country_code(&self) -> Option<String> {
        as_option(&self.country_code)
    }

    /// Get the region. (e.g. "CA" or "10")
    pub fn region(&self) -> Option<String> {
        as_option(&self.region)
    }

    /// Get the region name. (e.g. "California")
    pub fn region_name(&self) -> Option<String> {
        as_option(&self.region_name)
    }

    /// Get the city. (e.g. "Mountain View")
    pub fn city(&self) -> Option<String> {
        as_option(&self.city)
    }

    /// Get the zip code. (e.g. "94043")
    pub fn zip_code(&self) -> Option<String> {
        as_option(&self.zip)
    }

    /// Get the location as a tuple of latitude and longitude.
    pub fn location(&self) -> Option<(f32, f32)> {
        if self.lat == 0.0 && self.lon == 0.0 {
            None
        } else {
            Some((self.lat, self.lon))
        }
    }

    /// Get the timezone. (e.g. "America/Los_Angeles")
    pub fn timezone(&self) -> Option<String> {
        as_option(&self.timezone)
    }

    /// Get the internet service provider. (e.g. "Google")
    pub fn isp(&self) -> Option<String> {
        as_option(&self.isp)
    }

    /// Get the organization. (e.g. "Google")
    pub fn organization(&self) -> Option<String> {
        as_option(&self.org)
    }

    /// Get the [autonomous system](https://en.wikipedia.org/wiki/Autonomous_system_(Internet)) number and name. (e.g. "AS15169 Google Inc.")
    pub fn as_nn(&self) -> Option<String> {
        as_option(&self.as_nn)
    }

    /// Get whether the IP is a cellular connection.
    pub fn is_mobile(&self) -> bool {
        self.mobile
    }

    /// Get whether the IP is a known proxy.
    pub fn is_proxy(&self) -> bool {
        self.proxy
    }
}

fn unexpected_json(body: &str, reason: &str) -> IpApiError {
    IpApiError::OtherError(format!("Unexpected response: {}; body is: {}", reason, body))
}

fn as_option(string: &String) -> Option<String> {
    if string.is_empty() {
        None
    } else {
        Some(string.clone())
    }
}
