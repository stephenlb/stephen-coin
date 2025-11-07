use rusqlite::{Connection, Result, Error};

#[derive(Debug)]
struct Transaction {
    version: f64,
    /*difficulty: i32,
    nonce: i32,
    id: String,
    priorId: String,
    previousSignature: String,
    signature: String,
    timestamp: i64,
    fromAddress: String,
    toAddress: String,
    amount: f64,
    note: String,
    data: String,
    code: String,*/
}

// Check schema is loaded and bootstrap the database
pub async fn bootstrap() -> Result<Connection> {
    let conn = Connection::open("ledger.db")?;

    // open schema.sql text file and read it into string
    let schema = match std::fs::read_to_string("schema.sql") {
        Ok(s) => s,
        Err(e) => {
            println!("Error reading schema file: {}", e);
            return Err(Error::InvalidQuery); // or any other appropriate error
        }
    };

    // Bootstrap the database schema
    match execute(&conn, &schema).await {
        Ok(_)  => println!("Database schema loaded successfully."),
        Err(e) => println!("Error loading database schema: {}", e),
    };

    Ok(conn)
}

pub async fn execute(conn: &Connection, query: &str) -> Result<(), Error> {
    conn.execute(query, ())?;
    Ok(())
}

pub async fn query(conn: &Connection, statement: &str) -> Result<()> {
    let mut stmt = conn.prepare(statement)?;
    let iter = stmt.query_map([], |row| {
        let version = row.get(0)?;
        Ok(Transaction {
            version,
        })
    })?;

    for transaction in iter {
        println!("Found person {:?}", transaction?);
    }

    Ok(())
}

pub async fn transaction(
    conn: &Connection,
    from: &str,
    to: &str,
    amount: f64,
) -> Result<()> {
    let insert = format!(
        "INSERT INTO transactions (
            version,
            difficulty,
            nonce,
            id,
            priorId,
            previousSignature,
            signature,
            timestamp,
            fromAddress,
            toAddress,
            amount,
            note,
            data,
            code
        ) VALUES (
            1,
            1,
            1,
            'id',
            'priorId',
            'previousSignature',
            'signature',
            1762467084,
            '{}',
            '{}',
            {},
            'note',
            'data',
            'code'
        );",
        from, to, amount
    );
    match execute(conn, &insert).await {
        Ok(_)  => println!("Database schema loaded successfully."),
        Err(e) => println!("Error loading database schema: {}", e),
    };
    Ok(())
}
