use aws_config::BehaviorVersion;
use axum::{body::Bytes, http::HeaderMap, response::IntoResponse};

use crate::{context::DAEMON_CONTEXT, invoke::invoke_plugin};

mod context;
mod invoke;
mod upload;

async fn invoke_handler(headers: HeaderMap, body: Bytes) -> impl IntoResponse {
    let plugin = match headers.get("x-plugin") {
        Some(plugin) => plugin.to_str().unwrap().to_string(),
        None => return (axum::http::StatusCode::BAD_REQUEST, "Missing 'X-Plugin' header".as_bytes().to_vec())
    };

    match invoke_plugin(plugin, body.to_vec()).await {
        Ok(output_buffer) => (axum::http::StatusCode::OK, output_buffer),
        Err(err) => (axum::http::StatusCode::INTERNAL_SERVER_ERROR, err.to_string().as_bytes().to_vec())
    }
}

#[tokio::main(flavor = "current_thread")]
async fn main() -> anyhow::Result<()> {

    let addr_listen = std::env::var("ADDR_LISTEN")?;
    let aws_s3_endpoint = std::env::var("AWS_S3_ENDPOINT")?;

    let public_key = std::env::var("PUBLIC_KEY_PATH")
        .map(|path| std::fs::read_to_string(path).expect("Failed to read public key"))
        .ok();

    let sdk_config = aws_config::load_defaults(BehaviorVersion::latest()).await;

    let config = sdk_config.to_builder()
        .endpoint_url(aws_s3_endpoint)
        .build();
    
    let client = aws_sdk_s3::Client::new(&config);

    {
        DAEMON_CONTEXT.write().await
            .set_storage(&client)
            .set_public_key(&public_key);
    }

    let router = axum::Router::new()
        .route("/invocations", axum::routing::post(invoke_handler))
        .route("/plugins", axum::routing::get(upload::plugin_list))
        .route("/plugins", axum::routing::post(upload::plugin_upload));

    let listener = tokio::net::TcpListener::bind(addr_listen).await?;

    axum::serve(listener, router).await?;

    Ok(())
}
