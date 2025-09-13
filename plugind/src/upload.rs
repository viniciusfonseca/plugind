use axum::{extract::Multipart, http::{HeaderMap, StatusCode}, response::IntoResponse};
use futures::StreamExt;
use jwt_simple::{claims::NoCustomClaims, prelude::MACLike};
use minio::s3::types::ToStream;

use crate::context::DAEMON_CONTEXT;

pub async fn plugin_list() -> impl IntoResponse {

    let ctx = DAEMON_CONTEXT.read().await;
    let storage = ctx.storage.as_ref().unwrap();

    let list = storage.list_objects("plugins")
        .to_stream().await
        .next().await
        .unwrap()
        .unwrap();

    let list = list.contents;

    let list = list.into_iter()
        .map(|object| object.name)
        .collect::<Vec<String>>()
        .join("\n");

    (StatusCode::OK, list)
}

pub async fn plugin_upload(headers: HeaderMap, mut multipart: Multipart) -> impl IntoResponse {

    if let Some(public_key) = {
        let ctx = DAEMON_CONTEXT.read().await;
        ctx.public_key.clone()
    } {
        let token = match headers.get("authorization") {
            Some(token) => token.to_str().unwrap().to_string().replace("Bearer ", ""),
            None => return (StatusCode::UNAUTHORIZED, "")
        };

        if public_key.verify_token::<NoCustomClaims>(&token, None).is_err() {
            return (StatusCode::UNAUTHORIZED, "");
        }
    }

    let mut plugin_name = None;
    let mut plugin_bytes = None;

    while let Some(field) = multipart.next_field().await.unwrap() {

        let name = field.name().unwrap().to_string();
        let data = field.bytes().await.unwrap();

        match name.as_str() {
            "name" => plugin_name = Some(String::from_utf8_lossy(&data).to_string()),
            "file" => plugin_bytes = Some(data.to_vec()),
            _ => (),
        }
    }

    let libs_path = std::env::var("LIBS_PATH").unwrap();

    if plugin_name.is_none() || plugin_bytes.is_none() {
        return (StatusCode::BAD_REQUEST, "Missing plugin name or file");
    }
    let plugin_name = plugin_name.unwrap();

    {
        let ctx = DAEMON_CONTEXT.read().await;
        let mut libs = ctx.libs.write().await;
        _ = libs.remove(&plugin_name);
    }

    let plugin_path = format!("{}/{}.so", libs_path, plugin_name);
    tokio::fs::write(plugin_path, plugin_bytes.unwrap()).await.unwrap();

    (StatusCode::OK, "")
}