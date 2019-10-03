use std::fmt;
use std::error::Error;

#[derive(Debug)]
pub enum IpApiError {
    /// The IP address is part of a private range
    PrivateRange,
    /// The IP address is part of a reserved range
    ReservedRange,
    /// Invalid IP address or domain name
    InvalidQuery,
    /// Quota exceeded, go unban yourself
    Quota,
    /// Some other error has occurred
    OtherError(String)
}

impl Error for IpApiError {
    fn description(&self) -> &str {
        match *self {
            IpApiError::PrivateRange => "The IP address is part of a private range",
            IpApiError::ReservedRange => "The IP address is part of a reserved range",
            IpApiError::InvalidQuery => "Invalid IP address or domain name",
            IpApiError::Quota => "Quota exceeded, go unban yourself",
            IpApiError::OtherError(_) => "Some other error has occurred"
        }
    }
}

impl fmt::Display for IpApiError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Error: {}", self.description())
    }
}
