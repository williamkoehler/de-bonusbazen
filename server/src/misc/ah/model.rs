use serde::{Deserialize, Serialize};

#[serde_with::skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
pub struct Product {
    pub id: u64,
    pub name: String,
    pub image: Option<String>,
    pub bonus: bool,
    pub price: Option<f64>,
    pub price_before_bonus: Option<f64>,
}