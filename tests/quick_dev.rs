use anyhow::Result;
use httpc_test::new_client;
use serde_json::json;

#[tokio::test]
async fn quick_dev() -> Result<()> {
    let hc = new_client("http://127.0.0.1:6379")?;
    hc.do_get("/hello?name=huchen").await?.print().await?;
    hc.do_get("/hello2/bullinbenniu").await?.print().await?;
    hc.do_get("/src/main.rs").await?.print().await?;

    hc.do_post(
        "/api/login",
        json!({
            "username": "demo",
            "password": "welcome"
        })
    ).await?.print().await?;

    hc.do_post(
        "/api/tickets",
        json!({
            "title": "TicketA"
        })
    ).await?.print().await?;

    hc.do_post(
        "/api/tickets",
        json!({
            "title": "TicketB"
        })
    ).await?.print().await?;

    hc.do_get(
        "/api/tickets",
    ).await?.print().await?;

    hc.do_delete(
        "/api/tickets/1",
    ).await?.print().await?;

    hc.do_get(
        "/api/tickets",
    ).await?.print().await?;

    Ok(())
}