use std::time::Duration;
use mlua::{Lua, Result};

async fn sleep(_lua: Lua, n: u64) -> Result<&'static str> {
    tokio::time::sleep(Duration::from_millis(n)).await;
    Ok("done")
}

#[tokio::main]
async fn main() -> Result<()> {
    let lua = Lua::new();
    lua.globals().set("sleep", lua.create_async_function(sleep)?)?;
    let res: String = lua.load("return sleep(5000)").call_async(100).await?; // Sleep 100ms
    assert_eq!(res, "done");

    Ok(())
}
