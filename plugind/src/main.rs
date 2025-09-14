use axum::{body::Bytes, extract::Path, response::IntoResponse};
use minio::s3::creds::StaticProvider;

use crate::{context::DAEMON_CONTEXT, rpc::invoke_plugin};

mod context;
mod rpc;
mod upload;

async fn rpc_handler(Path(plugin): Path<String>, body: Bytes) -> impl IntoResponse {
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

    let credentials_provider = StaticProvider::new(
        &std::env::var("AWS_ACCESS_KEY_ID")?,
        &std::env::var("AWS_SECRET_ACCESS_KEY")?,
        None,
    );

    let storage = minio::s3::Client::new(
        aws_s3_endpoint.parse()?,
        Some(Box::new(credentials_provider)),
        None,
        None
    )?;

    {
        DAEMON_CONTEXT.write().await
            .set_storage(&storage)
            .set_public_key(&public_key);
    }

    let router = axum::Router::new()
        .route("/plugins/{plugin_name}/rpc", axum::routing::post(rpc_handler))
        .route("/plugins", axum::routing::get(upload::plugin_list))
        .route("/plugins", axum::routing::post(upload::plugin_upload));

    let listener = tokio::net::TcpListener::bind(addr_listen).await?;

    axum::serve(listener, router).await?;

    Ok(())
}
