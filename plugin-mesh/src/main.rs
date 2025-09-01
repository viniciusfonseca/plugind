use axum::response::IntoResponse;
use plugin_mesh_core::context::InvokeResult;
use serde::Deserialize;
use serde_json::Value;

use crate::{context::MESH_CONTEXT, invoke::invoke_plugin};

mod context;
mod invoke;

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

    let libs_path = std::env::var("LIBS_PATH")?;
    let addr_listen = std::env::var("ADDR_LISTEN")?;

    {
        MESH_CONTEXT.write().await.set_libs_path(&libs_path);
    }

    let router = axum::Router::new()
        .route("/", axum::routing::post(invoke_handler));

    let listener = tokio::net::TcpListener::bind(addr_listen).await?;

    axum::serve(listener, router).await?;

    Ok(())
}
