use super::Domain;
use crate::abc::Type;

pub struct EmailAddress {}

impl Type for EmailAddress {
    fn hname(&self) -> &'static str {
        "Email address"
    }

    fn rname(&self) -> &'static str {
        "email_address"
    }

    fn validate(&self, target: String) -> bool {
        let chars = "-abcdefghijklmnopqrstwxyz123456789.+@";

        for c in target.chars() {
            if !chars.contains(c) {
                return false;
            }
        }

        let parts = target.split("@").collect::<Vec<&str>>();

        if parts.len() != 2 {
            return false;
        }

        if !(&Domain {}).validate(parts[1].to_string()) {
            return false;
        }

        return true;
    }
}
