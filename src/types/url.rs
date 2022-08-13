use crate::common::t_type::Type;

use super::domain::Domain;
use super::ip_address::IPAddress;
use super::port::Port;

pub struct Url {}

impl Type for Url {
    fn hname(&self) -> &'static str {
        "URL"
    }

    fn rname(&self) -> &'static str {
        "url"
    }

    fn validate(&self, target: String) -> bool {
        if !(target.starts_with("https://")) && !(target.starts_with("http://")) {
            return false;
        }

        let parts = target.split("/").collect::<Vec<&str>>();

        let host = parts[2].to_string();

        // http(s)://domain.tld/
        if !(&Domain {}).validate(host.clone())
        // http(s)://0.0.0.0:80/
            && !(&Port {}).validate(host.clone())
        // http(s)://0.0.0.0/
            && !(&IPAddress {}).validate(host.clone())
        {
            return false;
        }

        return true;
    }
}
