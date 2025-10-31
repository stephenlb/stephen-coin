use mlua::{Lua, Result as LuaResult};
use std::time::Duration;

async fn sleep(_lua: Lua, n: u64) -> LuaResult<&'static str> {
    tokio::time::sleep(Duration::from_millis(n)).await;
    Ok("done")
}

pub async fn run(code: &str) -> LuaResult<()> {
    let lua = Lua::new();
    lua.globals().set("sleep", lua.create_async_function(sleep)?)?;
    let _res: String = lua.load(code).call_async(100).await?;
    assert_eq!(_res, "done");
    Ok(())
}
