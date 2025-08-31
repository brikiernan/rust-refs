use pretty_sqlite::{print_rows, print_table};
use rusqlite::{Connection, params};
use serde_json::json;
use sqlite_examples::db_utils::create_schema;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // -- Memory SQLite database
    let conn = Connection::open_in_memory()?;

    // -- Create schema
    create_schema(&conn)?;

    // -- Insert
    let data = &[("Jen", 94114), ("Mike", 94115)];
    let mut ids: Vec<i64> = Vec::new();
    for (name, zip) in data.iter() {
        let data_json = json!({
            "address": {
                "city": "San Francisco",
                "zip": zip
            }
        });

        let mut stmt = conn.prepare(
            "INSERT INTO person (name, yob, data_b)
                  VALUES (?1, ?2, jsonb(?3)) RETURNING id",
        )?;
        let person_id = stmt.query_row((name, &2000, data_json.to_string()), |row| row.get(0))?;
        ids.push(person_id);
    }

    let person_1_id = ids.first().ok_or("Should have at least one person ID")?;
    conn.execute(
        r#"
        UPDATE person SET data_b = 
        jsonb_set(
            data_b, 
            '$.address.zip', ?2, 
            '$.address.home_owner', json(?3)
        )
        WHERE id = ?1
    "#,
        (&person_1_id, &94222, true.to_string()),
    )?;

    // -- Select home_owner = true
    println!("== People owning homes:");
    // Note: using the `->>` notation to get the value back
    // (get sqlite type for primitive)
    // Note: If use `->` instead, the result will be a JSON value
    let mut stmt = conn.prepare(
        "SELECT id, name, yob, json(data_b) 
              FROM person 
              WHERE data_b ->> '$.address.home_owner' = :home_owner",
    )?;

    let rows = stmt.query(params![&true])?;
    // let rows = stmt.query(&[(":home_owner", &true)])?;
    print_rows(rows)?;

    // Select not home_owner
    println!("== People NOT owning homes:");
    // Note: using the jsonb_extract to get the value back
    // (get sqlite type for primitive)
    let mut stmt = conn.prepare(
        "SELECT * FROM person 
              WHERE jsonb_extract(data_b, '$.address.home_owner') IS NULL
              OR jsonb_extract(data_b, '$.address.home_owner') = 0",
    )?;

    let rows = stmt.query(())?;
    print_rows(rows)?;

    print_table(&conn, "person")?;

    Ok(())
}
