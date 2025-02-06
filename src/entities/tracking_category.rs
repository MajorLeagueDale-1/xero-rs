use serde::{Deserialize, Serialize};
use std::str::FromStr;
use url::Url;
use uuid::Uuid;
use crate::Client;
use crate::error::Error;


const ENDPOINT: &str = "https://api.xero.com/api.xro/2.0/TrackingCategories/";


#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum Status {
    Ok,
    Active,
    Inactive,
    Archived
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct CategoryOption {
    #[serde(rename = "TrackingOptionID")]
    pub tacking_option_id: Uuid,
    pub name: String,
    pub status: Status,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct TrackingCategory {
    #[serde(rename = "TrackingCategoryID")]
    pub tracking_category_id: Uuid,
    pub name: String,
    pub status: Status,
    pub options: Vec<CategoryOption>
}

#[derive(Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct TrackingCategoryResponse {
    pub id: Uuid,
    pub status: String,
    pub provider_name: String,
    #[serde(rename = "DateTimeUTC")]
    pub date_time_utc: String,
    pub tracking_categories: Vec<TrackingCategory>,
}

#[derive(Debug, Serialize, Default)]
pub struct ListParameters {
    pub r#where: Option<String>,
    pub order: Option<String>,
    #[serde(rename = "includeArchived")]
    pub include_archived: Option<bool>,
}

/// Retrieve a list of tracking categories.
#[instrument(skip(client))]
pub async fn list(client: &Client, parameters: ListParameters) -> crate::error::Result<TrackingCategoryResponse> {
    let response: TrackingCategoryResponse = client.get(ENDPOINT, parameters).await?;
    Ok(response)
}

/// Retrieve a single tracking category by its `tracking_category_id`.
#[instrument(skip(client))]
pub async fn get(client: &Client, category_id: Uuid) -> crate::error::Result<TrackingCategoryResponse> {
    let endpoint = Url::from_str(ENDPOINT)
        .and_then(|endpoint| endpoint.join(&category_id.to_string()))
        .map_err(|_| Error::InvalidEndpoint)?;
    let response: TrackingCategoryResponse = client.get(endpoint, Vec::<String>::default()).await?;
    Ok(response)
}
