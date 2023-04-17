use anyhow::Result;
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

    Ok(())
}
