use reqwest::{Client, ClientBuilder, RequestBuilder, Url};
use tracing::*;

pub mod api;
pub mod error;

#[derive(Clone)]
pub struct AhService {
    client: Client,
}

impl AhService {
    pub fn new() -> error::ResultNew<Self> {
        let client = ClientBuilder::new()
            .cookie_store(true)
            .build()
            .map_err(|err| error::ErrorNew::Error { inner: err.into() })?;

        Ok(AhService { client })
    }

    fn prepare_request(mut request_builder: RequestBuilder, token: Option<&str>) -> RequestBuilder {
        if let Some(token) = token {
            request_builder = request_builder.header("Authorization", format!("Bearer {}", token));
        }

        request_builder
            .header("Host", "api.ah.nl")
            .header("Accept", "application/json")
            .header("Accept-Encoding", "gzip, deflate, br")
            .header("User-Agent", "Appie/8.8.2 Model/phone Android/7.0-API24")
            .header("X-Application", "AHWEBSHOP")
    }

    pub async fn authenticate(&self) -> error::Result<String> {
        let url =
            Url::parse("https://api.ah.nl/mobile-auth/v1/auth/token/anonymous").map_err(|err| {
                error!("failed to parse ah url: {}", err);
                error::Error::UrlError { inner: err }
            })?;

        info!(url = url.to_string(), "authenticating with ah api");

        let response: api::auth::AuthenticationResponse =
            Self::prepare_request(self.client.post(url), None)
                .json(&api::auth::AuthenticationRequest {
                    client_id: "appie".to_string(),
                })
                .send()
                .await
                .map_err(|err| {
                    error!("failed to query ah auth api: {}", err);
                    error::Error::ReqwestError { inner: err }
                })?
                .json()
                .await
                .map_err(|err| {
                    error!("failed to deserialize ah auth response: {}", err);
                    error::Error::ReqwestError { inner: err }
                })?;

        Ok(response.access_token)
    }

    pub async fn get_categories(&self, token: &str) -> error::Result<Vec<api::category::Category>> {
        let url = Url::parse("https://api.ah.nl/mobile-services/v1/product-shelves/categories")
            .map_err(|err| {
                error!("failed to parse ah categories url: {}", err);
                error::Error::UrlError { inner: err }
            })?;
        info!(url = url.to_string(), "getting ah categories");

        Self::prepare_request(self.client.get(url), Some(token))
            .send()
            .await
            .map_err(|err| {
                error!("failed to query ah search api: {}", err);
                error::Error::ReqwestError { inner: err }
            })?
            .json::<Vec<api::category::Category>>()
            .await
            .map_err(|err| {
                error!("failed to deserialize ah search response: {}", err);
                error::Error::ReqwestError { inner: err }
            })
    }

    pub async fn search_products(
        &self,
        token: &str,
        query: &str,
        page: usize,
        limit: usize,
        category_id: Option<i64>,
    ) -> error::Result<api::product::SearchProducts> {
        let mut url =
            Url::parse("https://api.ah.nl/mobile-services/product/search/v2").map_err(|err| {
                error!("failed to parse ah search url: {}", err);
                error::Error::UrlError { inner: err }
            })?;
        {
            let mut query_pairs = url.query_pairs_mut();
            query_pairs.append_pair("query", &query);
            query_pairs.append_pair("page", &page.to_string());
            query_pairs.append_pair("size", &limit.to_string());

            if let Some(category_id) = category_id {
                query_pairs.append_pair("taxonomyId", &category_id.to_string());
            }
        }
        info!(url = url.to_string(), "searching ah products");

        Self::prepare_request(self.client.get(url), Some(token))
            .send()
            .await
            .map_err(|err| {
                error!("failed to query ah search api: {}", err);
                error::Error::ReqwestError { inner: err }
            })?
            .json::<api::product::SearchProducts>()
            .await
            .map_err(|err| {
                error!("failed to deserialize ah search response: {}", err);
                error::Error::ReqwestError { inner: err }
            })
    }
}
