use axum::response::IntoResponse;
use plugind_core::context::InvokeResult;
use serde::Deserialize;
use serde_json::Value;

use crate::{context::DAEMON_CONTEXT, invoke::invoke_plugin};

mod context;
mod invoke;
mod upload;

#[derive(Deserialize)]
struct InvokePayload {
    lib_name: String,
    params: Value
}

async fn invoke_handler(axum::Json(payload): axum::Json<InvokePayload>) -> impl IntoResponse {
    match invoke_plugin(payload.lib_name, serde_json::to_vec(&payload.params).unwrap()).await {
        InvokeResult::Ok(output_buffer) => (axum::http::StatusCode::OK, output_buffer),
        InvokeResult::Err(err) => (axum::http::StatusCode::INTERNAL_SERVER_ERROR, err.to_string().as_bytes().to_vec())
    }
}

#[tokio::main(flavor = "current_thread")]
async fn main() -> anyhow::Result<()> {

    let addr_listen = std::env::var("ADDR_LISTEN")?;

    let config = aws_config::load_from_env().await;
    let client = aws_sdk_s3::Client::new(&config);

    {
        DAEMON_CONTEXT.write().await.set_storage(&client);
    }

    let router = axum::Router::new()
        .route("/", axum::routing::post(invoke_handler))
        .route("/plugins", axum::routing::post(upload::plugin_upload));

    let listener = tokio::net::TcpListener::bind(addr_listen).await?;

    axum::serve(listener, router).await?;

    Ok(())
}
