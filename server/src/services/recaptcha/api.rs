use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct VerifyResponse {
    pub success: bool,
    pub challenge_ts: Option<String>,
    pub hostname: Option<String>,
    pub error_codes: Option<Vec<String>>,
}
