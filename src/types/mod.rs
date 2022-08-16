mod btc_address;
mod domain;
mod email_address;
mod eth_address;
mod ip_address;
mod phone;
mod port;
mod social;
mod url;
mod uuid;

use crate::abc::Type;

use btc_address::BtcAddress;
use domain::Domain;
use email_address::EmailAddress;
use eth_address::EthAddress;
use ip_address::IPAddress;
use phone::Phone;
use port::Port;
use social::Social;
use url::Url;
use uuid::Uuid;

pub fn get_types() -> Vec<&'static dyn Type> {
    let types: Vec<&'static dyn Type> = vec![
        &IPAddress {},
        &Domain {},
        &EmailAddress {},
        &Port {},
        &Url {},
        &Social {},
        &Uuid {},
        &Phone {},
        &EthAddress {},
        &BtcAddress {},
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
