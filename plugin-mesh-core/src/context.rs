use futures::{future::BoxFuture, AsyncWriteExt};

#[derive(Debug)]
pub enum InvokeResult {
    Ok(Vec<u8>),
    Err(Vec<u8>)
}

impl PartialEq for InvokeResult {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (InvokeResult::Ok(a), InvokeResult::Ok(b)) => a == b,
            (InvokeResult::Err(a), InvokeResult::Err(b)) => a == b,
            _ => false
        }
    }
}

pub type InvokeFn = fn(String, Vec<u8>) -> BoxFuture<'static, InvokeResult>;

pub struct Context {
    pub invoke: InvokeFn,
    logs: Vec<u8>
}

impl Context {
    pub fn new(invoke: InvokeFn) -> Context {
        Context {
            invoke,
            logs: Vec::new()
        }
    }

    pub async fn log(&mut self, msg: &str) {
        let msg = format!("{}\n", msg);
        _ = self.logs.write(msg.as_bytes()).await;
    }

}