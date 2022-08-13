mod domain;
mod email_address;
mod ip_address;
mod port;
mod social;
mod url;

use crate::common::t_type::Type;

use domain::Domain;
use email_address::EmailAddress;
use ip_address::IPAddress;
use port::Port;
use social::Social;
use url::Url;

pub fn get_types() -> Vec<&'static dyn Type> {
    let types: Vec<&'static dyn Type> = vec![
        &IPAddress {},
        &Domain {},
        &EmailAddress {},
        &Port {},
        &Url {},
        &Social {},
    ];
    types
}

pub fn match_types(target: String) -> Vec<&'static dyn Type> {
    let mut matches: Vec<&'static dyn Type> = Vec::new();
    for t in get_types() {
        if t.validate(target.clone()) {
            matches.push(t);
        }
    }
    matches
}
