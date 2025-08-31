use pretty_sqlite::print_rows;
use rusqlite::Connection;
use sqlite_examples::db_utils::create_schema;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // -- Memory SQLite database
    let conn = Connection::open_in_memory()?;

    // -- Create schema
    create_schema(&conn)?;

    // -- Insert organization
    let mut stmt = conn.prepare("INSERT INTO org (name) VALUES (?1) RETURNING id")?;
    let org_id = stmt.query_row(("Acme, Inc.",), |row| row.get::<_, i64>(0))?;

    let names = &["Jen", "Mike", "Alice", "Bob"];
    for (idx, name) in names.iter().enumerate() {
        let org_id = if idx % 2 == 0 { Some(org_id) } else { None };
        // -- Insert person
        conn.execute(
            "INSERT INTO person (name, org_id, yob) VALUES (?1, ?2, ?3)",
            (name, &org_id, &2000),
        )?;
    }

    // -- Select join
    let query = "
        SELECT person.id, person.name, person.yob, org.name as org_name
        FROM person
        INNER JOIN org ON person.org_id = org.id
        WHERE org.id = :org_id
    ";
    let mut stmt = conn.prepare(query)?;
    let rows = stmt.query(&[(":org_id", &org_id)])?;

    print_rows(rows)?;

    Ok(())
}
