use std::{fs::File, io::Read};

use crate::error::Result;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("IO error")]
    Io(#[from] std::io::Error),

    #[error("Reqwest error")]
    Reqwest(#[from] reqwest::Error),

    #[error("PEM error")]
    Pem(#[from] pem::PemError),
}

pub struct Certificate;

impl Certificate {
    #[allow(dead_code)]
    pub fn from_file(path: &str) -> Result<Vec<reqwest::Certificate>, Error> {
        let mut buf = Vec::new();

        let mut file = File::open(path)?;

        file.read_to_end(&mut buf)?;

        let pems = pem::parse_many(&buf)?;

        pems.into_iter()
            .map(|pem| {
                Ok(reqwest::Certificate::from_pem(
                    &pem::encode(&pem).into_bytes(),
                )?)
            })
            .collect::<Result<Vec<_>, Error>>()
    }

    pub fn from_string(subject: &str) -> Result<Vec<reqwest::Certificate>, Error> {
        let buf: Vec<u8> = subject.into();
        let pems = pem::parse_many(&buf)?;

        pems.into_iter()
            .map(|pem| {
                Ok(reqwest::Certificate::from_pem(
                    &pem::encode(&pem).into_bytes(),
                )?)
            })
            .collect::<Result<Vec<_>, _>>()
    }
}
