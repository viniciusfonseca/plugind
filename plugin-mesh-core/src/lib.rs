use futures::future::BoxFuture;

use crate::context::{Context, InvokeResult};

pub mod context;

pub type LibraryFn = fn(Vec<u8>, Context) -> BoxFuture<'static, InvokeResult>;