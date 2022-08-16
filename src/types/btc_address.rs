use crate::abc::Type;

pub struct BtcAddress {}

impl Type for BtcAddress {
    fn hname(&self) -> &'static str {
        "Bitcoin cryptocurrency address"
    }

    fn rname(&self) -> &'static str {
        "btc_address"
    }

    fn validate(&self, target: String) -> bool {
        false // Not implemented yet
    }
}
