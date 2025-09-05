use futures::future::BoxFuture;
use plugin_mesh_core::{context::{Context, InvokeResult}, LibraryFn};
use libloading::{Library, Symbol};

use crate::context::MESH_CONTEXT;

pub fn invoke_plugin(lib_name: String, input: Vec<u8>) -> BoxFuture<'static, InvokeResult> {

    Box::pin(async move {

        let libs_contains_key_lib_name = {
            let ctx = MESH_CONTEXT.read().await;
            let libs = ctx.libs.read().await;
            libs.contains_key(&lib_name)
        };
    
        if !libs_contains_key_lib_name {
            let ctx = MESH_CONTEXT.read().await;
            let storage = ctx.storage.as_ref().unwrap();
            let lib = storage.get_object()
                .bucket("plugins")
                .key(&lib_name)
                .send()
                .await
                .unwrap();
            let lib = lib.body.collect().await.unwrap();
            let lib = lib.into_bytes();
            let id = uuid::Uuid::new_v4().to_string();
            let tmp_path = format!("/tmp/{}", id);
            tokio::fs::write(&tmp_path, lib.to_vec()).await.unwrap();
            let mut libs = ctx.libs.write().await;
            libs.insert(lib_name.clone(), unsafe { Library::new(&tmp_path)? });
            tokio::fs::remove_file(&tmp_path).await.unwrap();
        }
    
        let ctx = MESH_CONTEXT.read().await;
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