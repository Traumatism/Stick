use super::IPAddress;
use crate::common::t_type::Type;

pub struct Domain {}

impl Type for Domain {
    fn hname(&self) -> &'static str {
        "Domain"
    }

    fn rname(&self) -> &'static str {
        "domain"
    }

    fn validate(&self, target: String) -> bool {
        let chars = "-abcdefghijklmnopqrstwxyz123456789.";

        if !target.contains(".") || target.ends_with(".") || target.starts_with(".") {
            return false;
        }

        for c in target.chars() {
            if !chars.contains(c) {
                return false;
            }
        }

        if (&IPAddress {}).validate(target) {
            return false;
        }

        return true;
    }
}
