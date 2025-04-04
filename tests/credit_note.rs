#[macro_use]
extern crate tracing;

use std::sync::Once;
use anyhow::Result;
use xero_rs::credit_note::ListParameters;
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
async fn get_credit_notes() -> Result<()> {
    setup_logging();
    let client = xero_rs::Client::from_client_credentials(KeyPair::from_env(), None).await?;

    let credit_notes_response = xero_rs::credit_note::list(&client, ListParameters::default()).await?;
    let credit_notes = credit_notes_response.credit_notes;
    debug!("found {:?} credit_notes", credit_notes.len());

    let credit_note_from_list = credit_notes.first().unwrap();

    let credit_note_response = xero_rs::credit_note::get(&client, credit_note_from_list.credit_note_id).await?;
    let credit_note = credit_note_response.credit_notes.first().unwrap();
    assert_eq!(credit_note_from_list.credit_note_id, credit_note.credit_note_id);

    Ok(())
}


#[tokio::test]
async fn get_credit_notes_by_page() -> Result<()> {
    setup_logging();

    let client = xero_rs::Client::from_client_credentials(KeyPair::from_env(), None).await?;

    let mut parameters = ListParameters::default();
    parameters.page = Some(1);

    let credit_note_response = xero_rs::credit_note::list(&client, parameters).await?;

    assert!(credit_note_response.credit_notes.len() <= 100);
    assert!(credit_note_response.pagination.is_some());
    assert_eq!(credit_note_response.pagination.unwrap().page, 1);

    Ok(())
}

