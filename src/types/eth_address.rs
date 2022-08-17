use regex::Regex;

use crate::abc::Type;

pub struct EthAddress {}

impl Type for EthAddress {
    fn hname(&self) -> &'static str {
        "Ethereum cryptocurrency address"
    }

    fn rname(&self) -> &'static str {
        "eth_address"
    }

    fn validate(&self, target: String) -> bool {
        Regex::new(r"0x[0-9a-f]{40}").unwrap().is_match(&target)
    }
}
