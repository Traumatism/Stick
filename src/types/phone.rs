use crate::abc::Type;

pub struct Phone {}

impl Type for Phone {
    fn hname(&self) -> &'static str {
        "Phone number"
    }

    fn rname(&self) -> &'static str {
        "phone_number"
    }

    fn validate(&self, target: String) -> bool {
        false // Not implemented yet
    }
}
