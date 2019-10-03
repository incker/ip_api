extern crate ip_api;

use ip_api::*;

fn main() {
    let us = match GeoIp::new(None, false) {
        Err(e) => {
            eprintln!("{}", e);
            return;
        },
        Ok(geo_ip) => geo_ip
    };

    println!("Our country is {}",
             us.country().unwrap_or("unknown".to_owned()));

    let google = match GeoIp::new(Some("www.google.com"), false) {
        Err(e) => {
            eprintln!("{}", e);
            return;
        },
        Ok(geo_ip) => geo_ip
    };

    println!("Google's ISP is {}",
             google.isp().unwrap_or("unknown".to_owned()));
}
