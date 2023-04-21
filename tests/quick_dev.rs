use core::time;
use std::thread;

use anyhow::Result;
use chrono::prelude::*;
// NOTE: Uses local modified version of https_test! See Cargo.toml
use httpc_test::Client;
use serde_json::json;

#[tokio::test]
async fn quick_dev() -> Result<()> {
    let client = httpc_test::new_client("http://localhost:8080")?;

    client.do_get("/hello?name=Bob").await?.print().await?;

    client.do_get("/hello2/Shaz").await?.print().await?;

    let get_tickets_req = client.do_get("/api/tickets");

    get_tickets_req.await?.print().await?;

    let login_req = client.do_post(
        "/api/login",
        json!({
            "username": "user1",
            "password": "123456",
        }),
    );

    login_req.await?.print().await?;

    // Two tickets
    create_ticket(&client).await?;
    create_ticket(&client).await?;

    // Get the tickets
    let get_tickets_req = client.do_get("/api/tickets");
    get_tickets_req.await?.print().await?;

    // Delete ticket id=1
    let delete_ticket_req = client.do_delete("/api/tickets/1");
    delete_ticket_req.await?.print().await?;

    Ok(())
}

async fn create_ticket(client: &Client) -> Result<()> {
    let title_text = new_ticket_title();
    let create_ticket_req = client.do_post("/api/tickets", json!({ "title": title_text }));
    create_ticket_req.await?.print().await?;

    Ok(())
}

fn new_ticket_title() -> String {
    thread::sleep(time::Duration::from_secs(1));
    let local_now = Local::now();
    let title_text = local_now.format("Ticket created at %H:%M:%S").to_string();

    title_text
}
