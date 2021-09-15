use rusqlite::{Result, Connection};

fn main()
{
    let sql = match Connection::open("SQLite.db")
    {
        Ok(sql) =>
        {
            println!("SQLite.db Opened");
            sql
        },
        Err(e) =>
        {
            println!("DB Error: {:?}", e);
            return
        }
    };

/*
    sql.execute(r#"
        CREATE TABLE example
        (
            pk INTEGER PRIMARY KEY,
            key TEXT NOT NULL,
            val TEXT DEFAULT NULL
        );
    "#, []);

    sql.execute(r#"
        INSERT INTO example (key, val) VALUES
        ('0', 'Zero'),
        ('1', 'One'),
        ('2', 'Two'),
        ('3', 'Three'),
        ('4', 'Four'),
        ('5', 'Five'),
        ('6', 'Six'),
        ('7', 'Seven'),
        ('8', 'Eight'),
        ('9', 'Nine'),
        ('10', 'Ten'),
        ('11', 'Eleven'),
        ('12', 'Twelve');
    "#, []);
*/

    let pkvalue: i64 = 1337;
    let row: Result<String, rusqlite::Error> = sql.query_row("SELECT val FROM example WHERE pk = :pkvalue;", &[(":pkvalue", &pkvalue)], |row| row.get(0),);

    println!("Value of row: {:?}", row);
}
