use std::str::FromStr;
use chrono::{NaiveDateTime};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use url::Url;
use uuid::Uuid;
use crate::{contact, Client};
use crate::error::Error;
use crate::generic::{Address, Pagination};
use crate::line_item::LineItem;

pub const ENDPOINT: &str = "https://api.xero.com/api.xro/2.0/CreditNotes/";


#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "ACCPAYCREDIT")]
    AccountsPayable,

    #[serde(rename = "ACCRECCREDIT")]
    AccountsReceivable,
}


#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Payment {
    #[serde(rename = "PaymentID")]
    pub payment_id: Uuid,
    pub date: String,
    pub amount: Decimal,
    pub reference: Option<String>,
    pub currency_rate: Decimal,
    pub has_account: bool,
    pub has_validation_errors: bool
}


#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct InvoiceSummary {
    #[serde(rename = "InvoiceID")]
    pub invoice_id: Uuid,
    pub invoice_number: String,
    pub payments: Vec<Payment>,
    // pub credit_notes: Vec<CreditNote>,
    pub is_discounted: bool,
    pub invoice_addresses: Vec<Address>,
    pub has_errors: bool,
    pub line_items: Vec<LineItem>,
}


#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Allocation {
    #[serde(rename = "AllocationID")]
    pub allocation_id: Uuid,
    pub amount: Decimal,
    pub date: String,
    pub invoice: InvoiceSummary,
}


#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum Status {
    Draft,
    Submitted,
    Deleted,
    Authorised,
    Paid,
    Voided,
}


#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct CreditNote {
    #[serde(rename = "CreditNoteID")]
    pub credit_note_id: Uuid,
    pub credit_note_number: String,
    pub payments: Vec<Payment>,
    pub has_errors: bool,
    pub invoice_addresses: Option<Vec<Address>>,
    pub currency_rate: Decimal,
    pub r#type: Type,
    pub reference: String,
    pub remaining_credit: Decimal,
    pub allocations: Vec<Allocation>,
    pub has_attachments: bool,
    pub contact: contact::Contact,
    #[serde(rename = "DateString")]
    pub date: NaiveDateTime,
    #[serde(rename = "BrandingThemeID")]
    pub branding_theme_id: Option<String>,
    pub status: Status,
    pub line_amount_types: String,
    pub line_items: Vec<LineItem>,
    pub sub_total: Decimal,
    pub total_tax: Decimal,
    pub total: Decimal,
    #[serde(rename = "UpdatedDateUTC")]
    pub updated_date_utc: String,
    pub currency_code: String,
    pub fully_paid_on_date: Option<String>,
}


#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct CreditNoteResponse {
    pub id: Uuid,
    pub status: String,
    pub provider_name: String,
    #[serde(rename = "DateTimeUTC")]
    pub date_time_utc: String,
    #[serde(rename = "pagination")]
    pub pagination: Option<Pagination>,
    pub credit_notes: Vec<CreditNote>,
}

#[derive(Debug, Serialize, Default)]
pub struct ListParameters {
    pub r#where: Option<String>,
    pub page: Option<u32>,
    pub order: Option<String>,
}


/// List invoices based on parameters
#[instrument(skip(client))]
pub async fn list(client: &Client, parameters: ListParameters) -> crate::error::Result<CreditNoteResponse> {
    let response: CreditNoteResponse = client.get(ENDPOINT, parameters).await?;
    Ok(response)
}


/// Retrieve a single invoice by its `invoice_id`.
#[instrument(skip(client))]
pub async fn get(client: &Client, invoice_id: Uuid) -> crate::error::Result<CreditNoteResponse> {
    let endpoint = Url::from_str(ENDPOINT)
        .and_then(|endpoint| endpoint.join(&invoice_id.to_string()))
        .map_err(|_| Error::InvalidEndpoint)?;
    let response: CreditNoteResponse = client.get(endpoint, Vec::<String>::default()).await?;
    Ok(response)
}
