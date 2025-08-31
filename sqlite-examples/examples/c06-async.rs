use pretty_sqlite::print_table;
use rusqlite::Connection;
use sqlite_examples::db_utils::create_schema;

const DB_PATH: &str = "_db.db";

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // -- Memory SQLite database
    let conn = Connection::open(DB_PATH)?;

    // -- Create schema
    create_schema(&conn)?;

    let names = &["Jen", "Mike", "Alice", "Bob"];

    for name in names {
        for i in 1..10 {
            let name = format!("{name}--{i}");
            let _res = tokio::task::spawn(async move {
                let conn = Connection::open(DB_PATH).map_err(|err| err.to_string())?;
                // -- Insert person
                conn.execute(
                    "INSERT INTO person (name, yob) 
				                  VALUES (?1, ?2)",
                    (name, &2000),
                )
                .map_err(|err| err.to_string())
            })
            .await?;
        }
    }

    // -- Final print
    let conn = Connection::open(DB_PATH)?;
    print_table(&conn, "person")?;

    Ok(())
}
