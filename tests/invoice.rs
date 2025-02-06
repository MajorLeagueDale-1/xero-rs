#[macro_use]
extern crate tracing;

use std::sync::Once;
use anyhow::Result;
use xero_rs::invoice::ListParameters;
use xero_rs::KeyPair;

static LOGGING_CONFIGURED: Once = Once::new();

fn setup_logging() {
    LOGGING_CONFIGURED.call_once(|| {
        tracing_subscriber::fmt()
            .with_env_filter("info,xero_rs=trace")
            .with_test_writer()
            .init()
    });
}

#[tokio::test]
async fn get_invoices() -> Result<()> {
    setup_logging();
    let client = xero_rs::Client::from_client_credentials(KeyPair::from_env(), None).await?;

    let invoices_response = xero_rs::invoice::list(&client, ListParameters::default()).await?;
    let invoices = invoices_response.invoices;
    debug!("found {:?} invoices", invoices.len());

    let invoice_from_list = invoices.first().unwrap();

    let invoice_response = xero_rs::invoice::get(&client, invoice_from_list.invoice_id).await?;
    let invoice = invoice_response.invoices.first().unwrap();
    assert_eq!(invoice_from_list.invoice_id, invoice.invoice_id);

    Ok(())
}


#[tokio::test]
async fn get_invoices_by_page() -> Result<()> {
    setup_logging();

    let client = xero_rs::Client::from_client_credentials(KeyPair::from_env(), None).await?;

    let mut parameters = ListParameters::default();
    parameters.page = Some(1);

    let invoice_response = xero_rs::invoice::list(&client, parameters).await?;

    assert!(invoice_response.invoices.len() <= 100);
    assert!(invoice_response.pagination.is_some());
    assert_eq!(invoice_response.pagination.unwrap().page, 1);

    Ok(())
}
