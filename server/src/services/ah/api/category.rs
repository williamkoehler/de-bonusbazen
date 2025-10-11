use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Category {
    pub id: i64,
    pub slugified_name: String,
    pub name: String,
    pub images: Vec<super::misc::Image>,
    pub nix18: bool,
}
