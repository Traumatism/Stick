use crate::common::t_type::Type;

pub struct IPAddress {}

impl Type for IPAddress {
    fn hname(&self) -> &'static str {
        "IP address"
    }

    fn rname(&self) -> &'static str {
        "ip_address"
    }

    fn validate(&self, target: String) -> bool {
        let parts = target.split(".").collect::<Vec<&str>>();

        if parts.len() != 4 {
            return false;
        }

        for part in parts.iter() {
            if part.parse::<u8>().is_err() {
                return false;
            }
        }

        return true;
    }
}
