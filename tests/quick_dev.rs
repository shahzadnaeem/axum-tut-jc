use anyhow::Result;
use chrono::prelude::*;
use serde_json::json;

#[tokio::test]
async fn quick_dev() -> Result<()> {
    let client = httpc_test::new_client("http://localhost:8080")?;

    client.do_get("/hello?name=Bob").await?.print().await?;

    client.do_get("/hello2/Shaz").await?.print().await?;

    // client.do_get("/src/main.rs").await?.print().await?;

    let login_req = client.do_post(
        "/api/login",
        json!({
            "username": "user1",
            "password": "123456",
        }),
    );

    login_req.await?.print().await?;

    let local_now = Local::now();
    let title_text = local_now.format("Ticket created at %H:%M:%S").to_string();

    let create_ticket_req = client.do_post("/api/tickets", json!({ "title": title_text }));

    create_ticket_req.await?.print().await?;

    let get_tickets_req = client.do_get("/api/tickets");

    get_tickets_req.await?.print().await?;

    Ok(())
}
