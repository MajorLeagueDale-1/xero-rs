#[macro_use]
extern crate tracing;

use anyhow::Result;
use xero_rs::tracking_category::ListParameters;
use xero_rs::KeyPair;


#[tokio::test]
async fn list_items() -> Result<()> {
    tracing_subscriber::fmt().with_test_writer().init();
    let client = xero_rs::Client::from_client_credentials(KeyPair::from_env(), None).await?;

    let item_response = xero_rs::tracking_category::list(&client, ListParameters::default()).await?;
    info!("received contacts: {:?}", item_response.tracking_categories.len());
    Ok(())
}
