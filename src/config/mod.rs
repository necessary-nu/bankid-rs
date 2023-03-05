//! BankID configuration objects.
//!
//! Used to populate [`Config`] that is ultimately used to construct a [`Client`][crate::Client].

#[derive(Debug)]
pub struct Config {
    pub identity: reqwest::Identity,
    pub url: &'static str,
    pub ca: &'static str,
}

impl Config {
    pub fn prod(identity: reqwest::Identity) -> Self {
        Self {
            identity,
            url: API_URL_PROD,
            ca: CA_PROD,
        }
    }
}

pub use reqwest::Identity;

#[allow(dead_code)]
pub const API_URL_TEST: &str = "https://appapi2.test.bankid.com/rp/v5.1";
#[allow(dead_code)]
pub const API_URL_PROD: &str = "https://appapi2.bankid.com/rp/v5.1";

#[allow(dead_code)]
pub const CA_TEST: &str = include_str!("../../resources/test.ca");
#[allow(dead_code)]
pub const CA_PROD: &str = include_str!("../../resources/production.ca");
#[allow(dead_code)]
pub const P12_TEST: &[u8] = include_bytes!("../../resources/testcert.p12");
