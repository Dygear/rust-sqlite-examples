use rusqlite::{Result, Connection};

#[derive(Debug)]
struct SelectedRow
{
    pub PKValue: u64
}

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

    let PKValue: u64 = 1337;
    let row: Result<SelectedRow, rusqlite::Error> = sql.query_row("SELECT * FROM table WHERE PKValue = :PKValue;", &[(":PKValue", &PKValue)], |row| row.get(0),);
}
