use reqwest::{Client, ClientBuilder};
use tracing::*;
use url::Url;

pub mod error;

pub mod api;

pub struct ReCaptchaService {
    secret_key: String,
    client: Client,
}

impl ReCaptchaService {
    pub fn new(
        config: &crate::config::ServerReCaptchaConfig,
    ) -> error::ResultNew<ReCaptchaService> {
        let client = ClientBuilder::new()
            .cookie_store(false)
            .build()
            .map_err(|err| error::ErrorNew::Error { inner: err.into() })?;

        Ok(ReCaptchaService {
            secret_key: config.secret_key.clone(),
            client,
        })
    }

    pub async fn verify_token(&self, token: &str) -> error::Result<bool> {
        let url = Url::parse(&format!(
            "https://www.google.com/recaptcha/api/siteverify?secret={}&response={}",
            &self.secret_key,
            &token, // TODO sanitize inputs
        ))
        .map_err(|err| {
            error!("failed to parse Google ReCaptcha url: {}", err);
            error::Error::UrlError { inner: err }
        })?;
        info!("verifying Google ReCaptcha token...");

        let response = self
            .client
            .post(url)
            .header("Content-Length", "0")
            .send()
            .await
            .map_err(|err| {
                error!("failed to query ah search api: {}", err);
                error::Error::ReqwestError { inner: err }
            })?
            .json::<api::VerifyResponse>()
            .await
            .map_err(|err| {
                error!("failed to deserialize Google ReCaptcha response: {}", err);
                error::Error::ReqwestError { inner: err }
            })?;

        Ok(response.success)
    }
}
