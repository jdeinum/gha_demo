use crate::utils::spawn_app;
use anyhow::Context;
use anyhow::Result;
use axum::http::StatusCode;

#[tokio::test]
pub async fn test_health() -> Result<()> {
    // spawn our app
    let app = spawn_app().await.context("spawn testing app")?;

    // send the request
    let endpoint = format!("{}/health", app.address);
    let resp = app
        .api_client
        .get(endpoint)
        .send()
        .await
        .context("send request")?;

    // check status
    assert_eq!(resp.status(), StatusCode::OK);

    Ok(())
}
