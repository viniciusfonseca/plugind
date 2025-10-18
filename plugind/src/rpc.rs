use axum::{body::Bytes, extract::Path, response::IntoResponse};
use futures::future::BoxFuture;
use minio::s3::types::S3Api;
use plugind_core::{context::{Context, InvokeResult}, LibraryFn};
use libloading::{Library, Symbol};

use crate::context::DAEMON_CONTEXT;

pub async fn rpc_handler(Path(plugin): Path<String>, body: Bytes) -> impl IntoResponse {
    match invoke_plugin(plugin, body.to_vec()).await {
        Ok(output_buffer) => {
            let mut headers = [httparse::EMPTY_HEADER; 128];
            let mut res = httparse::Response::new(&mut headers);
            match res.parse(&output_buffer) {
                Ok(status) => {
                    match status {
                        httparse::Status::Complete(len) => (
                            axum::http::StatusCode::from_u16(res.code.unwrap()).unwrap(),
                            output_buffer[..len].to_vec()
                        ),
                        httparse::Status::Partial => (axum::http::StatusCode::OK, output_buffer)
                    }
                },
                Err(_) => (axum::http::StatusCode::OK, output_buffer)
            }
        },
        Err(err) => (axum::http::StatusCode::INTERNAL_SERVER_ERROR, err.to_string().as_bytes().to_vec())
    }
}

pub fn invoke_plugin(lib_name: String, input: Vec<u8>) -> BoxFuture<'static, InvokeResult> {

    Box::pin(async move {

        let libs_contains_key_lib_name = {
            let ctx = DAEMON_CONTEXT.read().await;
            let libs = ctx.libs.read().await;
            libs.contains_key(&lib_name)
        };
    
        if !libs_contains_key_lib_name {
            let ctx = DAEMON_CONTEXT.read().await;
            let storage = ctx.storage.as_ref().unwrap();
            let lib = storage.get_object("plugins", &lib_name).send().await.unwrap();
            let id = uuid::Uuid::new_v4().to_string();
            let tmp_path = format!("/tmp/{}", id);
            lib.content.to_file(std::path::Path::new(&tmp_path)).await.unwrap();
            let mut libs = ctx.libs.write().await;
            libs.insert(lib_name.clone(), unsafe { Library::new(&tmp_path)? });
            tokio::fs::remove_file(&tmp_path).await.unwrap();
        }
    
        let ctx = DAEMON_CONTEXT.read().await;
        let libs = ctx.libs.read().await;
        let lib = libs.get(&lib_name).unwrap();
    
        let handler: Symbol<'_, LibraryFn> = unsafe { lib.get(b"init")? };
        
        let input_buffer = serde_json::to_string(&input)?.as_bytes().to_vec();
        let context = Context::new(|lib_name: String, input: Vec<u8>| {
            Box::pin(invoke_plugin(lib_name, input))
        });
        
        handler(input_buffer, context).await
    })
}