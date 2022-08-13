use crate::common::t_type::Type;

pub struct Social {}

const SUPPORTED: [&str; 7] = [
    "twitter",
    "instagram",
    "github",
    "facebook",
    "tiktok",
    "pornhub",
    "discord_id",
];

impl Type for Social {
    fn hname(&self) -> &'static str {
        "Social"
    }

    fn rname(&self) -> &'static str {
        "social"
    }

    fn validate(&self, target: String) -> bool {
        if !target.contains("@") {
            return false;
        }

        if !SUPPORTED.contains(&target.split("@").collect::<Vec<&str>>()[0]) {
            return false;
        }

        return true;
    }
}
