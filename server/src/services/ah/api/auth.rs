use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AuthenticationRequest {
    pub client_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AuthenticationResponse {
    pub access_token: String,
    pub refresh_token: String,
    pub expires_in: i64,
}