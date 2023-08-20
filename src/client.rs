use std::sync::Arc;

use serde::{de::DeserializeOwned, Serialize};

use crate::{
    config::Config,
    error::Result,
    model::{
        authenticate::{AuthenticatePayload, AuthenticateResponse},
        cancel::{CancelPayload, CancelResponse},
        collect::{CollectError, CollectPayload, CollectResponse, CollectValue},
        sign::{SignPayload, SignResponse},
    },
};

/// Client for communicating with the BankID API.
///
/// The best way to instantiate the client is with an existing [`Config`]
/// using [`Client::new`]
#[derive(Clone, Debug)]
pub struct BankID {
    pub(crate) client: Arc<reqwest::Client>,
    base_url: &'static str,
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Could not parse JSON body: {1:?}")]
    InvalidJson(#[source] serde_json::Error, String),

    #[error("Error making HTTP request")]
    Http(#[from] reqwest::Error),
}

#[derive(Debug, thiserror::Error)]
pub enum InitError {
    #[error("PEM error: {0}")]
    Pem(#[from] pem::PemError),

    #[error("Reqwest error: {0}")]
    Reqwest(#[from] reqwest::Error),
}

impl BankID {
    /// Create and initialize a [`BankID`] using the given configuration.
    ///
    /// # Panics
    /// Panics if the configuration supplied leads to an invalid [`HttpClient`].
    /// Refer to the [`reqwest::ClientBuilder::build`] docs for information
    /// on situations where this might fail.
    pub fn new(config: Config) -> Result<Self, InitError> {
        let pems = pem::parse_many(config.ca)?;

        let certificates = pems
            .into_iter()
            .map(|pem| {
                // This is infallible with rustls
                reqwest::Certificate::from_pem(&pem::encode(&pem).into_bytes())
                    .expect("invalid PEM")
            })
            .collect::<Vec<_>>();

        let mut builder: reqwest::ClientBuilder = reqwest::Client::builder();
        builder = builder.identity(config.identity);

        for cert in certificates {
            builder = builder.add_root_certificate(cert);
        }

        let client = builder.build()?;

        Ok(Self {
            client: Arc::new(client),
            base_url: config.url,
        })
    }

    async fn send_payload<T: Serialize, U: DeserializeOwned>(
        &self,
        url: &str,
        payload: T,
    ) -> Result<U, Error> {
        let result = self
            .client
            .post(format!("{}{url}", &self.base_url))
            .json(&payload)
            .send()
            .await?;
        let text = result.text().await?;

        match serde_json::from_str(&text) {
            Ok(v) => Ok(v),
            Err(e) => Err(Error::InvalidJson(e, text)),
        }
    }

    /// Authenticate - Initiates an authentication order.
    ///
    /// Use the collect method to query the status of the order.
    /// If the request is successful, the orderRef and autoStartToken is returned.
    pub async fn authenticate(
        &self,
        payload: AuthenticatePayload,
    ) -> Result<AuthenticateResponse, Error> {
        self.send_payload("/auth", payload).await
    }

    /// Sign - Initiates an sign order.
    ///
    /// Use the collect method to query the status of the order.
    /// If the request is successful, the orderRef and autoStartToken is returned.
    pub async fn sign(&self, payload: SignPayload) -> Result<SignResponse, Error> {
        self.send_payload("/sign", payload).await
    }

    /// Collect - Collects the result of a sign or auth order suing the orderRef as reference.
    ///
    /// RP should keep calling collect every two seconds as long as status indicates pending.
    /// RP must abort if status indicates failed. The user identity is returned when complete.
    pub async fn collect(&self, payload: CollectPayload) -> Result<CollectResponse, Error> {
        let result = self
            .client
            .post(format!("{}/collect", &self.base_url))
            .json(&payload)
            .send()
            .await?;
        let text = result.text().await?;

        if let Ok(v) = serde_json::from_str::<CollectError>(&text) {
            return Ok(v.into());
        }

        match serde_json::from_str::<CollectValue>(&text) {
            Ok(v) => Ok(v.into()),
            Err(e) => Err(Error::InvalidJson(e, text)),
        }
    }

    /// Cancel - Cancels an ongoing sign or auth order.
    ///
    /// This is typically used if the user cancels the order in your service or app.
    pub async fn cancel(&self, payload: CancelPayload) -> Result<CancelResponse, Error> {
        self.send_payload("/cancel", payload).await
    }
}
