use rusqlite::{Connection, Result as SQLResult};

// Check schema is loaded and bootstrap the database
pub async fn bootstrap() -> SQLResult<Connection> {
    let conn = Connection::open("ledger.db")?;

    // open schema.sql text file and read it into string
    // TODO fix UNWRAP
    let schema = std::fs::read_to_string("schema.sql").unwrap();
    //conn.execute_batch(&schema)?;

    match execute_query(&conn, &schema).await {
        Ok(_)  => println!("Database schema loaded successfully."),
        Err(e) => println!("Error loading database schema: {}", e),
    };

    Ok(conn)
}

pub async fn execute_query(conn: &Connection, query: &str) -> SQLResult<()> {
    conn.execute(query, ())?;
    Ok(())
}

/*
pub async fn _transaction(
    conn: Connection,
    from: &str,
    to: &str,
    amount: f64,
) -> SQLResult<Connection> {
    //let result = ledger::bootstrap().await.unwrap();

}*/
