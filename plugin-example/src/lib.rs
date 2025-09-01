use plugin_mesh_core::{context::{Context, InvokeResult}, plugin};

#[plugin]
pub async fn init(input: Vec<u8>, mut ctx: Context) -> InvokeResult {

    ctx.log("log -- Hello World").await;
    ctx.log(&format!("got input: {}", String::from_utf8_lossy(&input))).await;
    
    InvokeResult::Ok(b"Hello World".to_vec())
}

#[cfg(test)]
mod tests {

    use plugin_mesh_core::{context::{Context, InvokeResult}, LibraryFn};
    use futures::future::BoxFuture;

    #[test]
    fn it_works() {
        let input = b"Hello World".to_vec();
        let f: LibraryFn = super::init;
        fn invoke(_: String, _: Vec<u8>) -> BoxFuture<'static, InvokeResult> {
            unimplemented!()
        }
        let output = futures::executor::block_on(f(input.into(), Context::new(invoke)));
        assert!(output.is_ok());
        assert_eq!(output.unwrap(), b"Hello World".to_vec());
    }
}