use crate::abc::Type;

use super::ip_address::IPAddress;

pub struct Port {}

impl Type for Port {
    fn hname(&self) -> &'static str {
        "Port"
    }

    fn rname(&self) -> &'static str {
        "port"
    }

    fn validate(&self, target: String) -> bool {
        if !target.contains(":") {
            return false;
        }

        let parts = target.split(":").collect::<Vec<&str>>();

        if parts.len() != 2
            || !(&IPAddress {}).validate(parts[0].to_string())
            || parts[1].parse::<u16>().is_err()
        {
            return false;
        }

        return true;
    }
}
