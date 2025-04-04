use serde::{Deserialize, Serialize};


/// Describes the structure for pagination.
#[derive(Debug, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct Pagination {
    pub page: u32,
    pub page_size: u32,
    pub page_count: u32,
    pub item_count: u32
}


#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum AddressType {
    Street,
    PoBox
}


#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Address {
    pub address_type: AddressType,
    pub address_line_1: String,
    pub address_line_2: String,
    pub address_line_3: String,
    pub address_line_4: String,
    pub city: String,
    pub region: String,
    pub postal_code: String,
    pub country: String,
    pub attention_to: String
}
