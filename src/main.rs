mod lua;
use std::io::Result;
use rusqlite::{Connection, Result as SQLResult};

// Check schema is loaded and bootstrap the database
async fn bootstrap() -> SQLResult<Connection> {
    let conn = Connection::open("ledger.db")?;

    // open schema.sql text file and read it into string
    // TODO fix UNWRAP
    let schema = std::fs::read_to_string("schema.sql").unwrap();
    //conn.execute_batch(&schema)?;

    match conn.execute(&schema, ()) {
        Ok(_)  => println!("Database schema loaded successfully."),
        Err(e) => println!("Error loading database schema: {}", e),
    };

    Ok(conn)
}

#[tokio::main]
async fn main() -> Result<()> {
    let _conn = bootstrap().await.unwrap();
    let code = "return 'done'";
    match lua::run(code).await {
        Ok(_) => println!("Lua code executed successfully."),
        Err(e) => println!("Error executing Lua code: {}", e),
    }

    Ok(())
}
