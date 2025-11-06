mod lua;
mod ledger;
use std::io::Result;

#[tokio::main]
async fn main() -> Result<()> {
    let _conn = ledger::bootstrap().await.unwrap();
    let code = "return 'done'";
    match lua::run(code).await {
        Ok(_) => println!("Lua code executed successfully."),
        Err(e) => println!("Error executing Lua code: {}", e),
    }

    Ok(())
}
