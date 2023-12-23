use anyhow::Result;
use chrono::NaiveDate;
use rusqlite::Connection;

fn get_db() -> Result<Connection> {
    let db = Connection::open("test.db")?;
    db.execute(
        "CREATE TABLE IF NOT EXISTS birthdays (
             id INTEGER PRIMARY KEY,
             name TEXT NOT NULL,
             date TEXT NOT NULL
         ) STRICT",
        (),
    )?;
    Ok(db)
}

pub fn add_birthday(name: String, date: String) -> Result<()> {
    let db = get_db()?;
    let naive_date = NaiveDate::parse_from_str(&date, "%Y-%m-%d")?;
    db.execute(
        "INSERT INTO birthdays(name, date) VALUES(?1, ?2)",
        (name, naive_date),
    )?;
    Ok(())
}

#[derive(Debug)]
pub struct Birthday {
    name: String,
    date: NaiveDate,
}

pub fn get_all_birthdays() -> Result<Vec<Birthday>> {
    let db = get_db()?;
    let mut statement = db.prepare("SELECT name, date FROM birthdays")?;
    let birthday_iter = statement.query_map([], |row| {
        Ok(Birthday {
            name: row.get(0)?,
            date: row.get(1)?,
        })
    })?;
    let birthdays: Result<Vec<Birthday>, rusqlite::Error> = birthday_iter.collect();
    Ok(birthdays.unwrap())
}
