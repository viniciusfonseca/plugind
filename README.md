# plugind - Rust Plugin System

plugind is a plugin system example made with Rust.

**It relies heavily on the unstable Rust ABI and must not be used on prod unless you know what you're doing.**

It has 4 crates:

### `plugind`

The system that manages the plugins. It loads plugins from an arbitrary path upon request, executes the plugin entrypoint and returns the execution result.

### `plugind-core`

Library that provides the plugin interface. Basically a function that takes a `Vec<u8>` and an execution context and returns a `BoxFuture<'static, InvokeResult>`. `InvokeResult` is an enum that takes a `Vec<u8>` both for `Ok` and `Err`.

### `plugind-macros`

Library that provides a `#[plugin]` helper macro for creating plugins. With it you can create plugins like this:

```rs
use plugind_core::{context::{Context, InvokeResult}, plugin};

#[plugin]
pub async fn init(input: Vec<u8>, mut ctx: Context) -> InvokeResult {

    ctx.log("log -- Hello World").await;
    ctx.log(&format!("got input: {}", String::from_utf8_lossy(&input))).await;
    
    InvokeResult::Ok(b"Hello World".to_vec())
}
```

### `plugin-example`

Example plugin made with `plugind-core` and `plugind-macros`.

To run the example:

```sh
make compose-up
make invoke-example
```