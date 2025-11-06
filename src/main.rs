mod lua;
mod ledger;
use std::io::Result;

#[tokio::main]
async fn main() -> Result<()> {
    let conn = match ledger::bootstrap().await {
        Ok(c) => c,
        Err(e) => {
            println!("Error bootstrapping database: {}", e);
            return Err(std::io::Error::new(std::io::ErrorKind::Other, "DB bootstrap failed"));
        }
    };
    let code = "return 'done'";

    match lua::run(code).await {
        Ok(_) => println!("Lua code executed successfully."),
        Err(e) => println!("Error executing Lua code: {}", e),
    }

    // TODO unwrap errors
    ledger::transaction(&conn, "alice", "bob", 50.0).await.unwrap();
    ledger::query(&conn, "SELECT version FROM transactions limit 2").await.unwrap();

    Ok(())
}
