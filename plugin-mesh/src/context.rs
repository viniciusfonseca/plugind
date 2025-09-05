use std::{collections::HashMap, sync::{Arc, LazyLock}};
use libloading::Library;
use tokio::sync::RwLock;

#[derive(Clone)]
pub struct MeshContext {
    pub storage: Option<aws_sdk_s3::Client>,
    pub libs: Arc<RwLock<HashMap<String, Library>>>,
}

impl MeshContext {
    pub fn set_storage(&mut self, storage: &aws_sdk_s3::Client) {
        self.storage = Some(storage.clone());
    }
}

pub static MESH_CONTEXT: LazyLock<Arc<RwLock<MeshContext>>> = LazyLock::new(|| Arc::new(RwLock::new(MeshContext {
    storage: None,
    libs: Default::default(),
})));