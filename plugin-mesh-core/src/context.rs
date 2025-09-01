use futures::{future::BoxFuture, AsyncWriteExt};

pub type InvokeResult = anyhow::Result<Vec<u8>>;

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