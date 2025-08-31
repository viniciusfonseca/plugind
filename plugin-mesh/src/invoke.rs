use futures::future::BoxFuture;
use plugin_mesh_core::{context::{Context, InvokeResult}, LibraryFn};
use libloading::{Library, Symbol};
use serde_json::json;

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
            let lib_path = format!("{}/{}.so", ctx.libs_path, lib_name);
            let res_lib = unsafe { Library::new(&lib_path) };
            let lib = match res_lib {
                Ok(lib) => lib,
                Err(e) => return InvokeResult::Err(json!({"error": e.to_string()}).to_string().as_bytes().to_vec())
            };
            let mut libs = ctx.libs.write().await;
            libs.insert(lib_name.clone(), lib);
        }
    
        let ctx = MESH_CONTEXT.read().await;
        let libs = ctx.libs.read().await;
        let lib = libs.get(&lib_name).unwrap();
    
        let handler: Symbol<'_, LibraryFn> = unsafe {
            match lib.get(b"init") {
                Ok(f) => f,
                Err(e) => return InvokeResult::Err(json!({"error": e.to_string()}).to_string().as_bytes().to_vec())
            }
        };
        
        let input_buffer = serde_json::to_string(&input).unwrap().as_bytes().to_vec();
        let context = Context::new(|lib_name: String, input: Vec<u8>| {
            Box::pin(invoke_plugin(lib_name, input))
        });
        
        handler(input_buffer, context).await
    })
}