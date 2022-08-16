use crate::abc::Type;

pub struct Uuid {}

impl Type for Uuid {
    fn hname(&self) -> &'static str {
        "UUID"
    }

    fn rname(&self) -> &'static str {
        "uuid"
    }

    fn validate(&self, target: String) -> bool {
        false // Not implemented yet
    }
}
