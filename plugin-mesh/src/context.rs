use std::{collections::HashMap, sync::{Arc, LazyLock}};
use libloading::Library;
use tokio::sync::RwLock;

#[derive(Clone)]
pub struct MeshContext {
    pub libs_path: String,
    pub libs: Arc<RwLock<HashMap<String, Library>>>,
}

impl MeshContext {
    pub fn set_libs_path(&mut self, libs_path: &String) {
        self.libs_path = libs_path.clone();
    }
}

pub static MESH_CONTEXT: LazyLock<Arc<RwLock<MeshContext>>> = LazyLock::new(|| Arc::new(RwLock::new(MeshContext {
    libs_path: String::new(),
    libs: Default::default(),
})));