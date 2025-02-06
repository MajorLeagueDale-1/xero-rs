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
