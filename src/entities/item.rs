use std::str::FromStr;
use rust_decimal::Decimal;
use uuid::Uuid;
use serde::{Deserialize, Serialize};
use url::Url;
use crate::Client;
use crate::error::Error;


pub const ENDPOINT: &str = "https://api.xero.com/api.xro/2.0/Items/";


#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ItemDetails {
    pub unit_price: Option<Decimal>,
    pub account_code: Option<String>,
    #[serde(rename = "COGSAccountCode")]
    pub cogs_account_code: Option<String>,
    pub tax_type: Option<String>,
}


#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Item {
    #[serde(rename = "ItemID")]
    pub item_id: Uuid,
    pub code: String,
    pub description: Option<String>,
    pub purchase_description: Option<String>,
    #[serde(rename = "UpdateDateUTC")]
    pub update_date_utc: Option<String>,
    pub purchase_details: Option<ItemDetails>,
    pub sales_details: Option<ItemDetails>,
    pub name: Option<String>,
    pub is_tracked_as_inventory: Option<bool>,
    pub inventory_asset_account_code: Option<String>,
    pub total_cost_pool: Option<Decimal>,
    pub quantity_on_hand: Option<Decimal>,
    pub is_sold: Option<bool>,
    pub is_purchased: Option<bool>
}

#[derive(Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ItemResponse {
    pub id: Uuid,
    pub status: String,
    pub provider_name: String,
    #[serde(rename = "DateTimeUTC")]
    pub date_time_utc: String,
    pub items: Vec<Item>,
}


#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct ListParameters {
    pub r#where: Option<String>,
    pub order: Option<String>,
}


/// Retrieve a list of items.
#[instrument(skip(client))]
pub async fn list(client: &Client, parameters: ListParameters) -> crate::error::Result<ItemResponse> {
    client.get(ENDPOINT, parameters).await
}

/// Retrieve a single item by its `item_id`.
#[instrument(skip(client))]
pub async fn get(client: &Client, item_id: Uuid) -> crate::error::Result<ItemResponse> {
    let endpoint = Url::from_str(ENDPOINT)
        .and_then(|endpoint| endpoint.join(&item_id.to_string()))
        .map_err(|_| Error::InvalidEndpoint)?;
    client.get(endpoint, Vec::<String>::default()).await
}
