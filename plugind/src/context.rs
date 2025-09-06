use std::{collections::HashMap, sync::{Arc, LazyLock}};
use libloading::Library;
use tokio::sync::RwLock;

#[derive(Clone)]
pub struct DaemonContext {
    pub storage: Option<aws_sdk_s3::Client>,
    pub libs: Arc<RwLock<HashMap<String, Library>>>,
}

impl DaemonContext {
    pub fn set_storage(&mut self, storage: &aws_sdk_s3::Client) {
        self.storage = Some(storage.clone());
    }
}

pub static DAEMON_CONTEXT: LazyLock<Arc<RwLock<DaemonContext>>> = LazyLock::new(|| Arc::new(RwLock::new(DaemonContext {
    storage: None,
    libs: Default::default(),
})));