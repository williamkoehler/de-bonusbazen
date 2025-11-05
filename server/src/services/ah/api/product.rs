use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SearchProducts {
    pub page: Page,
    pub products: Vec<super::product::Product>,
    pub links: Option<Links>,
    pub filters: Option<Vec<Filter>>,
    pub sort_on: Option<Vec<String>>,
    pub configuration: Option<Configuration>,
    pub ads: Option<Vec<serde_json::Value>>,
    pub taxonomy_nodes: Option<Vec<serde_json::Value>>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Page {
    pub size: i64,
    pub total_elements: i64,
    pub total_pages: i64,
    pub number: i64,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Links {
    pub first: Option<Link>,
    pub current: Option<Link>,
    pub next: Option<Link>,
    pub last: Option<Link>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Link {
    pub href: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Filter {
    pub id: String,
    pub label: String,
    pub options: Vec<FilterOption>,
    #[serde(rename = "type")]
    pub filter_type: String,
    pub boolean_filter: bool,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FilterOption {
    pub id: String,
    pub label: String,
    pub count: u32,
    pub display: bool,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Configuration {
    pub google_banners: Option<GoogleBanners>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GoogleBanners {
    pub ad_unit_main_path: String,
    pub ad_unit_secondary_path: String,
    pub custom_template_id: String,
    pub div_gpt_ad: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Product {
    pub webshop_id: u64,
    pub hq_id: u64,
    pub title: String,
    pub sales_unit_size: Option<String>,
    pub unit_price_description: Option<String>,
    pub images: Vec<super::misc::Image>,
    pub price_before_bonus: Option<f64>,
    pub current_price: Option<f64>,
    pub order_availability_status: Option<String>,
    pub main_category: Option<String>,
    pub sub_category: Option<String>,
    pub brand: Option<String>,
    pub shop_type: Option<String>,
    pub available_online: Option<bool>,
    pub is_previously_bought: Option<bool>,
    pub description_highlights: Option<String>,
    pub property_icons: Option<Vec<String>>,
    pub nutriscore: Option<String>,
    pub nix18: Option<bool>,
    pub is_stapel_bonus: Option<bool>,
    pub extra_descriptions: Option<Vec<String>>,
    pub is_bonus: Option<bool>,
    pub description_full: Option<String>,
    pub is_orderable: Option<bool>,
    pub is_infinite_bonus: Option<bool>,
    pub is_sample: Option<bool>,
    pub is_sponsored: Option<bool>,
    pub is_virtual_bundle: Option<bool>,
    pub discount_labels: Option<Vec<DiscountLabel>>,
    pub min_best_before_days: Option<u32>,
    pub bonus_start_date: Option<String>,
    pub bonus_end_date: Option<String>,
    pub bonus_mechanism: Option<String>,
    pub bonus_segment_id: Option<i64>,
    pub bonus_segment_description: Option<String>,
    pub label_type: Option<String>,
    pub multiple_item_promotion: Option<bool>,
    pub product_count: Option<u32>,
    pub virtual_bundle_items: Option<Vec<VirtualBundleItem>>,
}

impl Into<crate::databases::postgres::model::AhProduct> for Product {
    fn into(self) -> crate::databases::postgres::model::AhProduct {
        crate::databases::postgres::model::AhProduct {
            id: self.hq_id,
            name: self.title,
            image: self
                .images
                .iter()
                .find(|image| image.width * image.height > 200 * 200)
                .map(|image| image.url.clone()),
            bonus: self.is_bonus.map_or(false, |x| x),
            price: self.current_price,
            price_before_bonus: self.price_before_bonus,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DiscountLabel {
    pub code: String,
    pub default_description: String,
    pub count: Option<u32>,
    pub price: Option<f64>,
    pub unit: Option<String>,
    pub percentage: Option<u32>,
    pub precise_percentage: Option<f64>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VirtualBundleItem {
    pub product_id: u64,
    pub quantity: u32,
}
