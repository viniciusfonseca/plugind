use axum::{extract::Multipart, http::StatusCode, response::IntoResponse};

use crate::context::MESH_CONTEXT;

pub async fn plugin_upload(mut multipart: Multipart) -> impl IntoResponse {

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
        let ctx = MESH_CONTEXT.read().await;
        let mut libs = ctx.libs.write().await;
        _ = libs.remove(&plugin_name);
    }

    let plugin_path = format!("{}/{}.so", libs_path, plugin_name);
    tokio::fs::write(plugin_path, plugin_bytes.unwrap()).await.unwrap();

    (StatusCode::OK, "")
}