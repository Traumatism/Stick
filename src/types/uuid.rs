use crate::abc::Type;
use regex::Regex;

pub struct Uuid {}

impl Type for Uuid {
    fn hname(&self) -> &'static str {
        "UUID"
    }

    fn rname(&self) -> &'static str {
        "uuid"
    }

    fn validate(&self, target: String) -> bool {
        Regex::new(r"[a-f0-9]{8}\-[a-f0-9]{4}\-[a-f0-9]{4}\-[a-f0-9]{12}")
            .unwrap()
            .is_match(&target)
    }
}
