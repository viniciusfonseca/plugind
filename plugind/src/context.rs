use std::{collections::HashMap, sync::{Arc, LazyLock}};
use jwt_simple::prelude::HS256Key;
use libloading::Library;
use tokio::sync::RwLock;

#[derive(Clone)]
pub struct DaemonContext {
    pub storage: Option<aws_sdk_s3::Client>,
    pub public_key: Option<HS256Key>,
    pub libs: Arc<RwLock<HashMap<String, Library>>>,
}

impl DaemonContext {
    pub fn set_storage(&mut self, storage: &aws_sdk_s3::Client) -> &mut Self {
        self.storage = Some(storage.clone());
        self
    }

    pub fn set_public_key(&mut self, public_key: &Option<String>) -> &mut Self {
        self.public_key = Some(HS256Key::from_bytes(public_key.as_ref().unwrap().as_bytes()));
        self
    }
}

pub static DAEMON_CONTEXT: LazyLock<Arc<RwLock<DaemonContext>>> = LazyLock::new(|| Arc::new(RwLock::new(DaemonContext {
    storage: None,
    public_key: None,
    libs: Default::default(),
})));